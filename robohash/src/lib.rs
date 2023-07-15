use crate::error::Error;

mod backgrounds;
pub mod error;
mod hash;
mod image;
mod robot_parts;

pub struct RoboHashBuilder<'a> {
    text: &'a str,
    image_size: ImageSize,
    use_background: &'a bool,
}

impl<'a> RoboHashBuilder<'a> {
    pub fn new(text: &'a str) -> Self {
        let image_size = ImageSize::default();
        let use_background = &true;
        Self {
            text,
            image_size,
            use_background,
        }
    }

    pub fn with_background(mut self, use_background: &'a bool) -> RoboHashBuilder<'a> {
        self.use_background = use_background;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> RoboHashBuilder<'a> {
        self.image_size = ImageSize { width, height };
        self
    }

    pub fn build(&self) -> Result<RoboHash, Error> {
        let hash_array_chunks = 11;
        let hash = hash::sha512_digest(self.text)?;
        let hash_array = hash::split_hash(&hash, hash_array_chunks)?;
        let use_background = self.use_background.to_owned();

        Ok(RoboHash {
            image_size: self.image_size,
            hash_array,
            use_background,
        })
    }
}

#[derive(Debug)]
pub struct RoboHash {
    image_size: ImageSize,
    hash_array: Vec<i64>,
    use_background: bool,
}

#[derive(Debug, Clone, Copy)]
struct ImageSize {
    width: u32,
    height: u32,
}

impl ImageSize {
    pub(crate) fn default() -> Self {
        Self {
            width: 256,
            height: 256,
        }
    }
}

impl RoboHash {
    pub fn assemble_base64(&self) -> Result<String, Error> {
        if self.is_missing_required_data() {
            return Err(Error::RoboHashMissingRequiredData);
        }

        let set = select_robot_parts(&self.hash_array);

        let background = match &self.use_background {
            true => select_background(&self.hash_array),
            false => None,
        };

        let hue_rotation = select_hue_rotation(&self.hash_array);

        let image = image::build_robo_hash_image(
            &set,
            &background,
            self.image_size.width,
            self.image_size.height,
            &hue_rotation,
        )?;

        let base64 = image::to_base_64(&image)?;
        Ok(base64)
    }

    fn is_missing_required_data(&self) -> bool {
        self.hash_array.is_empty()
    }
}

fn select_robot_parts(hash_array: &[i64]) -> Vec<String> {
    use robot_parts::{PARTS, PARTS_LENGTH};
    let mut selected_strings = Vec::new();

    for i in 0..PARTS.len() {
        let index = (hash_array[i] % PARTS_LENGTH[i] as i64) as usize;
        selected_strings.push(PARTS[i][index].to_string())
    }

    selected_strings
}

fn select_background(hash_array: &[i64]) -> Option<String> {
    use backgrounds::BACKGROUNDS;
    let index = 6;
    let i = (hash_array[index] % BACKGROUNDS.len() as i64) as usize;
    Some(BACKGROUNDS[i].to_string())
}

fn select_hue_rotation(hash_array: &[i64]) -> Option<i32> {
    let index = 7;
    let hue = (hash_array[index] % 360) as i32;
    Some(hue)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use crate::image::tests::load_base64_string_image_resources;

    use super::*;

    #[test]
    fn test_that_robo_hash_builder_returns_a_builder() {
        // arrange
        let text = "text";
        // act
        let robo_hash_builder = RoboHashBuilder::new(text);
        // assert
        assert_eq!(robo_hash_builder.text, text)
    }

    #[test]
    fn test_that_robo_hash_builder_build_returns_a_robo_hash_struct() {
        // arrange
        let text = "text";
        let expected_hash_array = vec![
            16145521472556,
            12696294247384,
            5154811788184,
            10555455865428,
            2642153577670,
            16342997499342,
            10550500569788,
            8328031981449,
            14915230302908,
            14678679777589,
            12705535333312,
            16145521472556,
            12696294247384,
            5154811788184,
            10555455865428,
            2642153577670,
            16342997499342,
            10550500569788,
            8328031981449,
            14915230302908,
            14678679777589,
            12705535333312,
        ];
        // act
        let robo_hash = RoboHashBuilder::new(text).build();
        // assert
        assert!(robo_hash.is_ok());
        assert_eq!(robo_hash.unwrap().hash_array, expected_hash_array)
    }

    #[test]
    fn test_robo_hash_assemble_base64_returns_missing_data_error_when_robo_hash_does_not_contain_hash_array(
    ) {
        // arrange
        let image_size = ImageSize {
            width: 512,
            height: 512,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![],
            use_background: false,
        };
        // act
        let image = robo_hash.assemble_base64();
        // assert
        assert!(image.is_err());
        assert_eq!(
            image.err().unwrap().to_string(),
            Error::RoboHashMissingRequiredData.to_string()
        )
    }

    #[test]
    fn test_that_robo_hash_image_is_generated() {
        // arrange
        let initial_string = "test";

        let test_resource = initial_string;
        let expected_robo_hash = load_base64_string_image_resources(test_resource);

        // act
        let robo_hash = RoboHashBuilder::new(initial_string).build().unwrap();
        let constructed_robo_hash = robo_hash.assemble_base64().unwrap();

        // _write_to_test_resources(&test_resource, &constructed_robo_hash);

        assert_eq!(constructed_robo_hash, expected_robo_hash);
    }

    fn _write_to_test_resources(location: &str, content: &str) -> std::io::Result<()> {
        let file_location = format!("./test_resources/{}.txt", location);
        let mut file = File::create(file_location)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
