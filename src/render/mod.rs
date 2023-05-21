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
		& self, _stage: & Stage, canvas: & mut WindowCanvas,
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

pub struct RendererFont {
	pub pixel_width: u32,
	pub pixel_height: u32,
	pub font: &'static [u8]
}

impl RendererFont {
	pub fn render(& self, stage: & Stage, canvas: & mut WindowCanvas, x:i32, y:i32, idx:usize) {
		let pw: u32 = stage.pixel_width;
		let ph: u32 = stage.pixel_height;
		let mut rect: Rect = Rect::new(x, y, pw, ph);

		rect.y = y * ph as i32;
		for byte_pos in 0..7 {
			rect.x = x * pw as i32;
			let mut byte = self.font[idx * 7 + byte_pos];
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
}

pub struct RendererNumber {
	pub renderer: RendererFont
}

impl RendererNumber {
	pub fn render(& self, stage: & Stage, canvas: & mut WindowCanvas, mut x: i32, y: i32, color: Color, mut number: i32, mut padding: i32) {
		canvas.set_draw_color(color);
		let mut buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
		let mut pos: usize = 7;

		loop {
			buffer[pos] = (number % 10) as u8;
			padding -= 1;
			number /= 10;
			if number == 0 {
				break;
			}
			pos -= 1;
		}

		if padding > 0 {
			pos -= padding as usize;
		}

		while pos < 8 {
			self.renderer.render(stage, canvas, x, y, 30 + buffer[pos] as usize);
			x += 8;
			pos += 1;
		}
	}
}

pub struct RendererText {
	pub renderer: RendererFont
}

impl RendererText {
	pub fn render(& self, stage: & Stage, canvas: & mut WindowCanvas, mut x: i32, y: i32, color: Color, s: & str) {
		let bytes = s.as_bytes();
		let s_len = bytes.len();
		canvas.set_draw_color(color);
		let mut idx: usize = 0;
		let mut i = 0;
		while i < s_len {
			let c: u8 = bytes[i];
			i += 1;
			// dirty hacks, needs lookup table
			if c >= b'A' && c <= b'Z' {
				idx = (c - b'A') as usize;
			} else if c >= b'0' && c <= b'9' {
				idx = (c - b'0' + 30) as usize;
			} else if c == b'-' {
				idx = 26;
			} else if c == b'+' {
				idx = 27;
			} else if c == b'!' {
				idx = 28;
			} else if c == b'?' {
				idx = 29;
			} else {
				x += 8;
				continue;
			}

			self.renderer.render(stage, canvas, x, y, idx);
			x += 8;
		}
	}
}

