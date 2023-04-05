extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
//use sdl2::pixels::PixelFormatEnum;
//use sdl2::surface::Surface;
//use sdl2::render::{ Texture, TextureCreator, TextureAccess };
//use sdl2::video::WindowContext;

pub trait Renderer {
	fn render(
		& self, canvas: & mut WindowCanvas,
		gmo: & GameObject
	);
}

pub struct RendererRect {}
pub struct RendererSprite {
	pub palette: &'static [Color],
	pub pixel_width: u8,
	pub pixel_height: u8
}

impl Renderer for RendererRect {
	fn render(
		& self, canvas: & mut WindowCanvas,
		gmo: & GameObject
	) {
		canvas.set_draw_color(gmo.color);
		canvas.fill_rect(gmo.rect);
	}
}

impl RendererSprite {
	fn new(palette: &'static [Color]) -> Self {
		Self {
			palette: palette,
			pixel_width: 4,
			pixel_height: 3
		}
	}
}

impl Renderer for RendererSprite {
	fn render(
		& self, canvas: & mut WindowCanvas,
		gmo: & GameObject
	) {
		let mut x:u32 = 0;
		let mut y:u32 = 0;
		let w:u32 = 8;
		let mut pos:usize = 0;
		let rle: &[u8] = gmo.sprite;
		let rle_len:usize = rle.len();
		let mut len:u32 = 0;

		while pos < rle_len {
			let mut index = rle[pos];
			pos += 1;
			if index & 128 != 0 {
				index &= !128;
				len = rle[pos] as u32;
				pos += 1;
			} else {
				len = 1;
			}
			if index == 0 {
				x += len as u32;
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
							(gmo.rect.x + x as i32) * self.pixel_width as i32,
							(gmo.rect.y + y as i32) * self.pixel_height as i32,
							chunk as u32 * self.pixel_width as u32,
							self.pixel_height as u32
						)
					);
					len -= chunk;
					x += chunk as u32;
					if x >= w {
						y += 1;
						x = 0;
					}
				}
			}
		}
	}
}

pub struct GameObject {
	color: Color,
	pub rect: Rect,
	pub sprite: &'static [u8],
	// Renderer должен жить не меньше, чем GameObject
	renderer: &'static dyn Renderer
}

impl GameObject {
	pub fn new(
		x: i32, y: i32,
		w: u32, h: u32,
		color: Color,
		sprite: &'static [u8],
		renderer: &'static dyn Renderer
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

