use web_sys::{WebGlProgram, WebGlRenderingContext};
use super::gl;

// Shader type indices
type ShaderType = usize;
const VERTEX_SHADER: ShaderType = 0;
const FRAGMENT_SHADER: ShaderType = 1;
const NUM_SHADER_TYPES: usize = 2;

// Shader name indices
pub type ShaderIndex = usize;
pub const BASIC_BITCH: ShaderIndex = 0;

pub const NUM_SHADERS: usize = 1;

/// An array including the shader source code
const SHADER_SOURCE: [[&str; NUM_SHADER_TYPES]; NUM_SHADERS] = [

	[   // Basic bitch shader
		include_str!("here_vert.glsl-min"),
		include_str!("orange_frag.glsl-min")
	]
];

/// Compiles and links a shader program
///
/// * `context` - the webGL rendering context for this program
/// * `index` - the index of the program to use
///
/// Returns - the built program
pub fn build_program(context: &WebGlRenderingContext, index: ShaderIndex) -> Option<WebGlProgram> {
	// Compile the vertex shader
	let vertex_shader = gl::compile_shader(
		context,
		WebGlRenderingContext::VERTEX_SHADER,
		SHADER_SOURCE[index][VERTEX_SHADER]
	);


	// Compile the fragment shader
	let fragment_shader = gl::compile_shader(
		context,
		WebGlRenderingContext::FRAGMENT_SHADER,
		SHADER_SOURCE[index][FRAGMENT_SHADER],
	);

	// Link the program
	match (vertex_shader, fragment_shader) {
		(Ok(vert), Ok(frag)) => gl::link_program(&context, &vert, &frag).ok(),
		_ => None
	}
}

/// Compiles shaders and returns a slice of program options
///
/// * `context` - the webgl context to compile for
pub fn compile_shaders(context: &WebGlRenderingContext) -> [Option<WebGlProgram>; NUM_SHADERS] {
	[
		build_program(context, BASIC_BITCH)
	]
}
