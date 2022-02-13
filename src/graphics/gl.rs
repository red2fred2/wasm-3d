use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

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

/// Clear the screen to prepare for drawing
///
/// * `context` - the rendering context
pub fn clear(context: &WebGlRenderingContext) {
	context.clear_color(0.0, 0.0, 0.0, 1.0);
	context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
}

/// Draws the scene to the screen
///
/// * `context` - the rendering context
pub fn draw(context: &WebGlRenderingContext) {
	context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 3i32);
}
