use super::object::Object;

/// Contains the game world
pub struct World {
	objects: Vec<Object>
}

impl World {
	/// Gets all the objects in the world as a vector
	///
	/// Returns - the vector containing all the objects in the world
	pub fn get_objects(&self) -> &Vec<Object> {
		&self.objects
	}

	/// Initialize the game world
	pub fn init() -> World {
		let objects = load_objects();

		World {
			objects: objects
		}
	}

	/// Update the game world for dt seconds
	///
	/// * `dt` - the time difference since this function last ran
	pub fn update(&self, dt: f32) {
		let _ = dt;
	}
}

/// Loads all the objects in this world
fn load_objects() -> Vec<Object> {
	let mut objects = Vec::new();

	// Left square
	let vertices = vec![
		-1.,  1., 0.,
		 0.,  1., 0.,
		 1.,  1., 0.,
		-1.,  0., 0.,
		 0.,  0., 0.,
		 1.,  0., 0.,
		-1., -1., 0.,
		 0., -1., 0.,
		 1., -1., 0.
	];
	let indices = vec![
		1, 3, 4,
		3, 4, 7
	];
	objects.push(Object::new("Basic bitch", indices, vertices));

	// Right triangle
	let vertices = vec![
		-1.,  1., 0.,
		 0.,  1., 0.,
		 1.,  1., 0.,
		-1.,  0., 0.,
		 0.,  0., 0.,
		 1.,  0., 0.,
		-1., -1., 0.,
		 0., -1., 0.,
		 1., -1., 0.
	];
	let indices = vec![
		1, 2, 7
	];
	objects.push(Object::new("Basic bitch", indices, vertices));

	objects
}
