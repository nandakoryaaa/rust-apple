extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::game::{ GMO, Stage };
use crate::factory::GmoFactory;
use crate::model::Model;

pub trait View {
	fn update(& self, stage: & mut Stage, model: & Model);
}

pub struct ViewMain {
	pub factory: &'static GmoFactory,
	player_id: usize
}

pub struct ViewMenu {
	pub factory: &'static GmoFactory,
	selector_x:i32,
	item_id: usize
}

impl ViewMenu {
	pub fn new(stage: & mut Stage, gmo_factory: &'static GmoFactory) -> Self {
		stage.add_child(
			gmo_factory.create_text(60, 100, Color::RGB(0, 255, 0), & "SELECT DIFFICULTY")
		);
		stage.add_child(
			gmo_factory.create_text(52, 140, Color::RGB(0, 255, 0), & "EASY")
		);
		stage.add_child(
			gmo_factory.create_text(172, 140, Color::RGB(0, 255, 0), & "HARD")
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
					renderer: & gmo_factory.renderer_rect
				}
			);
		}

		let id = stage.add_child(gmo_factory.create_apple(sel_x, 138));
		Self {
			factory: gmo_factory,
			item_id: id,
			selector_x: sel_x
		}
	}

}

impl ViewMain {
	pub fn new(stage: & mut Stage, gmo_factory: &'static GmoFactory) -> Self {
//		let id = stage.add_child(gmo_factory.create_player());
//		stage.add_child(
//			gmo_factory.create_text(120, 440, Color::RGB(0, 255, 0), & "GMO APPLE")
//		);
		Self {
			factory: gmo_factory,
			player_id: 0
		}
	}

}

impl View for ViewMenu {
	fn update(& self, stage: & mut Stage, model: & Model) {
		let item = stage.get_child(self.item_id);
		match item {
			GMO::GmoSprite { x, .. } => {
				match *model {
					Model::MenuModel { level, .. } => {
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
	fn update(& self, stage: & mut Stage, model: & Model) {
		let plr = stage.get_child(self.player_id);
		match plr {
			GMO::GmoSpriteAnimated { x, state, frame, sequence, .. } => {
				match *model {
					Model::MainModel { player_state, player_x, .. } => {
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

