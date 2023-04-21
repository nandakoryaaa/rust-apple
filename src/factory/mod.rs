use sdl2::pixels::Color;
use crate::render::{ RendererRect, RendererSpriteRLE, RendererText, Sprite, SpriteSequence };
use crate::game::{ Stage, GMO, PlayerAnimationState };
use crate::data::{ SPRITE_APPLE, PALETTE, FONT };
use crate::data as cd; //::{ SPRITE_PLAYER_0,  SPRITE_PLAYER_2,  SPRITE_PLAYER_2,  SPRITE_PLAYER_3, SPRITE_PLAYER_4, SPRITE_PLAYER_5,;
use crate::input::{ Input, InputMain };
use crate::controller::{ Controller, ControllerMain };
use crate::view::{ View, ViewMain };

pub struct GmoFactory {
	pub sp_apple: Sprite,
	pub sq_player_stand: SpriteSequence,
	pub sq_player_stand_l: SpriteSequence,
	pub sq_player_stand_r: SpriteSequence,
	pub sq_player_move_l: SpriteSequence,
	pub sq_player_move_r: SpriteSequence,
	pub sq_player_death: SpriteSequence,

	pub renderer_rect: RendererRect,
	pub renderer_sprite_rle: RendererSpriteRLE,
	pub renderer_text: RendererText,
}

impl GmoFactory {
	pub fn create_player(&'static self) -> GMO {
		GMO::GmoSpriteAnimated {
			x: 0,
			y: 0,
			w: 0,
			h: 0,
			state: PlayerAnimationState::Stand,
			frame: 0,
			delay: 0,
			sequence: & self.sq_player_stand,
			renderer: & self.renderer_sprite_rle
		}
	}

	pub fn create_text(&'static self, x: i32, y: i32, color: Color, text: &'static str) -> GMO {
		GMO::GmoText {
			x: x,
			y: y,
			color: color,
			text: text,
			renderer: & self.renderer_text
		}
	}

	pub fn create_apple(&'static self, x: i32, y: i32) -> GMO {
		GMO::GmoSprite {
			x: x,
			y: y,
			w: self.sp_apple.w,
			h: self.sp_apple.h,
			sprite: & self.sp_apple,
			renderer: & self.renderer_sprite_rle
		}
	}

	pub fn get_state(&'static self, state: PlayerAnimationState) -> & SpriteSequence {
		match state {
			PlayerAnimationState::MoveLeft => { & self.sq_player_move_l },
			PlayerAnimationState::MoveRight => { & self.sq_player_move_r },
			_ => { & self.sq_player_stand }
		}
	}

}

pub static gmo_factory: GmoFactory = GmoFactory {
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

pub struct MvcFactoryMain {
	pub controller: ControllerMain,
	pub view: ViewMain,
	pub input: InputMain
}

pub struct MvcAbstractFactory {
	pub factory_main: MvcFactoryMain
}
