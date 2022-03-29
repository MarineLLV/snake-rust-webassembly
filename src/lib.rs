use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen] // export to js
pub fn greet(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(string: &str);
}

// wasm-pack build --target web