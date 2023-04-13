extern crate sdl2;

use sdl2::rect::Rect;
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
		rect: Rect,
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
	pub fn render(& self, stage: & Stage, canvas: & mut WindowCanvas) {
		match self {
			GMO::GmoRect { renderer, color, rect, .. } => { renderer.render(stage, canvas, *color, *rect); },
			GMO::GmoSprite { renderer, sprite, x, y, .. } => { renderer.render(stage, canvas, *x, *y, sprite); },
			GMO::GmoSpriteAnimated { renderer, sequence, frame, x, y, .. } => { renderer.render(stage, canvas, *x, *y, (*sequence).frames[*frame]); },
			GMO::GmoText { renderer, color, x, y, text, .. } => { renderer.render(stage, canvas, *x, *y, *color, text); }
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
	pub grid_w: u32,
	pub grid_h: u32,
	pub pixel_width: u32,
	pub pixel_height: u32,
	//pub canvas: WindowCanvas,
	obj_list: Vec<GMO>
}

impl Stage {
	pub fn new(window_w: u32, window_h: u32, grid_w: u32, grid_h: u32
		//, canvas: WindowCanvas
	) -> Self {
		let mut p_w: u32 = window_w / grid_w;
		let mut p_h: u32 = window_h / grid_h;
		if p_w < 1 {
			p_w = 1;
		}
		if p_h < 1 {
			p_h = 1;
		}
		Self {
			grid_w: grid_w,
			grid_h: grid_h,
			pixel_width: p_w,
			pixel_height: p_h,
	//		canvas: canvas,
			obj_list: Vec::<GMO>::new()
		}
	}

	pub fn draw(& self, canvas: & mut WindowCanvas) {
		canvas.set_draw_color(Color::RGB(0, 0, 0));
		canvas.fill_rect(None);
		
		for i in 0..self.obj_list.len() {
			//let o = & self.obj_list[i];
			//o.render(self);
			& self.obj_list[i].render(self, canvas);
//			self.render(i);
		}
		canvas.present();
	}

	pub fn get_child(& mut self, i: usize) -> & mut GMO {
		return & mut self.obj_list[i];
	}

	pub fn add_child(& mut self, child: GMO) -> usize {
		let last_index: usize = self.obj_list.len();
		self.obj_list.push(child);
		last_index
	}
}
