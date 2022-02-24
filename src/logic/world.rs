use super::object::Object;

/// Contains the game world
pub struct World {
	objects: Vec<Object>,
	time_elapsed: f32
}

impl World {
	/// Gets all the objects in the world as a vector
	///
	/// Returns - the vector containing all the objects in the world
	pub fn get_objects(&self) -> &Vec<Object> {
		&self.objects
	}

	/// Gets the time (in seconds) since the program started
	pub fn get_time_elapsed(&self) -> f32 {
		self.time_elapsed
	}

	/// Initialize the game world
	pub fn init() -> World {
		let objects = load_objects();

		World {
			objects: objects,
			time_elapsed: 0.0
		}
	}

	/// Update the game world for dt seconds
	///
	/// * `dt` - the time difference since this function last ran
	pub fn update(&mut self, dt: f32) {
		self.time_elapsed += dt;
	}
}

/// Loads all the objects in this world
fn load_objects() -> Vec<Object> {
	let mut objects = Vec::new();

	let vertices = vec![
		-0.5,  0.5,  0.5,
		 0.5,  0.5,  0.5,
		-0.5, -0.5,  0.5,
		 0.5, -0.5,  0.5,
		-0.5,  0.5, -0.5,
		 0.5,  0.5, -0.5,
		-0.5, -0.5, -0.5,
		 0.5, -0.5, -0.5
	];

	let indices = vec![
		// // Front face
		// 0, 1, 2,
		// 1, 2, 3,
		// Right face
		1, 3, 5,
		3, 5, 7,
		// // Left face
		// 0, 2, 4,
		// 2, 4, 6,
		// // Back face
		// 4, 5, 6,
		// 5, 6, 7,
		// Top face
		0, 1, 4,
		1, 4, 5,
		// Bottom face
		2, 3, 6,
		3, 6, 7

	];
	objects.push(Object::new("3d orange", indices, vertices));

	objects
}
