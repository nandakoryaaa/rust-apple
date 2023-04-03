use crate::game::Stage;
use crate::input::Input;
use crate::view::MainView;

pub trait Controller {
	fn update(
		& mut self, stage: & mut Stage,
		view: & mut MainView, input: & Input
	) -> bool;
}

pub struct MainController {
	pub player_x: i32,
	pub player_w: u32
}

impl Controller for MainController {
	fn update(
		& mut self, stage: & mut Stage,
		view: & mut MainView, input: & Input
	) -> bool {
		let mut updated = false;
		if input.move_left || input.move_right {
			// Позволяет взять sw, так как player еще не используется.
			let sw = stage.w as i32;

			// get_child() возвращает & mut GameObject
			// GameОbject находится в stage.obj_list, мы получаем лок на & mut GameObject,
			// поэтому лок распространяется и на stage
			// лок должен исчезнуть, когда исчезнет player

			if input.move_left {
				if self.player_x > 0 {
					self.player_x -= 1;
					updated = true;
				}
			} else if input.move_right {
				if self.player_x < sw - self.player_w as i32 {
					self.player_x += 1;
					updated = true;
				}
			}

			if updated {
				let player = view.get_player(stage); //stage.get_child(0); //
				player.rect.x = self.player_x;
			}
		}

		return updated;
	}
}
