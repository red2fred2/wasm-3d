use js_sys::{WebAssembly, Float32Array};
use nalgebra::{Matrix4, Vector3, Point3};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::WebGlRenderingContext;

use crate::graphics::{shaders::CompiledShader, gl};

/// Something that can be rendered to the screen
pub struct Object {
	/// The JS memory allocated for this object
	js_memory: JsValue,
	/// The JS buffer contained the vertices for rendering
	js_vertex_buffer: Float32Array,
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
	/// Change the scale of this object relative to the world
	///
	/// * `scale` - the scale to set, 1.0 being bounded at 2 units^3
	pub fn change_scale(&mut self, scale: f32) {
		self.scale = scale;
		self.scale_matrix = Matrix4::new_scaling(scale);
	}

	/// Get the name of the shader to use when rendering this object
	pub fn get_shader_name(&self) -> &'static str {
		self.shader_name
	}

	/// Moves this object in some direction over some vector
	///
	/// * `direction` - the vector to move this object by
	pub fn move_dir(&mut self, direction: Vector3<f32>) {
		let position = self.position + direction;
		self.position = position;
		self.translation_matrix = Matrix4::new_translation(&position);
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

		// Allocate memory for vertex array
		let memory = wasm_bindgen::memory()
		.dyn_into::<WebAssembly::Memory>().expect("Failed to allocate memory to render an Object");

		let js_memory = memory.buffer();

		// Set vertex array
		let vertices_location = vertices.as_ptr() as u32 / 4;
		let js_vertex_buffer = js_sys::Float32Array::new(&js_memory)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

		// Return Object
		Object {
			js_memory,
			js_vertex_buffer,
			model_matrix,
			position,
			translation_matrix,
			rotation,
			rotation_matrix,
			scale,
			scale_matrix,
			shader_name,
			triangle_indices,
			vertices
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
		shader: &CompiledShader
	) {
		// Set Model uniform value
		gl::set_mat4_uniform(gl, &shader.model_uniform, self.model_matrix.as_slice());

		// Set vertex buffer
		gl.buffer_data_with_array_buffer_view(WebGlRenderingContext::ARRAY_BUFFER, &self.js_vertex_buffer, WebGlRenderingContext::STATIC_DRAW);

		// Set index buffer
		gl.buffer_data_with_u8_array(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, &self.triangle_indices[..], WebGlRenderingContext::STATIC_DRAW);

		// Set attrib pointer
		gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
		gl.enable_vertex_attrib_array(0);

		// Draw
		gl.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, self.triangle_indices.len() as i32, WebGlRenderingContext::UNSIGNED_BYTE, 0);
	}

	/// Rotate this object in all directions
	///
	/// * `pitch` -  how much this object is pitched up from the horizon
	/// * `yaw` - how much this object is yawed clockwise (when viewed from above)
	/// * `roll` - how much this object is rolled clockwise
	pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {

	}

	/// Set absolute rotations for this object
	///
	/// * `pitch` -  how much this object is pitched up from the horizon
	/// * `yaw` - how much this object is yawed clockwise (when viewed from above)
	/// * `roll` - how much this object is rolled clockwise
	pub fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) {

	}

	/// Teleports this object to a new position in the world
	///
	/// * `position` - the position to teleport to
	/// Specifically moves the origin point of this object to this position
	pub fn teleport(&mut self, position: Vector3<f32>) {
		self.position = position;
		self.translation_matrix = Matrix4::new_translation(&position);
	}
}
