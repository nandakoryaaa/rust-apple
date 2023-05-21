use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::render::{ RendererRect, RendererSpriteRLE, RendererText, RendererNumber, RendererFont, Sprite, SpriteSequence };
use crate::game::{ GMO, PlayerAnimationState };
use crate::data::{ SPRITE_APPLE, PALETTE, FONT };
use crate::data as cd;

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
	pub renderer_number: RendererNumber,
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
			looped: true,
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

	pub fn create_number(&'static self, x: i32, y: i32, color: Color, number: i32, padding: i32) -> GMO {
		GMO::GmoNumber {
			x: x,
			y: y,
			color: color,
			number: number,
			padding: padding,
			renderer: & self.renderer_number
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

	pub fn create_rect(&'static self, x: i32, y: i32, w: u32, h: u32, color: Color, pixel_width: u32, pixel_height: u32) -> GMO {
		GMO::GmoRect {
			x: x,
			y: y,
			w: w,
			h: h,
			color: color,
			rect: Rect::new(x * pixel_width as i32, y * pixel_height as i32, pixel_width * w, pixel_height * h),
			renderer: & self.renderer_rect
		}
	}

	pub fn get_state(&'static self, state: PlayerAnimationState) -> & SpriteSequence {
		match state {
			PlayerAnimationState::StandLeft => { & self.sq_player_stand_l },
			PlayerAnimationState::MoveLeft => { & self.sq_player_move_l },
			PlayerAnimationState::StandRight => { & self.sq_player_stand_r },
			PlayerAnimationState::MoveRight => { & self.sq_player_move_r },
			PlayerAnimationState::Death => { & self.sq_player_death },
			_ => { & self.sq_player_stand }
		}
	}
}

pub static GMO_FACTORY: GmoFactory = GmoFactory {
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
		frame_cnt: 7,
		frames: & [
			& Sprite { w: 22, h: 30, data: & cd::SPRITE_PLAYER_0 },
			& Sprite { w: 22, h: 30, data: & cd::SPRITE_PLAYER_1 },
			& Sprite { w: 23, h: 30, data: & cd::SPRITE_PLAYER_2 },
			& Sprite { w: 23, h: 30, data: & cd::SPRITE_PLAYER_3 },
			& Sprite { w: 23, h: 30, data: & cd::SPRITE_PLAYER_4 },
			& Sprite { w: 24, h: 30, data: & cd::SPRITE_PLAYER_5 },
			& Sprite { w: 24, h: 30, data: & cd::SPRITE_PLAYER_6 },
		]
	},
	renderer_rect: RendererRect {},
	renderer_sprite_rle: RendererSpriteRLE {
		palette: & PALETTE,
		pixel_width: 1,
		pixel_height: 1
	}, 
	renderer_text: RendererText {
		renderer: RendererFont {
			pixel_width: 1,
			pixel_height: 1,
			font: & FONT
		}
	},
	renderer_number: RendererNumber {
		renderer: RendererFont {
			pixel_width: 1,
			pixel_height: 1,
			font: & FONT
		}
	}
};
