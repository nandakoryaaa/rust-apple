//use sdl2::event::Event;
use sdl2::keyboard::Keycode;
#[derive (Copy, Clone)]
pub enum InputEvent {
	Empty,
	MoveLeft,
	MoveRight,
	Stop,
	Jump,
	ItempPrev,
	ItemNext,
	ItemSelect
}

pub trait Input {
	fn set_event(& mut self, k: Keycode) -> bool;
	fn get_event(& self) -> InputEvent;
}

pub struct InputMain {
	evt: InputEvent,
	move_left: bool,
	move_right: bool,
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

	fn get_event(& self) -> InputEvent {
		self.evt
	}
}
