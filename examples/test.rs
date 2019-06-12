extern crate cgmath;

use wasm_math::vec3::Vec3;
use cgmath::*;

fn main() {
	assert_eq!(vec3(1f64, 2f64, 3f64), Vector3::new(1f64, 2f64, 3f64));
	let s = vec3(5f64, 2f64, 3f64).rem_element_wise(vec3(1f64, 2f64, 3f64));
	let s2 = Vec3::new(5f32, 2f32, 3f32) % Vec3::new(1f32, 2f32, 3f32);
	println!("{:?},{:?}", s, s2);
}