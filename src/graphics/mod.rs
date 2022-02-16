mod gl;
mod shaders;

use std::collections::HashMap;

use web_sys::{WebGlRenderingContext, WebGlProgram};

use crate::logic::world::World;
use self::shaders::shader_sources::get_shader_sources;

pub struct Graphics {
	context: WebGlRenderingContext,
	shaders: HashMap<&'static str, Option<WebGlProgram>>
}

/// Holds all information regarding the graphics of the application
impl Graphics {

	/// Initialize graphics
	pub fn init() -> Graphics {
		// Set up the front end
		let context = gl::set_up_canvas();

		// Compile shaders
		let shader_source = get_shader_sources();
		let mut shaders = HashMap::new();

		for (name, source) in shader_source.iter() {
			let compiled_shader = gl::build_program(&context, source);
			shaders.insert(name as &str, compiled_shader);
		}

		// Return newly created Graphics object
		Graphics {context: context, shaders: shaders}
	}

	/// Renders a frame to the screen
	pub fn render(&self, world: &World) {
		// Set gl to the rendering context for easier use
		let gl = &self.context;

		// Clear the screen for rendering
		gl.clear_color(0.0, 0.0, 0.0, 1.0);
		gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

		// Render object by object
		let objects = world.get_objects();
		for object in objects {
			// Get shader to use
			let shader_name = object.get_shader_name();
			let shader = self.shaders.get(shader_name).unwrap();

			object.render(gl, shader);
		}
	}
}
