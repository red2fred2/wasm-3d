mod camera;
mod gl;
mod shaders;

use std::collections::HashMap;

use nalgebra::{Matrix4, Point3};
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlProgram, HtmlCanvasElement, WebGlBuffer};

use crate::logic::world::World;
use self::{shaders::shader_sources::get_shader_sources, camera::Camera};

pub struct Graphics {
	// The rendering buffer
	array_buffer: Option<WebGlBuffer>,
	/// A camera to be rendered from
	camera: Camera,
	/// The webgl context to render to
	context: WebGlRenderingContext,
	/// The index buffer
	index_buffer: Option<WebGlBuffer>,
	/// The projection matrix to apply to renders
	projection_matrix: Matrix4<f32>,
	/// The shaders that have been compiled
	shaders: HashMap<&'static str, Option<WebGlProgram>>
}

/// Holds all information regarding the graphics of the application
impl Graphics {
	/// Initialize graphics
	pub fn init() -> Graphics {
		// Set up the front end
		let context = gl::set_up_canvas();

		// Compile shaders
		let shader_source = get_shader_sources();
		let mut shaders = HashMap::new();

		for (name, source) in shader_source.iter() {
			let compiled_shader = gl::build_program(&context, source);
			shaders.insert(name as &str, compiled_shader);
		}

		// Create generic camera
		let origin = Point3::new(0.0, 0.0, 0.0);
		let starting_location = Point3::new(1.0, 0.0, 4.0);
		let camera = Camera::new_targeted(starting_location, origin, 0.0);

		// Create projection matrix
		// Find information about the screen being rendered to
		let canvas: HtmlCanvasElement = context.canvas().unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
		let width = canvas.client_width() as f32;
		let height = canvas.client_height() as f32;
		let aspect_ratio = width / height;

		// If the screen were 16:9, then the x field of view should be 90 degrees
		let fov_x_degrees_16x9 = 90.0;
		// Convert to y FOV in radians without worrying about the aspect ratio
		let fov_y_radians = fov_x_degrees_16x9 * 9.0 / 16.0 * 3.14159 / 180.0;

		let projection_matrix: Matrix4<f32> = Matrix4::new_perspective(aspect_ratio, fov_y_radians, 0.0, 100.0);

		// Create and bind array buffer for webGL
		let array_buffer = context.create_buffer();
		context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, array_buffer.as_ref());

		// Create index buffer for webGL
		let index_buffer = context.create_buffer();

		// Return newly created Graphics object
		Graphics {
			array_buffer,
			context,
			index_buffer,
			shaders,
			camera,
			projection_matrix
		}
	}

	/// Renders a frame to the screen
	pub fn render(&mut self, world: &World) {
		// Set gl to the rendering context for easier use
		let gl = &self.context;

		// Clear the screen for rendering
		gl.clear_color(0.0, 0.0, 0.0, 1.0);
		gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

		// Move camera
		let timer = world.get_time_elapsed();
		let cam_time = timer / 5_000.0;
		let cam_x = 3.0*(cam_time * 2.0 * 3.14).cos();
		let cam_y = 3.0*(cam_time * 2.0 * 3.14).sin();
		let cam_pos = Point3::new(cam_x, cam_y, 4.0);
		self.camera.teleport_keep_target(cam_pos);

		// Render object by object
		let objects = world.get_objects();
		for object in objects {
			// Get shader to use
			let shader_name = object.get_shader_name();
			let shader = self.shaders.get(shader_name);

			// Combine nested Options
			let shader = match shader {
				Some(Some(program)) => Some(program),
				_ => None
			};

			gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, self.index_buffer.as_ref());
			object.render(gl, shader, &self.camera.get_view_matrix().as_slice(), self.projection_matrix.as_slice());
		}
	}
}

impl Drop for Graphics {
	/// Destructor
	fn drop(&mut self) {
		// Free buffers from webGL memory
		self.context.delete_buffer(self.array_buffer.as_ref());
		self.context.delete_buffer(self.index_buffer.as_ref());
	}
}