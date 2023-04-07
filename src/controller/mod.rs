use crate::game::Stage;
use crate::input::Input;
use crate::input::InputEvent;
use crate::view::MainView;

pub trait Controller {
	fn update(
		& mut self, stage: & mut Stage,
		view: & mut MainView, input: & dyn Input
	) -> bool;
}

pub struct MainController {
	pub player_x: i32,
	pub player_w: u32
}

impl Controller for MainController {
	fn update(
		& mut self, stage: & mut Stage,
		view: & mut MainView, input: & dyn Input
	) -> bool {
		let mut updated = false;
		let evt: InputEvent = input.get_event();
		match evt {
			InputEvent::Empty => { },
			_ => {
				// Позволяет взять sw, так как player еще не используется.
				let sw = stage.w as i32;

				// get_child() возвращает & mut GameObject
				// GameОbject находится в stage.obj_list, мы получаем лок на & mut GameObject,
				// поэтому лок распространяется и на stage
				// лок должен исчезнуть, когда исчезнет player

				match evt {
					InputEvent::MoveLeft => {
						if self.player_x > 0 {
							self.player_x -= 1;
							updated = true;
						}
					}
					InputEvent::MoveRight => {
						if self.player_x < sw - self.player_w as i32 {
							self.player_x += 1;
							updated = true;
						}
					}
					_ => ()
				}
			}
		}
		if updated {
			let player = view.get_player(stage); //stage.get_child(0); //
			//player.rect.x = self.player_x;
		}

		return updated;
	}
}
