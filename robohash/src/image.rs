use std::io::Cursor;

use image::{imageops, DynamicImage, ImageBuffer, Rgba, RgbaImage};

use base64::{engine::general_purpose, Engine as _};

use crate::error::Error;

pub(crate) fn build_robo_hash_image(
    robo_parts: &[String],
    background: &Option<String>,
    width: u32,
    height: u32,
    hue_rotation: &Option<i32>,
) -> Result<RgbaImage, Error> {
    let mut base_image = image::ImageBuffer::new(width, height);
    if let Some(background) = background {
        append_to_image(&mut base_image, background, width, height, &0)?;
    }

    let hue = match hue_rotation {
        Some(hue) => hue,
        None => &0,
    };

    robo_parts
        .iter()
        .try_for_each(|image_path| -> Result<(), Error> {
            append_to_image(&mut base_image, image_path, width, height, hue)?;
            Ok(())
        })?;
    Ok(base_image)
}

fn append_to_image(
    base_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    image_path: &str,
    width: u32,
    height: u32,
    hue_rotation: &i32,
) -> Result<(), Error> {
    // let image = try_open_image(image_path)?;
    let image = from_base64(image_path)?;
    let mut image = imageops::resize(&image, width, height, imageops::FilterType::Lanczos3);
    imageops::colorops::huerotate_in_place(&mut image, *hue_rotation);
    imageops::overlay(base_image, &image, 0, 0);
    Ok(())
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

    use crate::backgrounds::BACKGROUNDS;
    use crate::robot_parts::PARTS;

    use super::*;

    #[test]
    fn build_robo_hash_image_returns_built_image_of_parts() {
        // arrange
        let robo_parts = vec![
            String::from(PARTS[0][0]),
            String::from(PARTS[1][0]),
            String::from(PARTS[2][0]),
            String::from(PARTS[3][0]),
            String::from(PARTS[4][0]),
        ];
        let hue_rotation = None;
        // act
        let robo_hash = build_robo_hash_image(&robo_parts, &None, 512, 512, &hue_rotation);
        // assert
        assert!(robo_hash.is_ok())
    }

    #[test]
    fn to_base64_converts_image_to_base64_string() {
        // arrange
        let robo_parts = vec![
            String::from(PARTS[0][0]),
            String::from(PARTS[1][0]),
            String::from(PARTS[2][0]),
            String::from(PARTS[3][0]),
            String::from(PARTS[4][0]),
        ];
        let hue_rotation = Some(90);
        let background = Some(String::from(BACKGROUNDS[0]));
        let expected_base64 = load_base64_string_image_resources("image");
        let robo_hash = build_robo_hash_image(&robo_parts, &background, 512, 512, &hue_rotation)
            .expect("Should return an actual ImageBuffer");
        // act
        let base64_string = to_base_64(&robo_hash);
        // assert
        assert!(base64_string.is_ok());

        // // Overwrite test image
        // use std::io::Write;
        // let mut output = File::create("./test_resources/image.txt").unwrap();
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
