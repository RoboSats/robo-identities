use crate::error::Error;

pub mod error;
mod hash;
mod image;
mod materials;

const SET_DEFAULT: &str = "set1";

pub struct RoboHashBuilder<'a> {
    text: &'a str,
    color: Option<String>,
    image_size: ImageSize,
    set: String,
    set_root: String,
    background_set: Option<String>,
    background_root: String,
}

impl<'a> RoboHashBuilder<'a> {
    pub fn new(text: &'a str) -> Self {
        let color = None;
        let image_size = ImageSize::default();
        let set = String::from(SET_DEFAULT);
        let set_root = String::from("./sets");
        let background_set = None;
        let background_root = String::from("./backgrounds");
        Self {
            text,
            color,
            image_size,
            set,
            set_root,
            background_set,
            background_root,
        }
    }

    pub fn with_set(mut self, set: &str) -> RoboHashBuilder<'a> {
        self.set = String::from(set);
        self
    }

    pub fn with_set_location(mut self, set_location: &str) -> RoboHashBuilder<'a> {
        self.set_root = String::from(set_location);
        self
    }

    pub fn with_background_set(mut self, background_set: &str) -> RoboHashBuilder<'a> {
        self.background_set = Some(String::from(background_set));
        self
    }

    pub fn with_background_location(mut self, background_location: &str) -> RoboHashBuilder<'a> {
        self.background_root = String::from(background_location);
        self
    }

    pub fn with_color(mut self, color: &str) -> RoboHashBuilder<'a> {
        self.color = Some(String::from(color));
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
        let color = color_selection(&hash_array, &self.color, &self.set, &self.set_root)?;
        let set = self.set_with_color(color);
        let sets_root = self.set_root.to_owned();
        let background_set = self.background_set.to_owned();
        let background_root = self.background_root.to_owned();

        Ok(RoboHash {
            image_size: self.image_size,
            hash_array,
            set,
            sets_root,
            background_set,
            background_root,
        })
    }

    fn set_with_color(&self, color: Option<String>) -> String {
        match self.set.as_str() {
            SET_DEFAULT => match color {
                Some(color) => format!("{}/{}", &self.set.as_str(), color.as_str()),
                None => String::from(self.set.as_str()),
            },
            _ => String::from(self.set.as_str()),
        }
    }
}

#[derive(Debug)]
pub struct RoboHash {
    image_size: ImageSize,
    hash_array: Vec<i64>,
    set: String,
    sets_root: String,
    background_set: Option<String>,
    background_root: String,
}

#[derive(Debug, Clone, Copy)]
struct ImageSize {
    width: u32,
    height: u32,
}

impl ImageSize {
    pub(crate) fn default() -> Self {
        Self {
            width: 1024,
            height: 1024,
        }
    }
}

impl RoboHash {
    pub fn assemble_base64(&self) -> Result<String, Error> {
        if self.is_missing_required_data() {
            return Err(Error::RoboHashMissingRequiredData);
        }

        let set = files_in_set(&self.hash_array, &self.sets_root, &self.set)?;
        let background = match &self.background_set {
            Some(set) => background(&self.hash_array, &self.background_root, set)?,
            None => None,
        };

        let image = image::build_robo_hash_image(
            &set,
            &background,
            self.image_size.width,
            self.image_size.height,
        )?;
        let base64 = image::to_base_64(&image)?;
        Ok(base64)
    }

    fn is_missing_required_data(&self) -> bool {
        self.hash_array.is_empty() || self.set.is_empty() || self.sets_root.is_empty()
    }
}

fn files_in_set(hash_array: &Vec<i64>, sets_root: &str, set: &str) -> Result<Vec<String>, Error> {
    let categories_in_set = materials::categories_in_set(sets_root, set)?;
    let mut index = 4;
    let mut files = categories_in_set
        .iter()
        .flat_map(
            |category| match materials::files_in_category(sets_root, set, category) {
                Ok(file) => {
                    let set_index = (hash_array[index] % file.len() as i64) as usize;
                    if let Some(selected_file) = file.get(set_index) {
                        index = index + 1;
                        Some(String::from(selected_file))
                    } else {
                        println!("failed to fetch index {set_index:#?} from {file:#?}");
                        None
                    }
                }
                Err(e) => {
                    println!("{e:#?}");
                    None
                }
            },
        )
        .collect::<Vec<String>>();
    files.sort_by(|a, b| {
        a.split("#").collect::<Vec<_>>()[1].cmp(b.split("#").collect::<Vec<_>>()[1])
    });
    Ok(files)
}

