
use super::rf32x2;
use simd::SseArth;

extern "C" {
    fn _add_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _sub_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _mul_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _div_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _max_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _min_f32x2(a: rf32x2, b: rf32x2) -> rf32x2;
    fn _sqrt_f32x2(a: rf32x2) -> rf32x2;
}

impl SseArth for rf32x2 {
	#[inline]
	fn addps(self, rhs: Self) -> Self {
		unsafe {
			_add_f32x2(self,rhs)
		}
	}
	#[inline]
	fn subps(self, rhs: Self) -> Self {
		unsafe {
			_sub_f32x2(self,rhs)
		}
	}
	#[inline]
	fn mulps(self, rhs: Self) -> Self {
		unsafe {
			_mul_f32x2(self,rhs)
		}
	}
	#[inline]
	fn divps(self, rhs: Self) -> Self {
		unsafe {
			_div_f32x2(self,rhs)
		}
	}
	#[inline]
	fn maxps(self, rhs: Self) -> Self {
		unsafe {
			_max_f32x2(self,rhs)
		}
	}
	#[inline]
	fn minps(self, rhs: Self) -> Self {
		unsafe {
			_min_f32x2(self,rhs)
		}
	}
	#[inline]
	fn sqrtps(self) -> Self {
		unsafe {
			_sqrt_f32x2(self)
		}
	}
}