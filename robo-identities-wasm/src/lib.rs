// mod utils;

use js_sys::Promise;
use robohash::RoboHashBuilder;
use robonames::generate_short_nickname;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

async fn string_to_js_result(string: String) -> Result<JsValue, JsValue> {
    // Turns a string into a future jsvalue result
    Ok(JsValue::from_str(&string))
}
#[wasm_bindgen]
pub fn generate_robohash(initial_string: &str, size: i32) -> String {
    // Generate Robot Avatar synchronously. Returns a base64 avatar string.
    let robohash = RoboHashBuilder::new(initial_string)
        .with_background(&true)
        .with_size(size as u32, size as u32)
        .build();

    match robohash {
        Ok(robo) => match robo.assemble_base64() {
            Ok(base64_string) => base64_string,
            Err(text) => text.to_string(),
        },
        Err(text) => text.to_string(),
    }
}

#[wasm_bindgen]
pub fn async_generate_robohash(initial_string: &str, size: i32) -> Promise {
    // Generate Robot Avatar asynchronously. Returns a base64 avatar string promise.
    let future = string_to_js_result(generate_robohash(initial_string, size));
    future_to_promise(future)
}

#[wasm_bindgen]
pub fn generate_roboname(initial_string: &str) -> String {
    // Generate Robot Nickname synchronousl. Returns a nickname string.
    let nickname = generate_short_nickname(initial_string);
    match nickname {
        Ok(nick) => nick,
        Err(_) => "Error".to_string(),
    }
}

#[wasm_bindgen]
pub fn async_generate_roboname(initial_string: &str) -> Promise {
    // Generate Robot Nickname asynchronously. Returns a nickname string promise.
    let future = string_to_js_result(generate_roboname(initial_string));
    future_to_promise(future)
}

// Print browser alerts, useful for testing.
// #[wasm_bindgen]
// extern "C" {
//     pub fn alert(s: &str);
// }
