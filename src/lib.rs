use graphics::Graphics;
use logic::world::World;
use wasm_bindgen::prelude::*;

pub mod graphics;
pub mod logic;

/// Contain all the information for the web app
#[wasm_bindgen]
pub struct WebApp {
	graphics: Graphics,
	world: World
}

#[wasm_bindgen]
impl WebApp {
	/// Initialize rust
	///
	/// Function called from the browser to initialize the rust program
	#[wasm_bindgen]
	pub fn init() -> WebApp {
		let world = World::init();
		let graphics = Graphics::init();
		WebApp { graphics: graphics, world: world }
	}

	/// Kicks off rendering
	#[wasm_bindgen]
	pub fn render(&self) {
		self.graphics.render(&self.world);
	}

	/// Kicks off world update
	#[wasm_bindgen]
	pub fn update(&self, dt: f32) {
		self.world.update(dt);
	}

}
