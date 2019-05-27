use std::f32::consts::PI;
use wasm_math::vec3::Vec3;
use rand::Rng;

struct Ray {
	pub origin: Vec3,
	pub direct: Vec3,
}

impl Ray {
	pub fn new(origin: Vec3, direct: Vec3) -> Self {
		Ray { origin, direct }
	}
}

enum Material {
	Diffuse,
	Specular,
	Refract,
}

struct Sphere {
	radius: f32,
	position: Vec3,
	emission: Vec3,
	color: Vec3,
	material: Material,
}

impl Sphere {
	pub fn new(radius: f32, position: Vec3, emission: Vec3, color: Vec3, material: Material) -> Self {
		Sphere {
			radius,
			position,
			emission,
			color,
			material,
		}
	}
	pub fn intersect(&self, ray: &Ray) -> f32 {
		0.0
	}
}

fn main() {
	let mut rng = rand::thread_rng();
	println!("Hello, world!,{}", rng.gen::<f32>());
}
