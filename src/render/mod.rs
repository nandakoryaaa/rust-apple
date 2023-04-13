extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::game::Stage;

pub struct Sprite {
	pub w: u32,
	pub h: u32,
	pub data: &'static [u8]
}

pub struct SpriteSequence {
	pub frame_cnt: usize,
	pub frames: &'static [&'static Sprite]
}

impl SpriteSequence {
	pub fn get_w(& self, frame: usize) -> u32 {
		self.frames[frame].w
	}
	pub fn get_h(& self, frame: usize) -> u32 {
		self.frames[frame].h
	}
}

pub struct RendererRect {}

impl RendererRect {
	pub fn render(
		& self, stage: & Stage, canvas: & mut WindowCanvas,
		color: Color,
		rect: Rect
	) {
		canvas.set_draw_color(color);
		canvas.fill_rect(rect);
//			Rect::new(
//				rect.x * (stage.pixel_width as i32),
//				rect.y * (stage.pixel_height as i32),
//			 	(rect.w * (stage.pixel_width as i32)) as u32,
//				(rect.h * (stage.pixel_height as i32)) as u32 // WTF???
//			)
//		);
	}
}

pub struct RendererSpriteRLE {
	pub palette: &'static [Color],
	pub pixel_width: i32,
	pub pixel_height: i32
}

impl RendererSpriteRLE {
	pub fn render(
		& self, stage: & Stage, canvas: & mut WindowCanvas,
		s_x: i32, s_y: i32,
		sprite: & Sprite
	) {
		let pw = stage.pixel_width;
		let ph = stage.pixel_height;
		let w:i32 = sprite.w as i32;
		let mut pos:usize = 0;
		let rle: & [u8] = sprite.data;
		let rle_len:usize = rle.len();
		let mut len:i32;
		let mut x = 0;
		let mut y = s_y;

		while pos < rle_len {
			let mut index = rle[pos];
			pos += 1;
			if index & 128 != 0 {
				index &= !128;
				len = rle[pos] as i32;
				pos += 1;
			} else {
				len = 1;
			}
			if index == 0 {
				x += len;
				y += x / w;
				x %= w;
			} else {
				let color:Color = self.palette[index as usize];
				canvas.set_draw_color(color);
				while len > 0 {
					let limit = w - x;
					let chunk = if limit < len { limit } else { len };
					canvas.fill_rect(
						Rect::new(
							(s_x + x) * pw as i32,
							y * ph as i32,
							(chunk * pw as i32) as u32,
							ph
						)
					);
					len -= chunk;
					x += chunk;
					if x >= w {
						y += 1;
						x = 0;
					}
				}
			}
		}
	}
}

pub struct RendererSpriteAnimation {
	pub renderer: &'static RendererSpriteRLE
}

pub struct RendererText {
	pub pixel_width: u32,
	pub pixel_height: u32,
	pub font: &'static [u8]
}

impl RendererText {
	pub fn render(& self, stage: & Stage, canvas: & mut WindowCanvas, x:i32, y:i32, color: Color, s: & str) {
		let bytes = s.as_bytes();
		let s_len = bytes.len();
		let pw = stage.pixel_width;
		let ph = stage.pixel_height;
		let mut rect: Rect = Rect::new(x, y, pw, ph);
		canvas.set_draw_color(color);
		let mut i = 0;
		while i < s_len {
			let c:u8 = bytes[i];
			if c >= b'A' && c <= b'Z' {
				let idx = ((c - b'A') * 7) as usize;
				rect.y = y * ph as i32;
				for byte_pos in 0..7 {
					rect.x = (x + i as i32 * 8) * pw as i32;
					let mut byte = self.font[idx + byte_pos];
					while byte != 0 {
						if byte & 1 != 0 {
							canvas.fill_rect(rect);
						}
						byte >>= 1;
						rect.x += pw as i32;
					}
					rect.y += ph as i32;
				}
			}
			i += 1;
		}
	}
}

