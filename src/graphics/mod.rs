pub mod shaders;
mod gl;

use std::collections::HashMap;

use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram};

use self::shaders::{ShaderSource, shader_sources::get_shader_sources};

/// Compiles and links a shader program
///
/// * `context` - the webGL rendering context for this program
/// * `index` - the index of the program to use
///
/// Returns - the built program
pub fn build_program(context: &WebGlRenderingContext, source: &ShaderSource) -> Option<WebGlProgram> {

	// Compile the vertex shader
	let vertex_shader = gl::compile_shader(
		context,
		WebGlRenderingContext::VERTEX_SHADER,
		source.vertex_shader.unwrap()
	);


	// Compile the fragment shader
	let fragment_shader = gl::compile_shader(
		context,
		WebGlRenderingContext::FRAGMENT_SHADER,
		source.fragment_shader.unwrap(),
	);

	// Link the program
	match (vertex_shader, fragment_shader) {
		(Ok(vert), Ok(frag)) => gl::link_program(&context, &vert, &frag).ok(),
		_ => None
	}
}


/// Initialize graphics
pub fn init() {
	// Set up the front end
	let context = set_up_front_end();

	// Compile shaders
	let shader_source = get_shader_sources();
	let mut shaders = HashMap::new();
	for (name, source) in shader_source.iter() {
		let compiled_shader = build_program(&context, source);
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
