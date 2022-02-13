use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;
use super::{gl, shaders};

/// Initialize graphics
pub fn init() {
	// Set up the front end
	let (context, screen_width, screen_height) = set_up_front_end();

	// Compile shaders
	let shaders = shaders::compile_shaders(&context);

	// Set shader
	let program = &shaders[shaders::BASIC_BITCH];
    context.use_program(program.as_ref());

	// Triangle vertices
	let vertices: [f32; 9] = [
		-0.7, -0.7, 0.0,
		 0.7, -0.7, 0.0,
		 0.0,  0.7, 0.0
	];

	// Initialize the array buffer
	let array_buffer = context.create_buffer();
	context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

	// Set up the shared memory buffer between rust and the web
	unsafe {
		let vert_array = js_sys::Float32Array::view(&vertices);

		context.buffer_data_with_array_buffer_view(
			WebGlRenderingContext::ARRAY_BUFFER,
			&vert_array,
			WebGlRenderingContext::STATIC_DRAW,
		);
	}

    context.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
	context.enable_vertex_attrib_array(0);

	// Clear the screen then draw
	gl::clear(&context);
	gl::draw(&context);
}

/// Set up front end
///
/// Sets up the window, canvas, and returns a valid rendering context for webGL
/// Shits the bed when it fails, because there's no reason to continue without it
///
/// returns - (rendering context, window width, window height)
fn set_up_front_end() -> (WebGlRenderingContext, u32, u32) {
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
	let context = canvas.get_context("webgl").unwrap().unwrap()
		.dyn_into::<WebGlRenderingContext>().unwrap();

	// Return
	(context, width, height)
}
