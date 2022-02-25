use std::collections::HashMap;

use web_sys::{WebGlProgram, WebGlUniformLocation};

pub mod shader_sources;

/// Holds information regarding a compiled shader
pub struct CompiledShader {
	/// Location of the model uniform
	pub model_uniform: Option<Uniform>,
	/// The compiled shader program
	pub program: Option<WebGlProgram>,
	/// Location of the projection uniform
	pub projection_uniform: Option<Uniform>,
	/// Map of other uniform locations
	pub uniforms: HashMap<&'static str, Uniform>,
	/// Location of the view uniform
	pub view_uniform: Option<Uniform>
}

/// Holds source code for a shader
/// Made up of Options to strs
pub struct ShaderSource<'a> {
	pub vertex_shader: Option<&'a str>,
	pub fragment_shader: Option<&'a str>,
	/// names for uniforms other than model, view, and projection
	pub uniform_names: Vec<&'a str>,
	/// types for uniforms other than model, view, and projection
	pub uniform_types: Vec<UniformType>
}

/// Describes the location and type of a uniform
pub struct Uniform {
	pub location: Option<WebGlUniformLocation>,
	pub u_type: UniformType
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum UniformType {
	Mat4
}
