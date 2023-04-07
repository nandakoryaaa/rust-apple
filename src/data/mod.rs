use sdl2::pixels::Color;

pub static PALETTE:[Color;4] = [
	Color::RGB(0, 0, 0),
	Color::RGB(255, 0, 0),
	Color::RGB(0, 255, 0),
	Color::RGB(0, 0, 255)
];

pub static SPRITE_APPLE:[u8;44] = [
	// w: 8, h:10
	128,4,1,128,6,1,128,7,1,128,5,129,4,2,1,0,129,4,130,3,129,5,130,4,129,4,130,3,129,6,2,129,2,0,129,6,128,3,1,131,2,1,128,2
];

pub struct Sprites {
	pub apple: &'static [u8]
}

pub static FONT:[u8;280] = [
//char w: 7, h: 7
	28, 54, 99, 99, 127, 99, 99, 
	63, 99, 99, 63, 99, 99, 63, 
	60, 102, 3, 3, 3, 102, 60, 
	31, 51, 99, 99, 99, 51, 31, 
	127, 3, 3, 127, 3, 3, 127, 
	127, 3, 3, 127, 3, 3, 3, 
	124, 6, 3, 115, 99, 102, 124, 
	99, 99, 99, 127, 99, 99, 99, 
	63, 12, 12, 12, 12, 12, 63, 
	96, 96, 96, 96, 96, 99, 62, 
	99, 51, 27, 15, 31, 59, 115, 
	3, 3, 3, 3, 3, 3, 127, 
	99, 119, 127, 107, 99, 99, 99, 
	99, 103, 111, 127, 123, 115, 99, 
	62, 99, 99, 99, 99, 99, 62, 
	63, 99, 99, 99, 63, 3, 3, 
	62, 99, 99, 99, 123, 51, 94, 
	63, 99, 99, 115, 31, 59, 115, 
	30, 51, 3, 62, 96, 99, 62, 
	63, 12, 12, 12, 12, 12, 12, 
	99, 99, 99, 99, 99, 99, 62, 
	99, 99, 99, 119, 62, 28, 8, 
	99, 99, 99, 107, 127, 119, 99, 
	99, 119, 62, 28, 62, 119, 99, 
	51, 51, 51, 30, 12, 12, 12, 
	127, 112, 56, 28, 14, 7, 127, 
	0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 
	0, 0, 0, 0, 0, 0, 0, 
	28, 50, 99, 99, 99, 38, 28, 
	12, 14, 12, 12, 12, 12, 63, 
	62, 99, 96, 60, 14, 7, 127, 
	126, 48, 24, 60, 96, 99, 62, 
	56, 60, 54, 51, 127, 48, 48, 
	127, 3, 63, 96, 96, 99, 62, 
	60, 6, 3, 63, 99, 99, 62, 
	127, 97, 48, 24, 12, 12, 12, 
	30, 35, 39, 30, 121, 97, 62, 
	62, 99, 99, 126, 96, 48, 30, 
];
