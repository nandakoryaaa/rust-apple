extern crate sdl2;
use sdl2::pixels::Color;
//use crate::game::GMO::GmoSpriteRLE;
use crate::game::GMO;
use crate::game::Stage;
use crate::game::render::RendererFactory;

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
	pub fn new(stage: & mut Stage, renderer_factory: &'static RendererFactory) -> Self {
		stage.add_child(
			GMO::newGmoSprite(
				0, 0, 10, 30,
				& renderer_factory.sprites.apple,
				& renderer_factory.renderer_sprite_rle
			)
		);
		stage.add_child(
			GMO::newGmoText(
				100, 200,
				Color::RGB(0, 255, 0),
				& "APPLE GAME",
				& renderer_factory.renderer_text
			)
		);
		Self {
			//player: GameObject::new(0, 0, 10, 30, Color::RGB(255, 0, 0), & RendererRect {})
		}
	}

	pub fn get_player<'a>(& self, stage: &'a mut Stage) -> &'a mut GMO {
		return stage.get_child(0);
	}
}
