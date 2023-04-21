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
use crate::factory::{ GmoFactory, MvcAbstractFactory, MvcFactoryMain };
use crate::game::{ Stage, PlayerAnimationState, GameState, GameStateEvent };
use crate::input::{ Input, InputTitle, InputMenu, InputMain };
use crate::controller::{ Controller, ControllerTitle, ControllerMenu, ControllerMain };
use crate::view::{ View, ViewTitle, ViewMenu, ViewMain };
use crate::data::{ SPRITE_APPLE, PALETTE, FONT };
use crate::data as cd; //::{ SPRITE_PLAYER_0,  SPRITE_PLAYER_2,  SPRITE_PLAYER_2,  SPRITE_PLAYER_3, SPRITE_PLAYER_4, SPRITE_PLAYER_5,;

use crate::model::{ Model, ModelFactory };
use crate::factory::gmo_factory;

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
	let mut game_state = GameState::new(GameStateEvent::RunMenu);

	let mut controller_title = ControllerTitle {};
	let mut controller_menu = ControllerMenu {};
	let mut controller_main = ControllerMain {};
	let mut view_title = ViewTitle::new(& gmo_factory);
	let mut view_menu = ViewMenu::new(& gmo_factory);
	let mut view_main = ViewMain::new(& gmo_factory);
	let mut input_title = InputTitle::new();
	let mut input_menu = InputMenu::new();
	let mut input_main = InputMain::new();
	//let mut model_menu = ModelFactory::ModelMenu();
	//let mut model_main = ModelFactory::ModelMain();

	let mut controller: & mut dyn Controller = & mut controller_title;
	let mut view: & mut dyn View = & mut view_title;
	let mut input: & mut dyn Input = & mut input_title;
	let mut model = ModelFactory::ModelTitle();

	view.init(& mut stage, & model);
	stage.draw(& mut canvas);

	let mut evt_pump = sdl.event_pump().unwrap();
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
					input.set_event(k);
				},
				_ => ()
			}

			let controller_evt = controller.update(& mut model, input);
			match controller_evt {
				GameStateEvent::Run => {
					input.clear();
					view.update(& mut stage, & model);
					stage.draw(& mut canvas);
				},
				GameStateEvent::RunMenu => {
					stage.clear();
					controller = & mut controller_menu;
					view = & mut view_menu;
					input = & mut input_menu;
					input.clear();
					model = ModelFactory::ModelMenu();
					view.init(& mut stage, & model);
					stage.draw(& mut canvas);
				},
				GameStateEvent::RunMain => {
					stage.clear();
					controller = & mut controller_main;
					view = & mut view_main;
					input = & mut input_main;
					input.clear();
					model = ModelFactory::ModelMain();
					view.init(& mut stage, & model);
					stage.draw(& mut canvas);
				},
				_ => ()
			}
		}
	}
}
