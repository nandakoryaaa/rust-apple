extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::game::GameObject;

pub trait GmoRenderer {
	fn render(
		& self, canvas: & mut WindowCanvas,
		gmo: & GameObject
	);
}

pub struct RendererRect {}

pub struct RendererSpriteRLE {
	pub palette: &'static [Color],
	pub pixel_width: i32,
	pub pixel_height: i32
}

impl GmoRenderer for RendererRect {
	fn render(
		& self, canvas: & mut WindowCanvas,
		gmo: & GameObject
	) {
		canvas.set_draw_color(gmo.color);
		canvas.fill_rect(gmo.rect);
	}
}

impl RendererSpriteRLE {
	fn new(palette: &'static [Color]) -> Self {
		Self {
			palette: palette,
			pixel_width: 4,
			pixel_height: 3
		}
	}
}

impl GmoRenderer for RendererSpriteRLE {
	fn render(
		& self, canvas: & mut WindowCanvas,
		gmo: & GameObject
	) {
		let mut x:i32 = 0;
		let mut y:i32 = 0;
		let w:i32 = 8;
		let mut pos:usize = 0;
		let rle: &[u8] = gmo.sprite;
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
							(gmo.rect.x + x) * self.pixel_width,
							(gmo.rect.y + y) * self.pixel_height,
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
	pub fn render(& self, canvas: & mut WindowCanvas, x:i32, y:i32, color:u32, s: &String) {
		let bytes = s.as_bytes();
		let s_len = bytes.len();
		let mut i = 0;
		while i < s_len {
			let c:u8 = bytes[i];
			i += 1;
			let idx = c - 65;
		}
	}
}

pub struct RendererFactory {
	pub renderer_rect: RendererRect,
	pub renderer_sprite_rle: RendererSpriteRLE,
	pub renderer_text: RendererText
}

