pub mod shader_sources;

/// Holds source code for a shader
/// Made up of Options to strs
pub struct ShaderSource<'a> {
	pub vertex_shader: Option<&'a str>,
	pub fragment_shader: Option<&'a str>
}
