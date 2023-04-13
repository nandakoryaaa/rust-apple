use sdl2::pixels::Color;
use crate::render::{ RendererRect, RendererSpriteRLE, RendererText, Sprite, SpriteSequence };
use crate::game::{ GMO, PlayerAnimationState };

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
