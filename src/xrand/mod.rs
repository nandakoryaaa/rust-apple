extern crate time;

pub struct XRand {
	s0: u32,
	s1: u32,
	s2: u32,
	s3: u32
}

#[allow(arithmetic_overflow)]
impl XRand {
	pub fn new() -> Self {
		let tm = time::now();
		Self {
			s0: tm.tm_sec as u32,
			s1: tm.tm_nsec as u32,
			s2: tm.tm_nsec as u32,
			s3: tm.tm_sec as u32
		}
	}

	pub fn randint(& mut self, min: u32, max:u32) -> u32 {
		let result:u32 = self.rotl(self.s1 * 5, 7) * 9;

		let t: u32 = self.s1 << 9;
		self.s2 ^= self.s0;
		self.s3 ^= self.s1;
		self.s1 ^= self.s2;
		self.s0 ^= self.s3;
		self.s2 ^= t;
		self.s3 = self.rotl(self.s3, 11);

		min + ((max - min) as u64 * result as u64 / u32::MAX as u64) as u32
	}

	#[inline]
	fn rotl(& self, x: u32, k: u32) -> u32 {
		(x << k) | (x >> (32 - k))
	}
}
