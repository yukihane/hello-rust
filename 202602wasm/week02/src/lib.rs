use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> Result<String, JsValue> {
    if name.is_empty() {
        Err(JsValue::from_str("Name cannot be empty"))
    } else {
        Ok(format!("Hello, {}!", name))
    }
}

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}
