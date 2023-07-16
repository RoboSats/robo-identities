mod utils;

use robohash::RoboHashBuilder;
use robonames::generate_nickname;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn generate_robohash(initial_string: &str) -> String {
    let size = 256;

    // build
    let robohash = RoboHashBuilder::new(initial_string)
        .with_background(&true)
        .with_size(size, size)
        .build()
        .expect("Should build RoboHash generator");

    match robohash.assemble_base64() {
        Ok(base64_string) => base64_string,
        Err(_) => "".to_string(),
    }
}

#[wasm_bindgen]
pub fn generate_roboname(initial_string: &str) -> String {
    generate_nickname(initial_string).to_string()
}
