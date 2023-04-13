use crate::game::PlayerAnimationState;

pub enum Model {
	MenuModel {
		level: u32
	},

	MainModel {
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
