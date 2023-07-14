use robohash::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    let initial_string = "test";
    let set = "set1";
    let color = String::from("red");
    let background_set = "bg1";
    let size = 256;

    // build
    let robo_hash: RoboHash = RoboHashBuilder::new(initial_string)
        .with_set(set)
        .with_color(&color)
        .with_background_set(background_set)
        .with_size(size, size)
        .build()
        .unwrap();

    let base64_robohash = robo_hash.assemble_base64()?;

    // Save output
    let mut output = File::create("robohash.txt")?;
    write!(output, "{}", base64_robohash)?;

    Ok(())
}
