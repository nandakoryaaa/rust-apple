pub mod render;

extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use crate::game::render::GmoRenderer;

pub struct GameObject {
	color: Color,
	pub rect: Rect,
	pub sprite: &'static [u8],
	// Renderer должен жить не меньше, чем GameObject
	renderer: &'static dyn GmoRenderer
}

impl GameObject {
	pub fn new(
		x: i32, y: i32,
		w: u32, h: u32,
		color: Color,
		sprite: &'static [u8],
		renderer: &'static dyn GmoRenderer
	) -> Self {
		Self {
			color: color,
			rect: Rect::new(x, y, w, h),
			sprite: sprite,
			renderer: renderer
		}
	}
}

pub struct Stage {
	pub w: u32,
	pub h: u32,
//	texture_creator: TextureCreator<WindowContext>,
	canvas: WindowCanvas,
	// в Vec находятся GameObject, которые должны жить не меньше чем Stage
	// а как они могут жить меньше, если они принадлежат Vec, а Vec принадлежит Stage?
	// потому что они содержат ссылки на Renderer
	// по факту не GameObject, а Renderer должен жить не меньше чем Stage
	obj_list: Vec<GameObject>
}

impl Stage {
	pub fn new(w:u32, h:u32, canvas: WindowCanvas) -> Self {
		//let	tc = canvas.texture_creator();
		Self {
			w: w,
			h: h,
			canvas: canvas,
			obj_list: Vec::<GameObject>::new()
		}
	}

	pub fn draw(& mut self) {
		self.canvas.set_draw_color(Color::RGB(0, 0, 0));
		self.canvas.fill_rect(None);
		
		for i in 0..self.obj_list.len() {
			let o = & self.obj_list[i];
			o.renderer.render(& mut self.canvas, o);
		}
		self.canvas.present();
	}

	pub fn get_child(& mut self, i:usize) -> & mut GameObject {
		return & mut self.obj_list[i];
	}

	pub fn add_child(& mut self, child: GameObject) {
		self.obj_list.push(child);
	}

//	pub fn get_texture<'a>(& self, tex_creator: &'a TextureCreator<WindowContext>) -> Texture<'a> {
//		let surface = Surface::new(512, 512, PixelFormatEnum::RGB24).unwrap();
//		let tex = Texture::from_surface(& surface, & tex_creator).unwrap();
//		tex_creator.create_texture_static(None, self.w, self.h).unwrap()
//	}
}

