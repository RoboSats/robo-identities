mod utils;

use robohash::RoboHashBuilder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_robohash(_hash: &str) -> String {
    let initial_string = "reckless";
    let size = 256;

    // build
    let robo_hash = RoboHashBuilder::new(initial_string)
        .with_background(&true)
        .with_size(size, size)
        .build()
        .unwrap();

    robo_hash.assemble_base64().unwrap()
}
