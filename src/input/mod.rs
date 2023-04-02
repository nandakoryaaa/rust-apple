pub struct Input {
	pub move_left: bool,
	pub move_right: bool
}

impl Input {
	pub fn new() -> Self {
		Self {
			move_left: false,
			move_right: false
		}
	}

	pub fn move_left(& mut self) {
		self.move_left = true;
		self.move_right = false;
	}

	pub fn move_right(& mut self) {
		self.move_right = true;
		self.move_left = false;
	}

	pub fn stop(& mut self) {
		self.move_left = false;
		self.move_right = false;
	}

	pub fn reset(& mut self) {
		self.stop();
	}
}

		