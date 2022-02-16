use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
use super::shaders::ShaderSource;

/// Compiles and links a shader program
///
/// * `context` - the webGL rendering context for this program
/// * `index` - the index of the program to use
///
/// Returns - the built program
pub fn build_program(context: &WebGlRenderingContext, source: &ShaderSource) -> Option<WebGlProgram> {

	// Compile the vertex shader if it exists
	let vertex_shader = match source.vertex_shader {
		Some(shader) => compile_shader(
			context,
			WebGlRenderingContext::VERTEX_SHADER,
			shader
		).ok(),
		_ => None
	};

	// Compile the fragment shader if it exists
	let fragment_shader = match source.fragment_shader {
		Some(shader) => compile_shader(
			context,
			WebGlRenderingContext::FRAGMENT_SHADER,
			shader
		).ok(),
		_ => None
	};

	// Link the program
	match (vertex_shader, fragment_shader) {
		(Some(vert), Some(frag)) => link_program(&context, &vert, &frag).ok(),
		_ => None
	}
}

/// Set up front end canvas
///
/// Sets up the window, canvas, and returns a valid rendering context for webGL
/// Shits the bed when it fails, because there's no reason to continue without it
///
/// returns - rendering context
pub fn set_up_canvas() -> WebGlRenderingContext {
	// Get to the canvas object
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	let canvas = document.get_element_by_id("webGL")
		.unwrap()
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.unwrap();

	// Set canvas to full window size
	let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
	let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

	canvas.set_width(width);
	canvas.set_height(height);

	// Get context
	canvas.get_context("webgl").unwrap().unwrap()
		.dyn_into::<WebGlRenderingContext>().unwrap()
}


/// Compiles a GLSL shader
///
/// * `context` - the GL context this is rendering in
/// * `shader_type` - the type of shader being compiled
/// * `source` - the source code for this shader
///
/// Returns - A result that may either have a compiled shader or a compile error
pub fn compile_shader(
	context: &WebGlRenderingContext,
	shader_type: u32,
	source: &str,
) -> Result<WebGlShader, String> {

	// Create the shader
	let shader = context
		.create_shader(shader_type)
		.ok_or_else(|| String::from("Unable to create shader object"))?;

	// Load and compile
	context.shader_source(&shader, source);
	context.compile_shader(&shader);

	// Check if it went well
	if context
		.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
		.as_bool()
		.unwrap_or(false)
	{
		Ok(shader)
	} else {
		// Return the error
		Err(context
			.get_shader_info_log(&shader)
			.unwrap_or_else(|| String::from("Unknown error creating shader")))
	}
}

/// Links a webGL program
///
/// * `context` - the GL context this is rendering in
/// * `vertex_shader` - the vertex shader to link
/// * `frag_shader` - the fragment shader to link
///
/// Returns - a result containing the webGL program
pub fn link_program(
	context: &WebGlRenderingContext,
	vertex_shader: &WebGlShader,
	frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {

	// Create the program
	let program = context
		.create_program()
		.ok_or_else(|| String::from("Unable to create shader object"))?;

	// Add and link the shaders
	context.attach_shader(&program, vertex_shader);
	context.attach_shader(&program, frag_shader);
	context.link_program(&program);

	// Check if it went well
	if context
		.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
		.as_bool()
		.unwrap_or(false)
	{
		Ok(program)
	} else {
		// Return the error
		Err(context
			.get_program_info_log(&program)
			.unwrap_or_else(|| String::from("Unknown error creating program object")))
	}
}
