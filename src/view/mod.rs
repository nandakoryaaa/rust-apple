extern crate sdl2;

use crate::game::GameObject;
use crate::game::Stage;
//use crate::game::Renderer;
use crate::game::RendererRect;
use crate::sdl2::pixels::Color;

pub trait View {
	//pub fn new(& mut self, & mut stage);
}

pub struct MainView {
	//pub player: GameObject<'a>
	//apple: GameObject,
}

impl View for MainView {
}

impl MainView {
	pub fn new(stage: & mut Stage) -> Self {
		stage.add_child(
			GameObject::new(0, 0, 10, 30, Color::RGB(255, 0, 0), & RendererRect {})
		);
		Self {
			//player: GameObject::new(0, 0, 10, 30, Color::RGB(255, 0, 0), & RendererRect {})
		}
	}

	pub fn get_player(& self, stage: & mut Stage) -> & mut GameObject {
		return stage.get_child(0);
	}
}