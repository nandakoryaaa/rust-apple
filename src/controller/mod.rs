use crate::game::{ Stage, PlayerAnimationState };
use crate::input::{ Input, InputEvent };
use crate::view::{ View, MainView };
use crate::model::{ Model };

pub trait Controller {
	fn update(& mut self, input: & dyn Input) -> bool;

	fn get_model(& self) -> & Model;
}

pub struct MainController {
	pub model: Model
}

pub struct MenuController {
	pub model: Model
}

impl Controller for MenuController {
	fn get_model(& self) -> & Model {
		& self.model
	}

	fn update(
		& mut self, input: & dyn Input
	) -> bool {
		let mut updated = false;
		let evt: InputEvent = input.get_event();
		let mut m = & mut self.model;
		match m {
			Model::MenuModel { level, .. } => {
				match evt {
					InputEvent::ItemPrev => {
						if *level > 0 {
							*level -= 1;
						}
						updated = true;
					}
					InputEvent::ItemNext => {
						if *level < 9 {
							*level += 1;
						}
						updated = true;
					}
					InputEvent::ItemSelect => {
						updated = true;
					},
					_ => ()
				}
			},
			_ => ()
		}

		return updated;
	}
}

impl Controller for MainController {
	fn get_model(& self) -> & Model {
		& self.model
	}

	fn update(
		& mut self, input: & dyn Input
	) -> bool {
		let mut updated = true;
		let evt: InputEvent = input.get_event();
		let mut m = & mut self.model;
		match m {
			Model::MainModel { player_x, player_state, grid_w, .. } => {
				match evt {
					InputEvent::MoveLeft => {
						if *player_state as u32 != PlayerAnimationState::MoveLeft as u32 {
							*player_state = PlayerAnimationState::MoveLeft;
						}
						if *player_x > 0 {
							*player_x -= 1;
						}
					}
					InputEvent::MoveRight => {
						if *player_state as u32 != PlayerAnimationState::MoveRight as u32 {
							*player_state = PlayerAnimationState::MoveRight;
						}
						if *player_x < *grid_w as i32 {
							*player_x += 1;
						}
					}
					InputEvent::Stop => {
						if *player_state as u32 != PlayerAnimationState::Stand as u32 {
							*player_state = PlayerAnimationState::Stand;
						}
					},
					_ => { updated = false; }
				}
			},
			_ => { updated = false; }
		}

		return updated;
	}
}
