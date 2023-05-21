extern crate sdl2;

use sdl2::pixels::Color;

use crate::game::{ GMO, Stage, PlayerAnimationState };
use crate::factory::GmoFactory;
use crate::model::Model;

pub trait View {
	fn init(& mut self, stage: & mut Stage, model: & Model);
	fn update(& self, stage: & mut Stage, model: & Model);
	fn clear(& mut self, stage: & mut Stage);
}

pub struct ViewTitle {
	pub factory: &'static GmoFactory
}

pub struct ViewMenu {
	pub factory: &'static GmoFactory,
	selector_x:i32,
	item_id: usize
}

pub struct ViewMain {
	pub factory: &'static GmoFactory,
	player_id: usize,
	apple_id: usize,
	num_collected_id: usize,
	num_lost_id: usize,
	grid_step_x: i32,
	grid_step_y: i32,
}

pub struct ViewGameOver {
	pub factory: &'static GmoFactory,
	player_id: usize,
	grid_step_x: i32,
	grid_step_y: i32,
}


impl ViewTitle {
	pub fn new(gmo_factory: &'static GmoFactory) -> Self {
		Self {
			factory: gmo_factory
		}
	}
}

impl ViewMenu {
	pub fn new(gmo_factory: &'static GmoFactory) -> Self {
		Self {
			factory: gmo_factory,
			item_id: 0,
			selector_x: 0
		}
	}
}

impl ViewMain {
	pub fn new(gmo_factory: &'static GmoFactory) -> Self {
		Self {
			factory: gmo_factory,
			player_id: 0,
			apple_id: 0,
			num_collected_id: 0,
			num_lost_id: 0,
			grid_step_x: 8,
			grid_step_y: 6
		}
	}

	fn update_gmo_number(& self, stage: & mut Stage, idx: usize, num: i32) {
		let gmo = stage.get_child(idx);
		match gmo {
			GMO::GmoNumber { number, .. } => {
				*number = num;
			},
			_ => ()
		}
	}
}

impl ViewGameOver {
	pub fn new(gmo_factory: &'static GmoFactory) -> Self {
		Self {
			factory: gmo_factory,
			player_id: 0,
			grid_step_x: 8,
			grid_step_y: 6
		}
	}
}

impl View for ViewTitle {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
		match *model {
			Model::ModelTitle { logo_w, logo_pattern, .. } => {
				stage.clear();
				let l = logo_pattern.len();
				let mut y:u32 = 0;
				let mut x:u32 = 0;
				let mut pos:usize = 0;
				while pos < l {
					if logo_pattern[pos] != 0 {
						stage.add_child(self.factory.create_apple((x * 8 + 12) as i32, (y * 10 + 16) as i32));
					}
					pos += 1;
					x += 1;
					if x == logo_w {
						x = 0;
						y += 1;
					}
				}
				stage.add_child(
					self.factory.create_text(20, 190, Color::RGB(255, 0, 0), & "ORIGINAL BK-0010 GAME BY FK")
				);
				stage.add_child(
					self.factory.create_text(40, 200, Color::RGB(0, 255, 0), & "2023 GMO REMAKE BY ZDG")
				);
				stage.add_child(
					self.factory.create_text(84, 224, Color::RGB(255, 0, 0), & "PRESS SPACE")
				);

			},
			_ => ()
		}
	}

	fn update(& self, _stage: & mut Stage, _model: & Model) {
	}

	fn clear(& mut self, stage: & mut Stage) {
		stage.clear();
	}
}

impl View for ViewMenu {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
		stage.clear();
		stage.add_child(
			self.factory.create_text(60, 100, Color::RGB(0, 255, 0), & "SELECT DIFFICULTY")
		);
		stage.add_child(
			self.factory.create_text(52, 140, Color::RGB(0, 255, 0), & "EASY")
		);
		stage.add_child(
			self.factory.create_text(172, 140, Color::RGB(0, 255, 0), & "HARD")
		);
		let sel_x: i32 = 91;
		for i in 0..9 {
			stage.add_child(
				self.factory.create_rect(sel_x + 3 + i * 8,	143, 2, 2, Color::RGB(0, 255, 0), stage.pixel_width, stage.pixel_height)
			);
		}

