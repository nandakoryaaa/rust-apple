extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
//use crate::game::GameObject;
//use crate::game::GMO;
use crate::data::Sprites;

pub struct RendererRect {}

impl RendererRect {
	pub fn render(
		& self, canvas: & mut WindowCanvas,
		color: Color,
		rect: Rect
	) {
		canvas.set_draw_color(color);
		canvas.fill_rect(rect);
	}
}

pub struct RendererSpriteRLE {
	pub palette: &'static [Color],
	pub pixel_width: i32,
	pub pixel_height: i32
}

impl RendererSpriteRLE {
	pub fn render(
		& self, canvas: & mut WindowCanvas,
		rle: &[u8]
	) {
		let mut x:i32 = 0;
		let mut y:i32 = 0;
		let w:i32 = 8;
		let mut pos:usize = 0;
		let rle_len:usize = rle.len();
		let mut len:i32 = 0;

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
							x * self.pixel_width,
							y * self.pixel_height,
							(chunk * self.pixel_width) as u32,
							self.pixel_height as u32
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

pub struct RendererText {
	pub font: &'static [u8]
}

impl RendererText {
	pub fn render(& self, canvas: & mut WindowCanvas, x:i32, y:i32, color: Color, s: & str) {
		let bytes = s.as_bytes();
		let s_len = bytes.len();
		let mut i = 0;
		let mut rect: Rect = Rect::new(x, y, 8, 8);
		canvas.set_draw_color(color);
		while i < s_len {
			let c:u8 = bytes[i];
			if c >= b'A' && c <= b'Z' {
				let idx = ((c - b'A') * 7) as usize;
				rect.y = y;
				for byte_pos in 0..7 {
					rect.x = x + (i * 8 * 8) as i32;
					let mut byte = self.font[idx + byte_pos];
					while byte != 0 {
						if byte & 1 != 0 {
							canvas.fill_rect(rect);
						}
						byte >>= 1;
						rect.x += 8;
					}
					rect.y += 8;
				}
			}
			i += 1;
		}
	}
}

pub struct RendererFactory {
	pub renderer_rect: RendererRect,
	pub renderer_sprite_rle: RendererSpriteRLE,
	pub renderer_text: RendererText,
	pub sprites: Sprites
}

