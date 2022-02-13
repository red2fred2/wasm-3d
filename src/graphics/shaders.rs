use web_sys::{WebGlProgram, WebGlRenderingContext};
use super::gl;

// Shader type indices
const VERTEX_SHADER: usize = 0;
const FRAGMENT_SHADER: usize = 1;
const NUM_SHADER_TYPES: usize = 2;

// Shader name indices
pub const BASIC_BITCH: usize = 0;
const NUM_SHADERS: usize = 1;

// The shader source code
const SHADER_SOURCE: [[&str; NUM_SHADER_TYPES]; NUM_SHADERS] = [

	[   // Basic bitch shader
		include_str!("shaders/here_vert.glsl-min"),
		include_str!("shaders/orange_frag.glsl-min")
	]
];

/**
 * build_program
 * Compiles and links a shader program
 * @param context the webGL rendering context for this program
 * @param index the index of the program to use
 * @return the built program
 */
pub fn build_program(context: &WebGlRenderingContext, index: usize) -> Option<WebGlProgram> {
	// Compile the vertex shader
	let vertex_shader = gl::compile_shader(
	context,
	WebGlRenderingContext::VERTEX_SHADER,
	SHADER_SOURCE[index][VERTEX_SHADER],
	);

	// Compile the fragment shader
	let fragment_shader = gl::compile_shader(
	context,
	WebGlRenderingContext::FRAGMENT_SHADER,
	SHADER_SOURCE[index][FRAGMENT_SHADER],
	);

	// Link the program
	match (vertex_shader, fragment_shader) {
		(Ok(vert), Ok(frag)) =>
			gl::link_program(&context, &vert, &frag).ok(),
		_ => None
	}
}

pub fn compile_shaders(context: &WebGlRenderingContext) -> [Option<WebGlProgram>; NUM_SHADERS] {
	let mut shader_programs: [Option<WebGlProgram>; NUM_SHADERS];


	[
		build_program(context, BASIC_BITCH)
	]
}