		self.item_id = stage.add_child(self.factory.create_apple(sel_x, 138));
		self.selector_x = sel_x;
		self.update(stage, model)
	}

	fn update(& self, stage: & mut Stage, model: & Model) {
		let item = stage.get_child(self.item_id);
		match item {
			GMO::GmoSprite { x, .. } => {
				match *model {
					Model::ModelMenu { level, .. } => {
						*x = self.selector_x + 8 * level as i32;
					},
					_ => ()
				}
			},
			_ => ()
		}
	}

	fn clear(& mut self, stage: & mut Stage) {
		stage.clear();
	}
}

impl View for ViewMain {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
		match model {
			Model::ModelMain { data } => {
				self.player_id = stage.add_child(
					GMO::GmoSpriteAnimated {
						x: data.player_x * self.grid_step_x,
						y: data.grid_h as i32 * self.grid_step_y + 7,
						w: 24,
						h: 32,
						state: PlayerAnimationState::Stand,
						looped: true,
						frame: 0,
						sequence: & self.factory.sq_player_stand,
						renderer: & self.factory.renderer_sprite_rle
					}
				);
				for i in 0..28 {
					stage.add_child(self.factory.create_apple((16 + i * self.grid_step_x) as i32, 16));
				}
				stage.add_child(self.factory.create_rect(0, 217, stage.w, 1, Color::RGB(0, 255, 0), stage.pixel_width, stage.pixel_height));
				stage.add_child(self.factory.create_text(8, 233, Color::RGB(0, 255, 0), & "APPLES"));
				self.num_collected_id = stage.add_child(self.factory.create_number(64, 233, Color::RGB(0, 255, 0), 0, 3));
				//stage.add_child(self.factory.create_text(96, 233, Color::RGB(0, 255, 0), & "STRIKE!"));
				stage.add_child(self.factory.create_text(160, 233, Color::RGB(0, 255, 0), & "DROPPED"));
				self.num_lost_id = stage.add_child(self.factory.create_number(224, 233, Color::RGB(0, 255, 0), 0, 3));
				self.apple_id = stage.add_child(self.factory.create_apple(16, 16));
			},
			_ => ()
		}
	}

	fn update(& self, stage: & mut Stage, model: & Model) {
		match model {
			Model::ModelMain { data } => {
				self.update_gmo_number(stage, self.num_collected_id, data.apples_collected as i32);
				self.update_gmo_number(stage, self.num_lost_id, data.apples_lost as i32);
				let plr = stage.get_child(self.player_id);
				match plr {
					GMO::GmoSpriteAnimated { x, state, frame, sequence, .. } => {
						*x = data.player_x * self.grid_step_x;
						if *state as u32 != data.player_state as u32 {
							*state = data.player_state;
							*sequence = self.factory.get_state(data.player_state);
							*frame = 0;
						}
						plr.update();
					},
					_ => ()
				}

				let apple = stage.get_child(self.apple_id);
				match apple {
					GMO::GmoSprite { x, y, .. } => {
						*x = data.apple_x * self.grid_step_x;
						*y = 26 + data.apple_y * self.grid_step_y;
					},
					_ => ()
					
				}
			},
			_ => ()
		}
	}

	fn clear(& mut self, stage: & mut Stage) {
		stage.remove_child(self.apple_id);
		stage.remove_child(self.player_id);
	}
}

impl View for ViewGameOver {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
		match model {
			Model::ModelMain { data } => {
				self.player_id = stage.add_child(
					GMO::GmoSpriteAnimated {
						x: data.player_x * self.grid_step_x,
						y: data.grid_h as i32 * self.grid_step_y + 7,
						w: 24,
						h: 32,
						state: PlayerAnimationState::Death,
						looped: false,
						frame: 0,
						sequence: & self.factory.sq_player_death,
						renderer: & self.factory.renderer_sprite_rle
					}
				);
			},
			_ => ()
		}
	}

	fn update(& self, stage: & mut Stage, model: & Model) {
		let plr = stage.get_child(self.player_id);
		match plr {
			GMO::GmoSpriteAnimated { .. } => {
				plr.update();
			},
			_ => ()
		}
	}

	fn clear(& mut self, stage: & mut Stage) {
		stage.clear();
	}
}
