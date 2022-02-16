use logic::world::World;
use wasm_bindgen::prelude::*;

pub mod graphics;
pub mod logic;

/// Initialize rust
///
/// Function called from the browser to initialize the rust program
#[wasm_bindgen]
pub fn init_rs() {
	// Initialize the world and graphics systems
	let world = World::init();
	let graphics = crate::graphics::Graphics::init();

	// Render on test frame
	graphics.render(&world);
}
