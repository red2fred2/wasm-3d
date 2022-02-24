use js_sys::WebAssembly;
use nalgebra::{Matrix4, Vector3};
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram};

/// Something that can be rendered to the screen
pub struct Object {
	/// Model matrix for rendering. A combination of translation, rotation, scale
	model_matrix: Matrix4<f32>,
	/// The position of this object's origin in world space
	position: Vector3<f32>,
	/// The rotation of this object in world space
	rotation: Vector3<f32>,
	/// The rotation matrix for rendering
	rotation_matrix: Matrix4<f32>,
	/// The scale of this object compared to world space
	scale: f32,
	/// The scale matrix for rendering
	scale_matrix: Matrix4<f32>,
	/// The name of the shader to use on this object
	shader_name: &'static str,
	/// Translation matrix for rendering
	translation_matrix: Matrix4<f32>,
	/// Contains an index array for rendering
	triangle_indices: Vec<u8>,
	/// A vector of vertices optimized for rendering vs physics
	/// Stored like [x1, y2, z1, x2, y2, z2]
	vertices: Vec<f32>
}
#[allow(dead_code)]
impl Object {
	/// Get the name of the shader to use when rendering this object
	pub fn get_shader_name(&self) -> &'static str {
		self.shader_name
	}

	/// Creates a new Object
	///
	/// * `position` - The position of this object's origin in world space
	/// * `rotation` - The rotation of this object in world space
	/// * `scale` - The relative scale of this object in world space
	/// * `shader_name` - The name of the shader to use on this object
	/// * `triangle_indices` - Contains an index array for rendering
	/// * `vertices` - A vector of vertices optimized for rendering vs physics
	/// Stored like [x1, y2, z1, x2, y2, z2]
	pub fn new(
		position: Vector3<f32>,
		rotation: Vector3<f32>,
		scale: f32,
		shader_name: &'static str,
		triangle_indices: Vec<u8>,
		vertices: Vec<f32>
	) -> Object {
		// Calculate matrices for rendering
		let translation_matrix = Matrix4::new_translation(&position);
		let rotation_matrix = Matrix4::new_rotation(rotation);
		let scale_matrix = Matrix4::new_scaling(scale);
		let model_matrix = translation_matrix * rotation_matrix * scale_matrix;

		// Return Object
		Object {
			model_matrix: model_matrix,
			position: position,
			translation_matrix: translation_matrix,
			rotation: rotation,
			rotation_matrix: rotation_matrix,
			scale: scale,
			scale_matrix: scale_matrix,
			shader_name: shader_name,
			triangle_indices: triangle_indices,
			vertices: vertices
		}
	}

	/// Render this object
	///
	/// If this is None the render just does nothing, because there would be
	/// no point.
	///
	/// * `gl` - the rendering context to use
	/// * `shader_option` - an option to a reference to a compile shader program
	/// * `view_matrix` - the view matrix to render with, usually from a Camera
	pub fn render(
		&self,
		gl: &WebGlRenderingContext,
		shader_option: Option<&WebGlProgram>,
		view_matrix: &[f32],
		projection_matrix: &[f32]
	) {
		// Check that the shader exists, if not just don't render
		if shader_option == None {
			return
		}
		let shader = shader_option.unwrap();

		// Set shader
		gl.use_program(shader_option);

		// Set MVP uniform values
		let model_uniform = gl.get_uniform_location(&shader, "model");
		gl.uniform_matrix4fv_with_f32_array(model_uniform.as_ref(), false, self.model_matrix.as_slice());

		let view_uniform = gl.get_uniform_location(&shader, "view");
		gl.uniform_matrix4fv_with_f32_array(view_uniform.as_ref(), false, view_matrix);

		let projection_uniform = gl.get_uniform_location(&shader, "projection");
		gl.uniform_matrix4fv_with_f32_array(projection_uniform.as_ref(), false, projection_matrix);

		// Set array buffer
		// Avoid creating a new buffer for every draw in the future
		let array_buffer = gl.create_buffer();
		let ab_ref = array_buffer.as_ref();
		gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, ab_ref);

		// Allocate memory for vertex array
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

		// Set vertex array
		let vertices_location = self.vertices.as_ptr() as u32 / 4;
		let vert_array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(vertices_location, vertices_location + self.vertices.len() as u32);
		gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &vert_array, WebGlRenderingContext::STATIC_DRAW);

		// Set index buffer
		let index_buffer = gl.create_buffer();
		let ib_ref = index_buffer.as_ref();
		gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, ib_ref);
		gl.buffer_data_with_u8_array(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, &self.triangle_indices[..], WebGlRenderingContext::STATIC_DRAW);

		// Set attrib pointer
		gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
		gl.enable_vertex_attrib_array(0);

		// Draw
		gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, self.triangle_indices.len() as i32, WebGlRenderingContext::UNSIGNED_BYTE, 0);

		// Deallocate buffers
		gl.delete_buffer(ib_ref);
		gl.delete_buffer(array_buffer.as_ref());
	}
}
