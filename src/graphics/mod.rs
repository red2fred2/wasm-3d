mod camera;
mod gl;
mod shaders;

use std::collections::HashMap;

use nalgebra::Point3;
use web_sys::{WebGlRenderingContext, WebGlProgram};

use crate::logic::world::World;
use self::{shaders::shader_sources::get_shader_sources, camera::Camera};

pub struct Graphics {
	camera: Camera,
	context: WebGlRenderingContext,
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

		// Return newly created Graphics object
		Graphics{context: context, shaders: shaders, camera: camera}
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

			object.render(gl, shader, &self.camera.get_view_matrix().as_slice());
		}
	}
}
