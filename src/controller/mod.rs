use crate::game::{ GameStateEvent, PlayerAnimationState };
use crate::input::{ Input, InputEvent };
use crate::model::{ Model };

pub trait Controller {
	fn update(& mut self, model: & mut Model, input: & dyn Input) -> GameStateEvent;
}

pub struct ControllerTitle {
}

pub struct ControllerMain {
}

pub struct ControllerMenu {
}

impl Controller for ControllerTitle {
	fn update(
		& mut self, model: & mut Model, input: & dyn Input
	) -> GameStateEvent {
		let evt: InputEvent = input.get_event();
		match evt {
			InputEvent::Continue => {
				return GameStateEvent::RunMenu;
			},
			_ => ()
		}

		GameStateEvent::Empty
	}
}

impl Controller for ControllerMenu {
	fn update(
		& mut self, model: & mut Model, input: & dyn Input
	) -> GameStateEvent {
		let mut updated = GameStateEvent::Empty;
		let evt: InputEvent = input.get_event();
		match model {
			Model::ModelMenu { level, .. } => {
				match evt {
					InputEvent::ItemPrev => {
						if *level > 0 {
							*level -= 1;
						}
						updated = GameStateEvent::Run;
					}
					InputEvent::ItemNext => {
						if *level < 8 {
							*level += 1;
						}
						updated = GameStateEvent::Run;
					}
					InputEvent::ItemSelect => {
						updated = GameStateEvent::RunMain;
					},
					_ => ()
				}
			},
			_ => ()
		}

		return updated;
	}
}

impl Controller for ControllerMain {
	fn update(
		& mut self, model: & mut Model, input: & dyn Input
	) -> GameStateEvent {
		let mut updated = GameStateEvent::Run;
		let evt: InputEvent = input.get_event();
		match model {
			Model::ModelMain { player_x, player_state, grid_w, .. } => {
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
					_ => { updated = GameStateEvent::Empty; }
				}
			},
			_ => { updated = GameStateEvent::Empty; }
		}

		return updated;
	}
}
