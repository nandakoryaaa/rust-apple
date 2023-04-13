extern crate sdl2;
use sdl2::pixels::Color;
use crate::game::{ GMO, Stage };
use crate::factory::GmoFactory;
use crate::model::Model;

pub trait View {
	fn update(& self, stage: & mut Stage, model: & Model);
}

pub struct MainView {
	pub factory: &'static GmoFactory
}

impl View for MainView {
	fn update(& self, stage: & mut Stage, model: & Model) {
		let plr = stage.get_child(0);
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

impl MainView {
	pub fn new(stage: & mut Stage, gmo_factory: &'static GmoFactory) -> Self {
		stage.add_child(gmo_factory.create_player());
		stage.add_child(
			gmo_factory.create_text(120, 440, Color::RGB(0, 255, 0), & "GMO APPLE")
		);
		Self {
			factory: gmo_factory
		}
	}

}