#[macro_use]
extern crate lazy_static;

use std::f32::consts::PI;
use wasm_math::vec3::Vec3;
use rand::Rng;

lazy_static! {
    static ref NUMBER: u32 = 123;
    static ref spheres: Vec<Sphere> = vec![
        Sphere::new(1e5, Vec3::new(1e5 + 1., 40.8, 81.6), Vec3::default(), Vec3::new(0.75, 0.25, 0.25), Material::Diffuse), //left
    	Sphere::new(1e5, Vec3::new(-1e5 + 99.,40.8,81.6),Vec3::default(),Vec3::new(0.25,0.25,0.75), Material::Diffuse),//Rght
    	Sphere::new(1e5, Vec3::new(50.,40.8, 1e5),     Vec3::default(),Vec3::new(0.75,0.75,0.75), Material::Diffuse),//Back
    	Sphere::new(1e5, Vec3::new(50.,40.8,-1e5 + 170.), Vec3::default(),Vec3::default(),           Material::Diffuse),//Frnt
    	Sphere::new(1e5, Vec3::new(50., 1e5, 81.6),    Vec3::default(),Vec3::new(0.75,0.75,0.75),Material::Diffuse),//Botm
    	Sphere::new(1e5, Vec3::new(50.,-1e5 + 81.6,81.6),Vec3::default(),Vec3::new(0.75,0.75,0.75),Material::Diffuse),//Top
    	Sphere::new(16.5,Vec3::new(27.,16.5,47.),       Vec3::default(),Vec3::new(1.,1.,1.), Material::Specular),//Mirr
    	Sphere::new(16.5,Vec3::new(73.,16.5,78.),       Vec3::default(),Vec3::new(1.,1.,1.), Material::Refract),//Glas
    	Sphere::new(600., Vec3::new(50.,681.6 - 0.27,81.6),Vec3::new(12.,12.,12.),  Vec3::default(), Material::Diffuse) //Lite
    ];
}

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
	center: Vec3,
	emission: Vec3,
	color: Vec3,
	material: Material,
}

impl Sphere {
	pub fn new(radius: f32, center: Vec3, emission: Vec3, color: Vec3, material: Material) -> Self {
		Sphere {
			radius,
			center,
			emission,
			color,
			material,
		}
	}
	pub fn intersect(&self, ray: &Ray) -> f32 {
		let mut op = Vec3::default();
		op.sub2(&self.center, &ray.origin);
		let eps: f32 = 0.0001;
		let b = op.dot(&ray.direct);
		let mut det = b * b + self.radius * self.radius - op.dot(&op);
		if det < 0.0 {
			return 0.0;
		} else {
			det = det.sqrt();
		}
		let mut t = b - det;
		if t > eps {
			return t;
		}
		t = b + det;
		if t > eps {
			t
		} else {
			0.0
		}
	}
}

fn main() {
	let mut rng = rand::thread_rng();
	println!("Hello, world!,{}", rng.gen::<f32>());
}
