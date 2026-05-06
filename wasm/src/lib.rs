use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_mocking_text(text: &str) -> String {
    text.split("")
        .map(|char| {
            if rand::random_bool(1.0 / 3.0) {
                char.to_uppercase()
            } else {
                char.to_lowercase()
            }
        })
        .collect()
}
