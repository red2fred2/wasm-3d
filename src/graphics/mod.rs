mod shaders;
mod gl;

use std::collections::HashMap;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use self::shaders::shader_sources::get_shader_sources;

pub struct Graphics {
	context: WebGlRenderingContext,
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
		Graphics {context: context, shaders: shaders}
	}

	/// Renders a frame to the screen
	pub fn render(&self) {
		let gl = &self.context;

		let vertices: [f32; 3*9] = [
			-1.,  1., 0.,
			 0.,  1., 0.,
			 1.,  1., 0.,
			-1.,  0., 0.,
			 0.,  0., 0.,
			 1.,  0., 0.,
			-1., -1., 0.,
			 0., -1., 0.,
			 1., -1., 0.
		];
		let mut indices: [u8; 12] = [
			0, 1, 4,
			2, 4, 5,
			4, 7, 8,
			3, 4, 6
		];

		let vertices_location = vertices.as_ptr() as u32 / 4;

		let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>().unwrap()
            .buffer();

		// Set shader
		let shader = self.shaders.get("Basic bitch").unwrap();
		gl.use_program(shader.as_ref());

		// Initialize the array buffer
		let array_buffer = gl.create_buffer();
		gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

		let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

		gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vert_array, WebGlRenderingContext::STATIC_DRAW);

		let index_buffer = gl.create_buffer().unwrap();
		gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        gl.buffer_data_with_u8_array(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, &mut indices[..], WebGlRenderingContext::STATIC_DRAW);

		gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
		gl.enable_vertex_attrib_array(0);

		gl.clear_color(0.0, 0.0, 0.0, 1.0);
		gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
		gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, indices.len() as i32, WebGlRenderingContext::UNSIGNED_BYTE, 0);
	}

}
