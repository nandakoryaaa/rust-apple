pub mod game;
pub mod input;
pub mod controller;
pub mod view;
pub mod data;
pub mod model;
pub mod factory;
pub mod render;

extern crate sdl2;
use sdl2::event::Event;

use crate::render::{ RendererRect, RendererSpriteRLE, RendererText, Sprite, SpriteSequence };
use crate::factory::{ GmoFactory };
use crate::game::{ Stage, PlayerAnimationState };
use crate::input::{ Input, InputMenu, InputMain };
use crate::controller::{ Controller, ControllerMenu, ControllerMain };

use crate::view::{ View, ViewMenu, ViewMain };
use crate::data::{ SPRITE_APPLE, PALETTE, FONT };
use crate::data as cd; //::{ SPRITE_PLAYER_0,  SPRITE_PLAYER_2,  SPRITE_PLAYER_2,  SPRITE_PLAYER_3, SPRITE_PLAYER_4, SPRITE_PLAYER_5,;

use crate::model::Model;

static gmo_factory: GmoFactory = GmoFactory {
	sp_apple: Sprite { w: 8, h: 10, data: & SPRITE_APPLE },
	sq_player_stand: SpriteSequence { frame_cnt: 1, frames: & [& Sprite { w: 22, h: 30, data: & cd::SPRITE_PLAYER_0 }] },
	sq_player_stand_l: SpriteSequence { frame_cnt: 1, frames: & [& Sprite { w: 32, h: 30, data: & cd::SPRITE_PLAYER_L_0 }] },
	sq_player_stand_r: SpriteSequence { frame_cnt: 1, frames: & [& Sprite { w: 32, h: 30, data: & cd::SPRITE_PLAYER_R_0 }] },
	sq_player_move_l: SpriteSequence {
		frame_cnt: 2,
		frames: & [& Sprite { w: 32, h: 30, data: & cd::SPRITE_PLAYER_L_1 }, & Sprite { w: 32, h: 30, data: & cd::SPRITE_PLAYER_L_2 }]
	},
	sq_player_move_r: SpriteSequence {
		frame_cnt: 2,
		frames: & [& Sprite { w: 32, h: 30, data: & cd::SPRITE_PLAYER_R_1 }, & Sprite { w: 32, h: 30, data: & cd::SPRITE_PLAYER_R_2 }]
	},
	sq_player_death: SpriteSequence {
		frame_cnt: 2,
		frames: & [
			& Sprite { w: 22, h: 30, data: & cd::SPRITE_PLAYER_1 },
			& Sprite { w: 22, h: 30, data: & cd::SPRITE_PLAYER_2 },
			& Sprite { w: 23, h: 30, data: & cd::SPRITE_PLAYER_3 },
			& Sprite { w: 23, h: 30, data: & cd::SPRITE_PLAYER_4 },
			& Sprite { w: 24, h: 30, data: & cd::SPRITE_PLAYER_5 },
			& Sprite { w: 24, h: 30, data: & cd::SPRITE_PLAYER_6 },
		]
	},
	renderer_rect: RendererRect {},
	renderer_text: RendererText {
		pixel_width: 1,
		pixel_height: 1,
		font: & FONT
	},
	renderer_sprite_rle: RendererSpriteRLE {
		palette: & PALETTE,
		pixel_width: 1,
		pixel_height: 1
	}, 
};

fn main() {
	let window_width:u32 = 1024;
	let window_height:u32 = 768;

	let sdl = sdl2::init().unwrap();
	let vss: sdl2::VideoSubsystem = sdl.video().unwrap();
	let wb = sdl2::video::WindowBuilder::new(
		& vss,
		"GMO APPLE",
		window_width,
		window_height
	);

	let window: sdl2::video::Window = wb.build().unwrap();
	let cb = sdl2::render::CanvasBuilder::new(window);
	let mut canvas = cb.build().unwrap();

	let mut stage: Stage = Stage::new(window_width, window_height, 256, 256);
//	gmo_factory.renderer_sprite_rle.pixel_width = stage.pixel_width as i32;
//	gmo_factory.renderer_sprite_rle.pixel_height = stage.pixel_height as i32;

/*	let mut controller_main = ControllerMain {
		model: Model::MainModel {
			grid_w: 28,
			grid_h: 10,
			player_x: 5,
			player_y: 10,
			player_state: PlayerAnimationState::Stand,
			player_frame: 0,
			apple_x: 0,
			apple_y: 0,
			apples_collected: 0,
			apples_left: 10,
			apples_lost: 0
		}
	};
*/
	let mut controller_menu = ControllerMenu {
		model: Model::MenuModel {
			level: 0
		}
	};

//	let mut input_main = InputMain::new();
	let mut input_menu = InputMenu::new();

//	let mut view_main = ViewMain::new(& mut stage, & gmo_factory);
	let mut view_menu = ViewMenu::new(& mut stage, & gmo_factory);

	let controller: & mut dyn Controller = & mut controller_menu;
	let view: & mut dyn View = & mut view_menu;
	let input: & mut dyn Input = & mut input_menu;

	let mut evt_pump = sdl.event_pump().unwrap();
	let mut running = true;

	stage.draw(& mut canvas);

	while running {
		let evt_option = evt_pump.poll_event();
		if evt_option != None { 
			let evt = evt_option.unwrap();
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { keycode: Some(k), .. } => {
					input.set_event(k);
				},
				_ => ()
			}

			if controller.update(input) {
				input.clear();
				view.update(& mut stage, & controller.get_model());
				stage.draw(& mut canvas);
			}
		}
	}
}
