use wasm_bindgen::prelude::*;

#[wasm_bindgen] // export to js
pub fn greet(name: &str) {
    println!("Hi there {}", name);
}
