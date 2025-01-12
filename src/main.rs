use nalgebra;

pub mod core;
pub mod accelerators;
pub mod shapes;
pub mod interface;

use core::math::Vector3d;

fn main() {
    let mut v = Vector3d::new(1.0f64, 2.0f64, 3.0f64);
    v.x = 1.0f64;
    println!("Hello, world!");
}
