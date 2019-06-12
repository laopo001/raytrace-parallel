extern crate cgmath;

use cgmath::*;

fn main() {
	assert_eq!(vec3(1f64, 2f64, 3f64), Vector3::new(1f64, 2f64, 3f64));
	let s = vec3(1f64, 2f64, 3f64) * vec3(1f64, 2f64, 3f64) ;


}