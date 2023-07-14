use std::io::Cursor;

use image::{imageops, DynamicImage, ImageBuffer, Rgba, RgbaImage};

use base64::{engine::general_purpose, Engine as _};

use crate::error::Error;

pub(crate) fn build_robo_hash_image(
    robo_parts: &[String],
    background: &Option<String>,
    width: u32,
    height: u32,
) -> Result<RgbaImage, Error> {
    let mut base_image = image::ImageBuffer::new(width, height);
    if let Some(background) = background {
        append_to_image(&mut base_image, background, width, height)?;
    }
    robo_parts
        .iter()
        .try_for_each(|image_path| -> Result<(), Error> {
            append_to_image(&mut base_image, image_path, width, height)?;
            Ok(())
        })?;
    Ok(base_image)
}

fn append_to_image(
    base_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    image_path: &String,
    width: u32,
    height: u32,
) -> Result<(), Error> {
    let image = try_open_image(image_path)?;
    let image = imageops::resize(&image, width, height, imageops::FilterType::Lanczos3);
    imageops::overlay(base_image, &image, 0, 0);
    Ok(())
}

fn try_open_image(image_path: &String) -> Result<DynamicImage, Error> {
    match image::open(image_path) {
        Ok(image) => Ok(image),
        Err(e) => Err(Error::ImageOpenFailed(format!("{e:#?}"))),
    }
}

pub(crate) fn to_base_64(image: &RgbaImage) -> Result<String, Error> {
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)?;
    Ok(general_purpose::STANDARD.encode(&bytes))
}

fn from_base64(base64_string: &str) -> Result<DynamicImage, Error> {
    let decoded_bytes = general_purpose::STANDARD
        .decode(base64_string)
        .expect("Hardcoded base_64 strings should be decodable");
    let cursor = Cursor::new(decoded_bytes);
    let image = image::load(cursor, image::ImageFormat::WebP)?;
    Ok(image)
}

#[cfg(test)]
pub(crate) mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn build_robo_hash_image_returns_built_image_of_parts() {
        // arrange
        let robo_parts = vec![
            String::from("./sets/set1/blue/003#01Body/000#blue_body-10.png"),
            String::from("./sets/set1/blue/004#02Face/000#blue_face-07.png"),
            String::from("./sets/set1/blue/000#03Mouth/000#blue_mouth-10.png"),
            String::from("./sets/set1/blue/001#04Eyes/000#blue_eyes-07.png"),
            String::from("./sets/set1/blue/002#05Accessory/000#blue_accessory-02.png"),
        ];
        // act
        let robo_hash = build_robo_hash_image(&robo_parts, &None, 512, 512);
        // assert
        assert!(robo_hash.is_ok())
    }

    #[test]
    fn to_base64_converts_image_to_base64_string() {
        // arrange
        let robo_parts = vec![
            String::from("./sets/set1/blue/003#01Body/000#blue_body-10.png"),
            String::from("./sets/set1/blue/004#02Face/000#blue_face-07.png"),
            String::from("./sets/set1/blue/000#03Mouth/000#blue_mouth-10.png"),
            String::from("./sets/set1/blue/001#04Eyes/000#blue_eyes-07.png"),
            String::from("./sets/set1/blue/002#05Accessory/000#blue_accessory-02.png"),
        ];
        let background = Some(String::from("./backgrounds/bg1/000#robotBG-11.png"));
        let expected_base64 = load_base64_string_image_resources("image");
        let robo_hash = build_robo_hash_image(&robo_parts, &background, 512, 512)
            .expect("Should return an actual ImageBuffer");
        // act
        let base64_string = to_base_64(&robo_hash);
        // assert
        assert!(base64_string.is_ok());

        // // Save output
        // use std::io::Write;
        // let mut output = File::create("./robohash.txt").unwrap();
        // write!(output, "{}", base64_string.unwrap());
        assert_eq!(base64_string.unwrap(), expected_base64)
    }

    pub(crate) fn load_base64_string_image_resources(filename: &str) -> String {
        let mut file_contents = String::new();
        let file_location = format!("./test_resources/{}.txt", filename);
        let mut file = File::open(file_location).unwrap();
        file.read_to_string(&mut file_contents).unwrap();
        file_contents
    }
}
