pub mod game;
pub mod input;
pub mod controller;
pub mod view;

extern crate sdl2;

use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
//use sdl2::pixels::Color;
//use std::time::Duration;
use sdl2::keyboard::Keycode;

//use crate::game::GameObject;
//use crate::game::Renderer as Renderer;
//use crate::game::RendererRect as RendererRect;
use crate::game::Stage;
use crate::input::Input;

use crate::controller::Controller;
use crate::controller::MainController;

use crate::view::MainView;

fn main() {
	let sdl = sdl2::init().unwrap();
	let vss: sdl2::VideoSubsystem = sdl.video().unwrap();
	let wb = sdl2::video::WindowBuilder::new(
		& vss,
		"hello",
		800,
		600
	);

	let window: sdl2::video::Window = wb.build().unwrap();
	let cb = sdl2::render::CanvasBuilder::new(window);
	let mut stage: Stage = Stage::new(800, 600, cb.build().unwrap());

	//let renderer_rect = RendererRect {};
	let mut main_controller = MainController {};
	let controller: & mut dyn Controller = & mut main_controller;
	
	let mut view = MainView::new(& mut stage);

	let mut evt_pump = sdl.event_pump().unwrap();
	let mut input = Input::new();
	let mut running = true;

	while running {
		let evt_option = evt_pump.poll_event();
		if evt_option != None { 
			let evt = evt_option.unwrap();
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { keycode: Some(k), .. } => {
					if k == Keycode::Left {
						input.move_left();
					} else if k == Keycode::Right {
						input.move_right();
					} else if k == Keycode::Space {
						input.stop();
					}
				},
				_ => ()
			}

			if controller.update(& mut stage, & mut view, & input) {
				stage.draw();
			}
		}
	}
}
