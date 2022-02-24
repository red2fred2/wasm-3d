use nalgebra::{Matrix4, Point3, Vector3};

/// Represents a camera that can render a view of the world
pub struct Camera {
	/// The coordinates this camera is at
	location: Point3<f32>,
	/// View matrix specific to this camera
	matrix: Matrix4<f32>,
	/// The amount (in radians) this camera has rolled clockwise from world up
	roll: f32,
	/// The coordinates this camera is aiming at
	target: Point3<f32>
}

#[allow(dead_code)]
impl Camera {
	/// Changes the direction this camera is facing
	///
	/// * `new direction` - a vector representing the new direction for the camera
	pub fn change_direction(&mut self, new_direction: Vector3<f32>) {
		// Get updated camera data
		let new = Self::new_directional(self.location, new_direction, self.roll);

		// Apply changes
		self.matrix = new.matrix;
		self.target = new.target;
	}

	/// Changes the target of this camera
	///
	/// * `new_target` - the target point to switch to
	pub fn change_target(&mut self, new_target: Point3<f32>) {
		// Get updated data
		let new = Self::new_targeted(self.location, new_target, self.roll);

		// Apply changes
		self.matrix = new.matrix;
		self.target = new.target;
	}

	/// Get the view matrix from this camera for rendering
	pub fn get_view_matrix(&self) -> &Matrix4<f32> {
		&self.matrix
	}

	/// Creates a new generic camera at the world origin looking straight in the
	/// X direction
	pub fn new() -> Camera {
		// Arbitrary default values
		let direction = Vector3::new(1.0, 0.0, 0.0);
		let location = Point3::new(0.0, 0.0, 0.0);

		// plug them into another constructor
		Self::new_directional(location, direction, 0.0)
	}

	/// Creates a new camera that is aiming in a direction.
	///
	/// * `location` - the coordinates this camera is at
	/// * `direction` - the vector for the direction this camera is aiming
	/// * `roll` - the clockwise roll of the camera (in radians) from being level
	pub fn new_directional(location: Point3<f32>, direction: Vector3<f32>, roll: f32) -> Camera {
		let target: Point3<f32> = location + direction;
		let up = get_up_vector_from_angle(&direction, roll);
		let matrix = Matrix4::look_at_rh(&location, &target, &up);

		// Return the new camera
		Camera {
			location: location,
			matrix: matrix,
			roll: roll,
			target: target
		}
	}

	/// Creates a new camera that is aiming at a specific target.
	///
	/// * `location` - the coordinates this camera is at
	/// * `target` - the target this camera is aiming at
	/// * `roll` - the clockwise roll of the camera (in radians) from being level
	pub fn new_targeted(location: Point3<f32>, target: Point3<f32>, roll: f32) -> Camera {
		// Get vector from location to target because apparently this linear
		// algebra library doesn't want to do that.
		let direction = Vector3::new(target.x - location.x, target.y - location.y, target.z - location.z);

		let up = get_up_vector_from_angle(&direction, roll);
		let matrix = Matrix4::look_at_rh(&location, &target, &up);

		// Return the new camera
		Camera {
			location: location,
			matrix: matrix,
			roll: roll,
			target: target
		}
	}

	/// Teleports the camera to a new location with the same direction as before
	///
	/// * `new_location` - the location to teleport the camera to
	pub fn teleport_keep_direction(&mut self, new_location: Point3<f32>) {
		let direction = Vector3::new(self.target.x - self.location.x, self.target.y - self.location.y, self.target.z - self.location.z);

		// Generate data for new position
		let new = Self::new_directional(new_location, direction, self.roll);

		// Assign to current camera
		self.location = new.location;
		self.matrix = new.matrix;
		self.target = new.target;
	}

	/// Teleports the camera to a new location with the same target as before
	///
	/// * `new_location` - the location to teleport the camera to
	pub fn teleport_keep_target(&mut self, new_location: Point3<f32>) {
		// Generate data for new position
		let new = Self::new_targeted(new_location, self.target, self.roll);

		// Assign to current camera
		self.location = new.location;
		self.matrix = new.matrix;
	}

	/// Rotates the camera by a certain pitch, yaw, and roll
	///
	/// Not yet implemented because I don't want to deal with the math yet
	///
	/// * `pitch` - value to pitch the camera by
	/// * `yaw` - the value to yaw by
	/// * `roll` - value to roll
	pub fn turn(&mut self, pitch: f32, yaw: f32, roll: f32) {
		(_,_,_) = (pitch, yaw, roll);
		todo!();
	}
}

/// Gets the camera up vector from a direction vector and roll value
///
/// For now, since I don't feel like dealing with math, the up vector is always
/// the same as world up.
///
/// * `direction` - the direction the camera is pointing
/// * `roll` - the amount (in radians) the camera has rolled clockwise from world up
fn get_up_vector_from_angle(direction: &Vector3<f32>, roll: f32) -> Vector3<f32> {
	// Make rustc happy with the unused code
	let (_,_) = (direction, roll);
	// Give BS value
	Vector3::new(0.0, 1.0, 0.0)
}
