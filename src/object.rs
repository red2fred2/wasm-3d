use crate::graphics::shaders::ShaderIndex;

/// An object in the world
pub struct Object {
	/// A vector of vertices in 3d
	///
	/// x, y, z = i%3, i%3+1, i%3+2
	vertices: Vec<f32>,
	/// The index of the shader to use
	shader: ShaderIndex
}

impl Object {
	/// Get which shader to use for this object
	pub fn get_shader(&self) -> ShaderIndex {
		self.shader
	}

	/// Gets the vertices of this object
	pub fn get_vertices(&self) -> &Vec<f32> {
		&self.vertices
	}

	/// Creates a new object
	///
	/// * `vertices` - A vector of vertices in 3d
	/// 		       x, y, z = i%3, i%3+1, i%3+2
	/// * `shader` - The index of the shader to use
	pub fn new(vertices: Vec<f32>, shader: ShaderIndex) -> Object {
		Object {vertices: vertices, shader: shader}
	}
}
