mod shaders;
mod gl;

use std::collections::HashMap;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use self::shaders::shader_sources::get_shader_sources;

pub struct Graphics {
	webgl: WebGlRenderingContext,
	shaders: HashMap<&'static str, Option<WebGlProgram>>
}

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
		Graphics { webgl: context, shaders: shaders }
	}

	/// Renders a frame to the screen
	pub fn render(&self) {
		// Set shader
		let shader = self.shaders.get("Basic bitch").unwrap();
		self.webgl.use_program(shader.as_ref());

		// Initialize the array buffer
		let array_buffer = self.webgl.create_buffer();
		self.webgl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

		// Set up the shared memory buffer between rust and the web
		unsafe {
			self.webgl.buffer_data_with_array_buffer_view(
				WebGlRenderingContext::ARRAY_BUFFER,
				&js_sys::Float32Array::view(&vec![
					1.0, 1.0, 0.0,
					0.0, 1.0, 0.0,
					0.0, 0.0, 0.0,

					0.0, 0.0, 0.0,
					1.0, 1.0, 0.0,
					1.0, 0.0, 0.0
				]),
				WebGlRenderingContext::STATIC_DRAW,
			);
		}

		self.webgl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
		self.webgl.enable_vertex_attrib_array(0);

		// Clear the screen then draw
		self.webgl.clear_color(0.0, 0.0, 0.0, 1.0);
		self.webgl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
		self.webgl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
	}

}
