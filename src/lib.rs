use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::decode;
use image::load_from_memory;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) {
    log(&"Grayscale Called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image Loaded".into())
} 