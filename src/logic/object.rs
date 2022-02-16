use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram};

use super::vertex::Vertex;

/// Something that can be rendered to the screen
pub struct Object {
	/// The name of the shader to use on this object
	shader_name: &'static str,
	/// Contains an index array for rendering
	triangle_indices: Vec<u8>,
	/// A vector of vertices optimized for rendering vs physics
	///
	/// Stored like [x1, y2, z1, x2, y2, z2]
	vertices: Vec<f32>
}

impl Object {
	/// Get the name of the shader to use when rendering this object
	pub fn get_shader_name(&self) -> &'static str {
		self.shader_name
	}

	pub fn get_vertices(&self) -> Vec<Vertex> {
		let mut vertices = Vec::new();

		for i in 0..self.vertices.len()/3 {

		vertices.push(Vertex {
			x: self.vertices[3*i],
			y: self.vertices[3*i+1],
			z: self.vertices[3*i+2]
		});
		}

		vertices
	}

	/// Creates a new Object
	///
	/// * `shader_name` - The name of the shader to use on this object
	/// * `triangle_indices` - Contains an index array for rendering
	/// * `vertices` - A vector of vertices optimized for rendering vs physics
	///
	/// Stored like [x1, y2, z1, x2, y2, z2]
	pub fn new(
		shader_name: &'static str,
		triangle_indices: Vec<u8>,
		vertices: Vec<f32>
	) -> Object {
		Object {
			shader_name: shader_name,
			triangle_indices: triangle_indices,
			vertices: vertices
		}
	}

	/// Render this object
	///
	/// * `gl` - the rendering context to use
	pub fn render(&self, gl: &WebGlRenderingContext, shader: Option<&WebGlProgram>) {
		// Allocate memory
		let memory = wasm_bindgen::memory()
		.dyn_into::<WebAssembly::Memory>();

		let memory_buffer = match memory {
			Ok(mem) => mem.buffer(),
			_ =>
			if cfg!(debug_assertions) {
				panic!("Failed to allocate memory to render an Object");
			} else {
				return
			}
		};

		// Set shader
		gl.use_program(shader);

		// Initialize the array buffer
		let array_buffer = gl.create_buffer();
		gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

		let vertices_location = self.vertices.as_ptr() as u32 / 4;
		let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + self.vertices.len() as u32);

		gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vert_array, WebGlRenderingContext::STATIC_DRAW);

		let index_buffer = gl.create_buffer().unwrap();
		gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

		gl.buffer_data_with_u8_array(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, &self.triangle_indices[..], WebGlRenderingContext::STATIC_DRAW);

		gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
		gl.enable_vertex_attrib_array(0);

		gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, self.triangle_indices.len() as i32, WebGlRenderingContext::UNSIGNED_BYTE, 0);
	}
}
