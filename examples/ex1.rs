extern crate rfloat;
extern crate frounding;
extern crate num_traits;


use num_traits::{Float,Num};
use rfloat::rf32x2;
use rfloat::simd::SseArth;
use frounding::RoundingState;

fn main() {
    let mut rs = RoundingState::new();
    let a: rf32x2 = rf32x2 { v0: 1.0f32, v1: 1.0f32 };
    let b: rf32x2 = rf32x2 { v0: 3.0f32, v1: 3.0f32 };
    rs.upward();
    println!("rounding sse {}, fpu {}",rs.current_rounding().0, rs.current_rounding().1);
    println!("1,1/3,3 {}", a.divps(b));
    println!("1,1+3,3 {}", a.addps(b));
    println!("1,1x3,3 {}", a.mulps(b));
    println!("1,1-3,3 {}", a.subps(b));
    println!("sqrt(3,3) {}", b.sqrtps());
    println!("max(1,1;3,3) {}", a.maxps(b));
    println!("min(1,1;3,3) {}", a.minps(b));
    rs.downward();
    println!("rounding sse {}, fpu {}",rs.current_rounding().0, rs.current_rounding().1);
    println!("1,1/3,3 {}", a.divps(b));
    println!("1,1+3,3 {}", a.addps(b));
    println!("1,1x3,3 {}", a.mulps(b));
    println!("1,1-3,3 {}", a.subps(b));
    println!("sqrt(3,3) {}", b.sqrtps());
    println!("max(1,1;3,3) {}", a.maxps(b));
    println!("min(1,1;3,3) {}", a.minps(b));
}
