use sdl2::keyboard::Keycode;

#[derive (Copy, Clone)]
pub enum InputEvent {
	Empty,
	MoveLeft,
	MoveRight,
	Stop,
	Jump,
	ItemPrev,
	ItemNext,
	ItemSelect,
	Continue
}

pub trait Input {
	fn set_event(& mut self, k: Keycode) -> bool;
	fn get_event(& self) -> InputEvent;
	fn clear(& mut self);
}

pub struct InputTitle {
	evt: InputEvent
}

pub struct InputMenu {
	evt: InputEvent
}

pub struct InputMain {
	evt: InputEvent
}

pub struct InputGameOver {
	evt: InputEvent
}

impl InputTitle {
	pub fn new() -> Self {
		Self {
			evt: InputEvent::Empty
		}
	}
}

impl InputMenu {
	pub fn new() -> Self {
		Self {
			evt: InputEvent::Empty
		}
	}
}

impl InputMain {
	pub fn new() -> Self {
		Self {
			evt: InputEvent::Empty,
		}
	}
}

impl InputGameOver {
	pub fn new() -> Self {
		Self {
			evt: InputEvent::Empty,
		}
	}
}

impl Input for InputTitle {
	fn set_event(& mut self, k: Keycode) -> bool {
		if k == Keycode::Space || k == Keycode::Return {
			self.evt = InputEvent::Continue;
			return true;
		}

		false
	}

	fn clear(& mut self) {
		self.evt = InputEvent::Empty;
	}

	fn get_event(& self) -> InputEvent {
		self.evt
	}
}

impl Input for InputMenu {
	fn set_event(& mut self, k: Keycode) -> bool {
		if k == Keycode::Left {
			self.evt = InputEvent::ItemPrev;
		} else if k == Keycode::Right {
			self.evt = InputEvent::ItemNext;
		} else if k == Keycode::Space || k == Keycode::Return {
			self.evt = InputEvent::ItemSelect;
		} else {
			self.evt = InputEvent::Empty;
			return false;
		}

		true
	}

	fn clear(& mut self) {
		self.evt = InputEvent::Empty;
	}

	fn get_event(& self) -> InputEvent {
		self.evt
	}
}

impl Input for InputMain {
	fn set_event(& mut self, k: Keycode) -> bool {
		if k == Keycode::Left {
			self.evt = InputEvent::MoveLeft;
		} else if k == Keycode::Right {
			self.evt = InputEvent::MoveRight;
		} else {
			self.evt = InputEvent::Stop;
		}

		true
	}

	fn clear(& mut self) {
		self.evt = InputEvent::Empty;
	}

	fn get_event(& self) -> InputEvent {
		self.evt
	}
}

impl Input for InputGameOver {
	fn set_event(& mut self, k: Keycode) -> bool {
		false
	}

	fn clear(& mut self) { }

	fn get_event(& self) -> InputEvent {
		self.evt
	}
}
