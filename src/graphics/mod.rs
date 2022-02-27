mod camera;
pub mod gl;
pub mod shaders;

use std::collections::HashMap;

use nalgebra::{Matrix4, Point3};
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, HtmlCanvasElement, WebGlBuffer};

use crate::logic::world::World;
use self::{shaders::{shader_sources::get_shader_sources, CompiledShader, UniformType, Uniform}, camera::Camera};

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
	shaders: HashMap<&'static str, CompiledShader>
}

/// Holds all information regarding the graphics of the application
impl Graphics {
	/// Compiles shaders and returns them as a hash map
///
/// * `context` - the webGL rendering context these are being compiled in
pub fn compile_shaders(&mut self) {
	// Compile shaders
	let shader_source = get_shader_sources();
	let mut shaders = HashMap::new();

	// Compile each shader and insert to map
	for (name, source) in shader_source.iter() {
		// Compile program
		let compiled_program = gl::build_program(&self.context, source);

		// Check if it compiled correctly
		match &compiled_program {
			Some(program) => {
				// If success
				// Find uniform locations
				// Start with MVP uniforms
				let model_location = self.context.get_uniform_location(&program, "model");
				let model_uniform = Uniform {
					location: model_location,
					u_type: UniformType::Mat4
				};
				let view_location = self.context.get_uniform_location(&program, "view");
				let view_uniform = Uniform {
					location: view_location,
					u_type: UniformType::Mat4
				};
				let projection_location = self.context.get_uniform_location(&program, "projection");
				let projection_uniform = Uniform {
					location: projection_location,
					u_type: UniformType::Mat4
				};

				// Find other uniforms
				let mut uniforms = HashMap::new();

				// Run through each uniform name
				// there should be a matching type in uniform_types
				for i in 0..source.uniform_names.len() {
					let uniform_name = source.uniform_names[i];
					let u_type = source.uniform_types[i];

					let location = self.context.get_uniform_location(&program, uniform_name);

					let uniform = Uniform {location, u_type};

					uniforms.insert(uniform_name, uniform);
				}

				// Put it all together
				let compiled_shader = CompiledShader {
					model_uniform: Some(model_uniform),
					program: compiled_program,
					projection_uniform: Some(projection_uniform),
					uniforms,
					view_uniform: Some(view_uniform)
				};
				shaders.insert(name as &str, compiled_shader);
			},
			_ => ()
		}
	}

	self.shaders = shaders;
}


	/// Initialize graphics
	pub fn init() -> Graphics {
		// Set up the front end
		let context = gl::set_up_canvas();

		// Create generic camera
		let origin = Point3::new(0.0, 0.0, 0.0);
		let starting_location = Point3::new(0.0, 0.0, 4.0);
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

		let shaders = HashMap::new();

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

		// Render object by object
		let objects = world.get_objects();
		for object in objects {
			// Get shader to use
			let shader_name = object.get_shader_name();
			let shader = self.shaders.get(shader_name);

			// Only render if it can find the shader
			match shader {
				Some(shader) => {
					// If it found the shader

					// Prepare to render
					gl.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, self.index_buffer.as_ref());

					// Set shader
					gl.use_program(shader.program.as_ref());

					// Set view matrix
					let view_matrix = self.camera.get_view_matrix().as_slice();
					gl::set_mat4_uniform(&gl, &shader.view_uniform, view_matrix);

					// Set projection matrix
					let projection_matrix = self.projection_matrix.as_slice();
					gl::set_mat4_uniform(&gl, &shader.projection_uniform, projection_matrix);

					// Render
					object.render(gl, shader);
				},
				_ => ()
			}
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
