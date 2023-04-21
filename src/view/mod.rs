extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::game::{ GMO, Stage, PlayerAnimationState };
use crate::factory::GmoFactory;
use crate::model::Model;
use crate::data::TITLE;

pub trait View {
	fn init(& mut self, stage: & mut Stage, model: & Model);
	fn update(& self, stage: & mut Stage, model: & Model);
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
	player_id: usize
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
			player_id: 0
		}
	}
}

impl View for ViewTitle {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
		match *model {
			Model::ModelTitle { logo_w, logo_h, logo_pattern } => {
				let l = logo_pattern.len();
				let mut y:u32 = 0;
				let mut x:u32 = 0;
				let mut pos:usize = 0;
				while pos < l {
					if logo_pattern[pos] != 0 {
						stage.add_child(self.factory.create_apple((x * 8 + 16) as i32, (y * 8 + 16) as i32));
					}
					pos += 1;
					x += 1;
					if x == logo_w {
						x = 0;
						y += 1;
					}
				}
				stage.add_child(
					self.factory.create_text(0, 150, Color::RGB(255, 0, 0), & "ORIGINAL BK-0010 GAME BY FK")
				);
				stage.add_child(
					self.factory.create_text(0, 160, Color::RGB(0, 255, 0), & "2023 GMO REMAKE BY ZDG")
				);

			},
			_ => ()
		}
	}

	fn update(& self, stage: & mut Stage, model: & Model) {
	}
}

impl View for ViewMenu {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
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
				GMO::GmoRect {
					x: sel_x + 3 + i * 8,
					y: 143,
					w: 2,
					h: 2,
					color: Color::RGB(0, 255, 0),
					rect: Rect::new((sel_x + 3 + i * 8) * stage.pixel_width as i32, 143 * stage.pixel_height as i32, stage.pixel_width * 2, stage.pixel_height * 2),
					renderer: & self.factory.renderer_rect
				}
			);
		}

		self.item_id = stage.add_child(self.factory.create_apple(sel_x, 138));
		self.selector_x = sel_x;
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
}

impl View for ViewMain {
	fn init(& mut self, stage: & mut Stage, model: & Model) {
		match model {
			Model::ModelMain { player_x, player_y, .. } => {
				self.player_id = stage.add_child(
					GMO::GmoSpriteAnimated {
						x: *player_x,
						y: *player_y,
						w: 24,
						h: 32,
						state: PlayerAnimationState::Stand,
						frame: 0,
						delay: 6,
						sequence: & self.factory.sq_player_stand,
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
			GMO::GmoSpriteAnimated { x, state, frame, sequence, .. } => {
				match *model {
					Model::ModelMain { player_state, player_x, .. } => {
						*x = player_x;
						if (*state) as u32 != player_state as u32 {
							*state = player_state;
							*sequence = self.factory.get_state(player_state);
							*frame = 0;
						}
						plr.update();
					},
					_ => ()
				}
			},
			_ => ()
		}
	}
}