fn background(
    hash_array: &Vec<i64>,
    background_root: &str,
    set: &str,
) -> Result<Option<String>, Error> {
    let index = 3;
    let backgrounds = materials::categories_in_set(background_root, set)?;
    let set_index = (hash_array[index] % backgrounds.len() as i64) as usize;
    Ok(match backgrounds.get(set_index) {
        Some(background) => {
            let background_path = [background_root, "/", set, "/", background].concat();
            Some(background_path)
        }
        None => {
            println!("failed to fetch index {set_index:#?} from {backgrounds:#?}");
            None
        }
    })
}

fn color_selection(
    hash_array: &Vec<i64>,
    color: &Option<String>,
    set: &str,
    set_root: &str,
) -> Result<Option<String>, Error> {
    if set == SET_DEFAULT && color.is_none() {
        Ok(Some(random_color(hash_array, set_root)?))
    } else {
        Ok(color.clone())
    }
}

fn random_color(hash_array: &Vec<i64>, set_root: &str) -> Result<String, Error> {
    let available_colors = materials::categories_in_set(set_root, "set1")?;
    let selected_index = (hash_array[0] % available_colors.len() as i64) as usize;
    Ok(available_colors[selected_index].clone())
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
    fn test_that_robo_hash_builder_returns_a_builder_with_default_set() {
        // arrange
        let text = "text";
        let expected_set = SET_DEFAULT;
        // act
        let robo_hash_builder = RoboHashBuilder::new(text);
        // assert
        assert_eq!(robo_hash_builder.set, expected_set)
    }

    #[test]
    fn test_that_robo_hash_builder_returns_a_builder_with_color_set_to_any() {
        // arrange
        let text = "text";
        let expected_color = None;
        // act
        let robo_hash_builder = RoboHashBuilder::new(text);
        // assert
        assert_eq!(robo_hash_builder.color, expected_color)
    }

    #[test]
    fn test_that_robo_hash_builder_with_set_changes_the_set() {
        // arrange
        let text = "text";
        let set = "set1";
        let expected_set = "set1";
        // act
        let robo_hash_builder = RoboHashBuilder::new(text).with_set(set);
        // assert
        assert_eq!(robo_hash_builder.set, expected_set)
    }

    #[test]
    fn test_that_robo_hash_builder_with_color_changes_sets_color() {
        // arrange
        let text = "text";
        let color = "blue";
        let expected_color = Some(String::from("blue"));
        // act
        let robo_hash_builder = RoboHashBuilder::new(text).with_color(color);
        // assert
        assert_eq!(robo_hash_builder.color, expected_color)
    }

    #[test]
    fn test_that_robo_hash_builder_with_set_root_changes_sets_new_set_root() {
        // arrange
        let text = "text";
        let set_root = "new_set_root";
        let expected_set_root = "new_set_root";
        // act
        let robo_hash_builder = RoboHashBuilder::new(text).with_set_location(set_root);
        // assert
        assert_eq!(robo_hash_builder.set_root, expected_set_root)
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
            width: 1024,
            height: 1024,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![],
            set: String::from("set1"),
            sets_root: String::from("set_root"),
            background_set: None,
            background_root: String::from("background_root"),
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
    fn test_robo_hash_assemble_base64_returns_missing_data_error_when_set_does_not_contain_any_data(
    ) {
        // arrange
        let image_size = ImageSize {
            width: 1024,
            height: 1024,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![1, 2],
            set: String::from(""),
            sets_root: String::from("set_root"),
            background_set: None,
            background_root: String::from("background_root"),
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
    fn test_robo_hash_assemble_base64_returns_missing_data_error_when_sets_root_does_not_contain_any_data(
    ) {
        // arrange
        let image_size = ImageSize {
            width: 1024,
            height: 1024,
        };
        let robo_hash = RoboHash {
            image_size,
            hash_array: vec![1, 2],
            set: String::from("set1"),
            sets_root: String::from(""),
            background_set: None,
            background_root: String::from("background_root"),
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
        let set = SET_DEFAULT;
        let color: Option<String> = None;
        let background_set = "bg1";

        let test_resource = format!("{initial_string}_{set}_{color:#?}_{background_set}");
        let expected_robo_hash = load_base64_string_image_resources(&test_resource);

        // act
        let robo_hash = RoboHashBuilder::new(initial_string)
            .with_set(set)
            .with_background_set(background_set)
            .build()
            .unwrap();
        let constructed_robo_hash = robo_hash.assemble_base64().unwrap();

        assert_eq!(constructed_robo_hash, expected_robo_hash);
    }

    fn _write_to_test_resources(location: &str, content: &str) -> std::io::Result<()> {
        let file_location = format!("./test_resources/{}.txt", location);
        let mut file = File::create(file_location)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
