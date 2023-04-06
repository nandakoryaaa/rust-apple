extern crate sdl2;

use crate::game::GameObject;
use crate::game::Stage;
//use crate::game::Renderer;
use crate::game::render::RendererSpriteRLE;
use crate::game::render::RendererFactory;
use crate::sdl2::pixels::Color;
use crate::data::Sprites;

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
	pub fn new(stage: & mut Stage, renderer_factory: &'static RendererFactory, sprites: &'static Sprites) -> Self {
		stage.add_child(
			GameObject::new(0, 0, 10, 30, Color::RGB(255, 0, 0), sprites.apple, & renderer_factory.renderer_sprite_rle)
		);
		Self {
			//player: GameObject::new(0, 0, 10, 30, Color::RGB(255, 0, 0), & RendererRect {})
		}
	}

	pub fn get_player<'a>(& self, stage: &'a mut Stage) -> &'a mut GameObject {
		return stage.get_child(0);
	}
}
