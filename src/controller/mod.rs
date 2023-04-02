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
}

impl Controller for MainController {
	fn update(
		& mut self, stage: & mut Stage,
		view: & mut MainView, input: & Input
	) -> bool {
		if input.move_left || input.move_right {
			// Позволяет взять sw, так как player еще не используется.
			let sw = stage.w as i32;

			// get_child() возвращает & mut GameObject
			// GameОbject находится в stage.obj_list, мы получаем лок на & mut GameObject,
			// поэтому лок распространяется и на stage
			// лок должен исчезнуть, когда исчезнет player

			let player = stage.get_child(0); //view.get_player(stage); //
			if input.move_left {
				if player.rect.x > 0 {
					player.rect.x -= 1;
				}
			} else if input.move_right {
				if player.rect.x < sw - player.rect.w {
					player.rect.x += 1;
				}
			}

			// Позволяет взять sw, так как player больше не используется. Хотя он все еще в контексте, но считается, что уже нет
			// let sw = stage.w as i32;
			return true;
		}

		return false;
	}
}
