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
	ItemSelect
}

pub trait Input {
	fn set_event(& mut self, k: Keycode) -> bool;
	fn get_event(& self) -> InputEvent;
	fn clear(& mut self);
}

pub struct InputMain {
	evt: InputEvent,
	move_left: bool,
	move_right: bool
}

pub struct InputMenu {
	evt: InputEvent,
	item_prev: bool,
	item_next: bool,
	item_select: bool
}

impl InputMenu {
	pub fn new() -> Self {
		Self {
			evt: InputEvent::Empty,
			item_prev: false,
			item_next: false,
			item_select: false
		}
	}
}

impl InputMain {
	pub fn new() -> Self {
		Self {
			evt: InputEvent::Empty,
			move_left: false,
			move_right: false
		}
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
			self.move_left = true;
			self.move_right = false;
			self.evt = InputEvent::MoveLeft;
		} else if k == Keycode::Right {
			self.move_right = true;
			self.move_left = false;
			self.evt = InputEvent::MoveRight;
		} else if k == Keycode::Space {
			self.move_left = false;
			self.move_right = false;
			self.evt = InputEvent::Stop;
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
