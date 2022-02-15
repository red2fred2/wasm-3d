mod shaders;
mod gl;

use std::collections::HashMap;
use web_sys::{WebGlRenderingContext, WebGlProgram};
use self::shaders::shader_sources::get_shader_sources;

/// Initialize graphics
pub fn init() {
	// Set up the front end
	let context = gl::set_up_canvas();

	// Compile shaders
	let shader_source = get_shader_sources();
	let mut shaders = HashMap::new();
	for (name, source) in shader_source.iter() {
		let compiled_shader = gl::build_program(&context, source);
		shaders.insert(name as &str, compiled_shader);
	}

	// Render a frame
	render(&context, &shaders);
}

/// Renders a frame to the screen
///
/// * `context` - webGL rendering context
/// * `screen_width` - the width of the screen in pixels
/// * `screen_height` - the height of the screen in pixels
/// * `shaders` - the compiled shaders available to use
fn render(
	context: &web_sys::WebGlRenderingContext,
	shaders: &HashMap<&str, Option<WebGlProgram>>
) {
	// Set shader
	let shader = shaders.get("Basic bitch").unwrap();
    context.use_program(shader.as_ref());

	// Initialize the array buffer
	let array_buffer = context.create_buffer();
	context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

	// Set up the shared memory buffer between rust and the web
	unsafe {
		context.buffer_data_with_array_buffer_view(
			WebGlRenderingContext::ARRAY_BUFFER,
			&js_sys::Float32Array::view(&vec![
				1.0, 1.0, 0.0,
				0.0, 1.0, 0.0,
				0.0, 0.0, 0.0,

				0.0, 0.0, 0.0,
				1.0, 1.0, 0.0,
				1.0, 0.0, 0.0
			]),
			WebGlRenderingContext::STATIC_DRAW,
		);
	}

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
	context.enable_vertex_attrib_array(0);

	// Clear the screen then draw
	context.clear_color(0.0, 0.0, 0.0, 1.0);
	context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
	context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
}
