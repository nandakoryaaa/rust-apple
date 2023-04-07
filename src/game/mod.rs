pub mod render;

extern crate sdl2;

//use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::game::render::RendererRect;
use crate::game::render::RendererSpriteRLE;
use crate::game::render::RendererText;

pub enum GMO {
	GmoRect {
		x: i32,
		y: i32,
		w: u32,
		h: u32,
		color: Color,
		renderer: &'static RendererRect
	},
	GmoSprite {
		x: i32,
		y: i32,
		w: u32,
		h: u32,
		sprite: &'static [u8],
		renderer: &'static RendererSpriteRLE
	},
	GmoText {
		x: i32,
		y: i32,
		color: Color,
		text: &'static str,
		renderer: &'static RendererText
	}
}

impl GMO {
	pub fn newGmoRect(
		x: i32,
		y: i32,
		w: u32,
		h: u32,
		color: Color,
		renderer: &'static RendererRect
	) -> Self {
		GMO::GmoRect {
			x: x,
			y: y,
			w: w,
			h: h,
			color: color,
			renderer: renderer
		}
	}

	pub fn newGmoSprite(
		x: i32,
		y: i32,
		w: u32,
		h: u32,
		sprite: &'static [u8],
		renderer: &'static RendererSpriteRLE
	) -> Self {
		GMO::GmoSprite {
			x: x,
			y: y,
			w: w,
			h: h,
			sprite: sprite,
			renderer: renderer
		}
	}

	pub fn newGmoText(
		x: i32,
		y: i32,
		color: Color,
		text: &'static str,
		renderer: &'static RendererText
	) -> Self {
		GMO::GmoText {
			x: x,
			y: y,
			color: color,
			text: text,
			renderer: renderer
		}
	}

	pub fn render(& self, canvas: & mut WindowCanvas) {
		match self {
			GMO::GmoSprite { renderer: r, sprite: s, .. } => { r.render(canvas, s); },
			GMO::GmoText { renderer: r, color: c, x: x, y: y, text: t, .. } => { r.render(canvas, *x, *y, *c, t); },
			_ => ()
		}
	}
}

pub struct Stage {
	pub w: u32,
	pub h: u32,
	canvas: WindowCanvas,
	// в Vec находятся GameObject, которые должны жить не меньше чем Stage
	// а как они могут жить меньше, если они принадлежат Vec, а Vec принадлежит Stage?
	// потому что они содержат ссылки на Renderer
	// по факту не GameObject, а Renderer должен жить не меньше чем Stage
	obj_list: Vec<GMO>
}

impl Stage {
	pub fn new(w:u32, h:u32, canvas: WindowCanvas) -> Self {
		Self {
			w: w,
			h: h,
			canvas: canvas,
			obj_list: Vec::<GMO>::new()
		}
	}

	pub fn draw(& mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.fill_rect(None);
		
		for i in 0..self.obj_list.len() {
			let o = & self.obj_list[i];
			o.render(& mut self.canvas);
		}
		self.canvas.present();
	}

	pub fn get_child(& mut self, i:usize) -> & mut GMO {
		return & mut self.obj_list[i];
	}

	pub fn add_child(& mut self, child: GMO) {
		self.obj_list.push(child);
	}
}

