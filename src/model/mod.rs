use crate::game::PlayerAnimationState;
use crate::data::TITLE;

pub trait ModelTransfer {
	fn transfer(& mut self, from: & Model);
}

pub struct ModelDataMain {
	pub level: i32,
	pub grid_w: u32,
	pub grid_h: u32,
	pub player_x: i32,
	pub player_state: PlayerAnimationState,
	pub apple_x: i32,
	pub apple_y: i32,
	pub apples_collected: u32,
	pub apples_left: u32,
	pub apples_lost: u32
}

impl ModelTransfer for ModelDataMain {
	fn transfer(& mut self, from: & Model) {
		match *from {
			Model::ModelMenu { level } => {
				self.level = level;
			},
			_ => ()
		}
	}
}

pub enum Model {
	ModelTitle {
		logo_w: u32,
		logo_pattern: &'static [u8]
	},

	ModelMenu {
		level: i32
	},

	ModelMain {
		data: ModelDataMain,
	}
}

pub struct ModelFactory {
}

impl ModelFactory {
	pub fn model_title() -> Model {
		Model::ModelTitle {
			logo_w: 29,
			logo_pattern: & TITLE
		}
	}

	pub fn model_menu() -> Model {
		Model::ModelMenu {
			level: 4
		}
	}

	pub fn model_main(from_model: & Model) -> Model {
		let mut data = ModelDataMain {
			level: 0,
			grid_w: 32,
			grid_h: 30,
			player_x: 16,
			player_state: PlayerAnimationState::Stand,
			apple_x: 0,
			apple_y: 0,
			apples_collected: 0,
			apples_left: 100,
			apples_lost: 0
		};

		data.transfer(from_model);

		Model::ModelMain {
			data: data
		}
	}
}
