use crate::game::PlayerAnimationState;
use crate::data::TITLE;

pub struct ModelFactory {
}

impl ModelFactory {
	pub fn ModelTitle() -> Model {
		Model::ModelTitle {
			logo_w: 29,
			logo_h: 15,
			logo_pattern: & TITLE
		}
	}

	pub fn ModelMenu() -> Model {
		Model::ModelMenu {
			level: 0
		}
	}

	pub fn ModelMain() -> Model {
		Model::ModelMain {
			grid_w: 256,
			grid_h: 256,
			player_x: 112,
			player_y: 200,
			player_state: PlayerAnimationState::Stand,
			player_frame: 0,
			apple_x: 0,
			apple_y: 0,
			apples_collected: 0,
			apples_left: 0,
			apples_lost: 0
		}
	}
}

pub enum Model {
	ModelTitle {
		logo_w: u32,
		logo_h: u32,
		logo_pattern: &'static [u8]
	},

	ModelMenu {
		level: u32
	},

	ModelMain {
		grid_w: u32,
		grid_h: u32,
		player_x: i32,
		player_y: i32,
		player_state: PlayerAnimationState,
		player_frame: u32,
		apple_x: i32,
		apple_y: i32,
		apples_collected: u32,
		apples_left: u32,
		apples_lost: u32
	}
}
