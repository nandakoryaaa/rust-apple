pub mod game;
pub mod input;
pub mod controller;
pub mod view;
pub mod data;
pub mod model;
pub mod factory;
pub mod render;
pub mod xrand;

extern crate sdl2;
use sdl2::event::Event;

use std::{thread, time};

use crate::game::{ Stage, GameStateEvent };
use crate::input::{ Input, InputTitle, InputMenu, InputMain, InputGameOver };
use crate::controller::{ PlayerAppleCollider, Controller, ControllerTitle, ControllerMenu, ControllerMain, ControllerGameOver };
use crate::view::{ View, ViewTitle, ViewMenu, ViewMain, ViewGameOver };

use crate::model::{ ModelFactory };
use crate::factory::GMO_FACTORY;
use crate::xrand::XRand;

const FPS_DELAY: i32 = 33;
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

fn main() {

	let sdl = sdl2::init().unwrap();
	let vss: sdl2::VideoSubsystem = sdl.video().unwrap();
	let wb = sdl2::video::WindowBuilder::new(
		& vss,
		"GMO APPLE",
		WINDOW_WIDTH,
		WINDOW_HEIGHT
	);

	let window: sdl2::video::Window = wb.build().unwrap();
	let cb = sdl2::render::CanvasBuilder::new(window);
	let mut canvas = cb.build().unwrap();

	let mut stage: Stage = Stage::new(WINDOW_WIDTH, WINDOW_HEIGHT, 256, 256);

	let mut controller_title = ControllerTitle { evt: GameStateEvent::Empty };
	let mut controller_menu = ControllerMenu { evt: GameStateEvent::Empty };
	let mut controller_main = ControllerMain { evt: GameStateEvent::Empty, rand: XRand::new(), tick: 0, player_dir: 0, player_step: 0, collider: PlayerAppleCollider {} };
	let mut controller_game_over = ControllerGameOver { phase: 0, tick: 0 };

	let mut view_title = ViewTitle::new(& GMO_FACTORY);
	let mut view_menu = ViewMenu::new(& GMO_FACTORY);
	let mut view_main = ViewMain::new(& GMO_FACTORY);
	let mut view_game_over = ViewGameOver::new(& GMO_FACTORY);

	let mut input_title = InputTitle::new();
	let mut input_menu = InputMenu::new();
	let mut input_main = InputMain::new();
	let mut input_game_over = InputGameOver::new();

	let mut controller: & mut dyn Controller = & mut controller_title;
	let mut view: & mut dyn View = & mut view_title;
	let mut input: & mut dyn Input = & mut input_title;
	let mut model = ModelFactory::model_title();

	view.init(& mut stage, & model);
	stage.draw(& mut canvas);

	let mut evt_pump = sdl.event_pump().unwrap();
	let timer = sdl.timer().unwrap();
	let mut running = true;
	let mut next_tick: i32 = timer.ticks() as i32 + FPS_DELAY;
	let mut controller_evt: GameStateEvent = GameStateEvent::Empty;

	while running {
		let mut has_event = false;
		for evt in evt_pump.poll_iter() {
			match evt {
				Event::Quit { .. } => {
					running = false;
				},
				Event::KeyDown { keycode: Some(k), .. } => {
					has_event = input.set_event(k);
				},
				_ => ()
			}
		}

		if has_event {
			controller.update_input(& mut model, input);
			input.clear();
		}

		controller_evt = controller.update(& mut model);

		match controller_evt {
			GameStateEvent::Run => {
				view.update(& mut stage, & model);
				stage.draw(& mut canvas);
			},
			GameStateEvent::RunMenu => {
				view.clear(& mut stage);
				input.clear();
				controller = & mut controller_menu;
				view = & mut view_menu;
				input = & mut input_menu;
				model = ModelFactory::model_menu();
				view.init(& mut stage, & model);
				stage.draw(& mut canvas);
			},
			GameStateEvent::RunMain => {
				view.clear(& mut stage);
				input.clear();
				controller = & mut controller_main;
				view = & mut view_main;
				input = & mut input_main;
				model = ModelFactory::model_main(& model);
				view.init(& mut stage, & model);
				stage.draw(& mut canvas);
			},
			GameStateEvent::RunGameOver => {
				controller = & mut controller_game_over;
				view.clear(& mut stage);
				view = & mut view_game_over;
				input = & mut input_game_over;
				view.init(& mut stage, & model);
				stage.draw(& mut canvas);
			},
			_ => ()
		}

		let mut diff: i32 = next_tick - timer.ticks() as i32;
		while diff > 0 {
			thread::sleep(time::Duration::from_millis(diff as u64));
			diff = next_tick - timer.ticks() as i32;
		}

		next_tick += FPS_DELAY;
	}
}
