pub mod game;
pub mod input;
pub mod controller;
pub mod view;
pub mod data;

extern crate sdl2;

use sdl2::event::Event;
//use sdl2::keyboard::Keycode;

use crate::game::render::RendererRect;
use crate::game::render::RendererSpriteRLE;
use crate::game::render::RendererText;
use crate::game::render::RendererFactory;
use crate::game::Stage;
use crate::input::Input;
use crate::input::InputMain;

use crate::controller::Controller;
use crate::controller::MainController;

use crate::view::MainView;
use crate::data::Sprites;
use crate::data::SPRITE_APPLE;
use crate::data::PALETTE;
use crate::data::FONT;

fn main() {
	static renderer_factory: RendererFactory = RendererFactory {
		renderer_rect: RendererRect {},
		renderer_sprite_rle: RendererSpriteRLE {
			palette: & PALETTE,
			pixel_width: 16,
			pixel_height: 12
		},
		renderer_text: RendererText {
			font: & FONT
		},
		sprites: Sprites {
			apple: & SPRITE_APPLE
		}
	};

	let sdl = sdl2::init().unwrap();
	let vss: sdl2::VideoSubsystem = sdl.video().unwrap();
	let wb = sdl2::video::WindowBuilder::new(
		& vss,
		"APPLE",
		800,
		600
	);

	let window: sdl2::video::Window = wb.build().unwrap();
	let cb = sdl2::render::CanvasBuilder::new(window);
	let canvas = cb.build().unwrap();

	let mut stage: Stage = Stage::new(800, 600, canvas);

	let mut main_controller = MainController { player_x: 0, player_w: 16};
	let controller: & mut dyn Controller = & mut main_controller;

	let mut view = MainView::new(& mut stage, & renderer_factory);

	let mut evt_pump = sdl.event_pump().unwrap();
	let mut input_main = InputMain::new();
	let mut input: & mut dyn Input = & mut input_main;
	let mut running = true;

	stage.draw();

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

			if controller.update(& mut stage, & mut view, input) {
				stage.draw();
			}
		}
	}
}
