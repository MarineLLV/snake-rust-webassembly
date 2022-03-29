use wasm_bindgen::prelude::*;

#[wasm_bindgen] // export to js
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(string: &str);
}

// wasm-pack build --target web