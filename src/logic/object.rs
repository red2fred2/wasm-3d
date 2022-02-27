use js_sys::{WebAssembly, Float32Array};
use nalgebra::{Matrix4, Vector3, UnitQuaternion, UnitVector3};
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
	/// The unit quaternion that describes the orientation of this object
	///
	/// Pure evil, but really good at what it does unfortunately. Hopefully
	/// this abomination of mathematics can be fully abstracted away so I can
	/// pretend to go on living with the childlike innocence I once had before
	/// attempting to understand the Lovecraftian horrors that these unleash.
	orientation_quaternion: UnitQuaternion<f32>,
	/// The position of this object's origin in world space
	position: Vector3<f32>,
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
	fn change_scale(&mut self, scale: f32) {
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
	fn move_dir(&mut self, direction: Vector3<f32>) {
		let position = self.position + direction;
		self.position = position;
		self.translation_matrix = Matrix4::new_translation(&position);
		self.model_matrix = self.translation_matrix * self.scale_matrix;
	}

	/// Creates a new Object
	///
	/// * `position` - The position of this object's origin in world space
	/// * `pitch` - The pitch of this object up from the horizon (radians)
	/// * `yaw` - The yaw of this object clockwise viewed from above (radians)
	/// * `roll` - The roll of this object clockwise viewed in the x direction (radians)
	/// * `scale` - The relative scale of this object in world space
	/// * `shader_name` - The name of the shader to use on this object
	/// * `triangle_indices` - Contains an index array for rendering
	/// * `vertices` - A vector of vertices optimized for rendering vs physics
	/// Stored like [x1, y2, z1, x2, y2, z2]
	pub fn new(
		position: Vector3<f32>,
		pitch: f32,
		yaw: f32,
		roll: f32,
		scale: f32,
		shader_name: &'static str,
		triangle_indices: Vec<u8>,
		vertices: Vec<f32>
	) -> Object {
		// Calculate matrices for rendering
		let translation_matrix = Matrix4::new_translation(&position);
		let scale_matrix = Matrix4::new_scaling(scale);
		let model_matrix = translation_matrix * scale_matrix;

		// Allocate memory for vertex array
		let memory = wasm_bindgen::memory()
		.dyn_into::<WebAssembly::Memory>().expect("Failed to allocate memory to render an Object");

		let js_memory = memory.buffer();

		// Set vertex array
		let vertices_location = vertices.as_ptr() as u32 / 4;
		let js_vertex_buffer = js_sys::Float32Array::new(&js_memory)
            .subarray(vertices_location, vertices_location + vertices.len() as u32);

		// Generate the orientation quaternion
		let orientation_quaternion = UnitQuaternion::from_euler_angles(roll, pitch, yaw);

		// Return Object
		Object {
			js_memory,
			js_vertex_buffer,
			model_matrix,
			orientation_quaternion,
			position,
			translation_matrix,
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
	fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
		let rotation = UnitQuaternion::from_euler_angles(roll, pitch, yaw);
		self.orientation_quaternion *= rotation;
	}

	/// Sets the rotation of this object to aim in the same direction as a vector
	///
	/// Uses an axis angle description
	///
	/// * `vector` - the vector to face this object in the direction of
	/// * `roll` - the roll angle after aiming in that direction
	fn set_direction_from_vector(&mut self, vector: Vector3<f32>, roll: f32) {
		let axis = UnitVector3::new_normalize(vector);
		self.orientation_quaternion = UnitQuaternion::from_axis_angle(&axis, roll);
	}

	/// Set absolute rotations for this object
	///
	/// * `pitch` -  how much this object is pitched up from the horizon
	/// * `yaw` - how much this object is yawed clockwise (when viewed from above)
	/// * `roll` - how much this object is rolled clockwise
	fn set_rotation(&mut self, pitch: f32, yaw: f32, roll: f32) {
		self.orientation_quaternion = UnitQuaternion::from_euler_angles(roll, pitch, yaw);
	}

	/// Teleports this object to a new position in the world
	///
	/// * `position` - the position to teleport to
	/// Specifically moves the origin point of this object to this position
	fn teleport(&mut self, position: Vector3<f32>) {
		self.position = position;
		self.translation_matrix = Matrix4::new_translation(&position);
	}

	/// Update function for this object
	///
	/// Will probably need to be overloaded or whatever rust's version is
	/// by types of object.
	///
	/// * `dt` - change in time since this object has been updated
	pub fn update(&mut self, dt: f32) {
		let dir = Vector3::new(0.0, 0.0, 1.0);
		let vector = dir * (dt/1000.0);
		self.move_dir(vector);
	}
}
