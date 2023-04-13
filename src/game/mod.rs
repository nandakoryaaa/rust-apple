extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::render::{ RendererRect, RendererSpriteRLE, RendererText, Sprite, SpriteSequence };

#[derive (Copy, Clone)]
pub enum PlayerAnimationState {
	Stand,
	StandLeft,
	StandRight,
	MoveLeft,
	MoveRight,
	Death
}

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
		sprite: &'static Sprite,
		renderer: &'static RendererSpriteRLE
	},

	GmoSpriteAnimated {
		x: i32,
		y: i32,
		w: u32,
		h: u32,
		state: PlayerAnimationState,
		frame: usize,
		delay: u32,
		sequence: &'static SpriteSequence,
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
	pub fn render(& self, canvas: & mut WindowCanvas) {
		match self {
			GMO::GmoSprite { renderer, sprite, x, y, .. } => { renderer.render(canvas, *x, *y, sprite); },
			GMO::GmoSpriteAnimated { renderer, sequence, frame, x, y, .. } => { renderer.render(canvas, *x, *y, (*sequence).frames[*frame]); },
			GMO::GmoText { renderer, color, x, y, text, .. } => { renderer.render(canvas, *x, *y, *color, text); },
			_ => ()
		}
	}

	pub fn update(& mut self) {
		match self {
			GMO::GmoSpriteAnimated { frame, sequence, .. } => {
				let mut f:usize = *frame;
				f += 1;
				if f == sequence.frame_cnt {
					f = 0;
				}
				*frame = f;
			},
			_ => ()
		}
	}
}

pub struct Stage {
	pub w: u32,
	pub h: u32,
	canvas: WindowCanvas,
	obj_list: Vec<GMO>
}

impl Stage {
	pub fn new(w: u32, h: u32, canvas: WindowCanvas) -> Self {
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

	pub fn get_child(& mut self, i: usize) -> & mut GMO {
		return & mut self.obj_list[i];
	}

	pub fn add_child(& mut self, child: GMO) {
		self.obj_list.push(child);
	}
}
