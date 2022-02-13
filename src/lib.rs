use wasm_bindgen::prelude::*;

pub mod graphics;

/**
 * Init
 * Function called from the browser to initialize the rust program
 */
#[wasm_bindgen]
pub fn init_rs() {
	crate::graphics::graphics::init();
}
