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

fn erand48() -> f32 {
	let mut rng = rand::thread_rng();
	rng.gen::<f32>()
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
		let mut op = self.center - ray.origin;
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

fn intersect(ray: &Ray, distance: &mut f32, id: &mut usize) -> bool {
	let size = spheres.len();
	*distance = 1e20;
	let inf = *distance;
	for i in 0..size {
		let d = spheres[i].intersect(ray);
		if d != 0.0 && d < *distance {
			*distance = d;
			*id = i;
		}
	}
	return *distance < inf;
}

fn radiance(ray: &Ray, depth: i32) -> Vec3 {
	let mut t = 0_f32;
	let mut id = 0;
	if !intersect(ray, &mut t, &mut id) {
		return Vec3::default();
	}
	let obj = &spheres[id];
	let x = ray.origin + ray.direct.scale(t);
	let n = (x - obj.center).normalize();
	let nl = if n.dot(&ray.direct) < 0.0 {
		n
	} else {
		n.scale(-1.0)
	};
	let f: Vec3 = obj.color;
	let mut p = std::f32::MIN;
	for item in f.data().iter() {
		if item > &p {
			p = *item;
		}
	}
//	nl=n.dot(r.d)<0?n:n*-1

	Vec3::default()
}

fn main() {
	let mut rng = rand::thread_rng();
	let width: f32 = 256.0;
	let height: f32 = 256.0;
	let samples: f32 = 25.0;
	let mut direct = Vec3::new(0.0, -0.042612, -1.0);
	direct.normalize();
	let camera = Ray::new(
		Vec3::new(50.0, 52.0, 295.6),
		direct,
	);
	let cx = Vec3::new(width * 0.5135 / height, 0.0, 0.0);
	let mut cy = cx * camera.direct;
	cy.normalize();
	cy.scale(0.5135);
	let content = Vec3::new(width * height, 0.0, 0.0);

	println!("Hello, world!,{}", rng.gen::<f32>());
}
