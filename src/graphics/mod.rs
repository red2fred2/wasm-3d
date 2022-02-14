pub mod shaders;
mod gl;

use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
use crate::object::Object;

/// Initialize graphics
pub fn init() {
	// Set up the front end
	let context = set_up_front_end();

	// Compile shaders
	let shaders = shaders::compile_shaders(&context);

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
	shaders: &[std::option::Option<web_sys::WebGlProgram>; shaders::NUM_SHADERS]
) {
	// Set object to render
	let top_right = Object::new(
		vec![
			1.0, 1.0, 0.0,
			0.0, 1.0, 0.0,
			0.0, 0.0, 0.0,

			0.0, 0.0, 0.0,
			1.0, 1.0, 0.0,
			1.0, 0.0, 0.0
		],
		shaders::BASIC_BITCH
	);

	// Set shader
	let shader = &shaders[top_right.get_shader()];
    context.use_program(shader.as_ref());

	// Initialize the array buffer
	let array_buffer = context.create_buffer();
	context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

	// Set up the shared memory buffer between rust and the web
	unsafe {
		context.buffer_data_with_array_buffer_view(
			WebGlRenderingContext::ARRAY_BUFFER,
			&js_sys::Float32Array::view(top_right.get_vertices()),
			WebGlRenderingContext::STATIC_DRAW,
		);
	}

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
	context.enable_vertex_attrib_array(0);

	// Clear the screen then draw
	gl::clear(&context);
	gl::draw(&context, 2);
}

/// Set up front end
///
/// Sets up the window, canvas, and returns a valid rendering context for webGL
/// Shits the bed when it fails, because there's no reason to continue without it
///
/// returns - rendering context
fn set_up_front_end() -> WebGlRenderingContext {
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
