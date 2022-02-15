use wasm_bindgen::prelude::*;

pub mod graphics;

/// Initialize rust
///
/// Function called from the browser to initialize the rust program
#[wasm_bindgen]
pub fn init_rs() {
	let graphics = crate::graphics::Graphics::init();
	graphics.render();
}
