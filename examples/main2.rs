#[macro_use]
extern crate lazy_static;
extern crate cgmath;

use std::f64::consts::PI;
//use wasm_math::vec3::Vector3::<f64>;
use rand::Rng;
use rand::prelude::ThreadRng;
use std::ops::{Add, Sub, AddAssign, SubAssign, MulAssign, Mul, Rem};
use cgmath::*;

#[macro_use]
extern crate nameof;

trait More<T> {
	fn default() -> Vector3<T>;
	fn scale(&self, s: f64) -> Vector3<T>;
	fn rem2(self, b: Self) -> Self;
}

impl More<f64> for Vector3<f64> {
	fn default() -> Vector3<f64> {
		Vector3::new(0.0, 0.0, 0.0)
	}
	fn scale(&self, s: f64) -> Vector3<f64> {
		Vector3::new(self.x * s, self.y * s, self.z * s)
	}
	fn rem2(self, b: Self) -> Self {
		let mut res = vec3(0.0,0.0,0.0);
		res.x = self.y * b.z - self.z * b.y;
		res.y = self.z * b.x - self.x * b.z;
		res.z = self.x * b.y - self.y * b.x;
		res
	}
}

//impl Mul for Vector3<f64> {
//	type Output = Vector3<f64>;
//	fn mul(self, other: Vector3<f64>) -> Vector3<f64> {
//		let mut res = Vector3::<f64>::default();
//		res.x = self.x * other.x;
//		res.y = self.y * other.y;
//		res.z = self.z * other.z;
//		res
//	}
//}

lazy_static! {
    static ref NUMBER: u32 = 123;
    static ref spheres: Vec<Sphere> = vec![
        Sphere::new(1e5, Vector3::<f64>::new(1e5 + 1., 40.8, 81.6), Vector3::<f64>::default(), Vector3::<f64>::new(0.75, 0.25, 0.25), Material::Diffuse), //left
    	Sphere::new(1e5, Vector3::<f64>::new(-1e5 + 99.,40.8,81.6),Vector3::<f64>::default(),Vector3::<f64>::new(0.25,0.25,0.75), Material::Diffuse),//Rght
    	Sphere::new(1e5, Vector3::<f64>::new(50.,40.8, 1e5),     Vector3::<f64>::default(),Vector3::<f64>::new(0.75,0.75,0.75), Material::Diffuse),//Back
    	Sphere::new(1e5, Vector3::<f64>::new(50.,40.8,-1e5 + 170.), Vector3::<f64>::default(),Vector3::<f64>::default(),           Material::Diffuse),//Frnt
    	Sphere::new(1e5, Vector3::<f64>::new(50., 1e5, 81.6),    Vector3::<f64>::default(),Vector3::<f64>::new(0.75,0.75,0.75),Material::Diffuse),//Botm
    	Sphere::new(1e5, Vector3::<f64>::new(50.,-1e5 + 81.6,81.6),Vector3::<f64>::default(),Vector3::<f64>::new(0.75,0.75,0.75),Material::Diffuse),//Top
    	Sphere::new(16.5,Vector3::<f64>::new(27.,16.5,47.),       Vector3::<f64>::default(),Vector3::<f64>::new(1.,1.,1.).scale(0.999), Material::Specular),//Mirr
    	Sphere::new(16.5,Vector3::<f64>::new(73.,16.5,78.),       Vector3::<f64>::default(),Vector3::<f64>::new(1.,1.,1.).scale(0.999), Material::Refract),//Glas
    	Sphere::new(600., Vector3::<f64>::new(50.,681.6 - 0.27,81.6),Vector3::<f64>::new(12.,12.,12.),  Vector3::<f64>::default(), Material::Diffuse) //Lite
    ];
}

fn erand48() -> f64 {
	let mut rng = rand::thread_rng();
	unsafe {
		rng.gen::<f64>()
	}
}

struct Ray {
	pub origin: Vector3<f64>,
	pub direct: Vector3<f64>,
}

impl Ray {
	pub fn new(origin: Vector3<f64>, direct: Vector3<f64>) -> Self {
		Ray { origin, direct }
	}
}

enum Material {
	Diffuse,
	Specular,
	Refract,
}

struct Sphere {
	radius: f64,
	center: Vector3<f64>,
	emission: Vector3<f64>,
	color: Vector3<f64>,
	material: Material,
}

impl Sphere {
	pub fn new(radius: f64, center: Vector3<f64>, emission: Vector3<f64>, color: Vector3<f64>, material: Material) -> Self {
		Sphere {
			radius,
			center,
			emission,
			color,
			material,
		}
	}
	pub fn intersect(&self, ray: &Ray) -> f64 {
		let mut op = self.center - ray.origin;
		let eps: f64 = 1e-4;
		let b = op.dot(ray.direct);
		let mut det = b * b + self.radius * self.radius - op.dot(op);
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

fn intersect(ray: &Ray, distance: &mut f64, id: &mut usize) -> bool {
	let size = spheres.len();
	*distance = 1e20;
	let inf = *distance;
	for i in (0..size).rev() {
		let d = spheres[i].intersect(ray);
		if d != 0.0 && d < *distance {
			*distance = d;
			*id = i;
		}
	}
	return *distance < inf;
}

fn radiance(ray: &Ray, mut depth: i32) -> Vector3<f64> {
	let mut t = 0_f64;
	let mut id = 0;
	if !intersect(ray, &mut t, &mut id) {
		return Vector3::<f64>::default();
	}
	let obj: &Sphere = &spheres[id];
	let x = ray.origin + ray.direct.scale(t);
	let n = (x - obj.center).normalize();
	let nl = if n.dot(ray.direct) < 0.0 {
		n
	} else {
		n.scale(-1.0)
	};
	let mut f: Vector3<f64> = obj.color;
//	let mut p = std::f64::MIN;
//	for item in f.data().iter() {
//		if item > &p {
//			p = *item;
//		}
//	}
	let p = if f.x > f.y && f.x > f.z {
		f.x
	} else if f.y > f.z {
		f.y
	} else {
		f.z
	};
//	let p = std::cmp::max(f.x, std::cmp::max(f.y, f.z));
	depth += 1;
	if depth > 5 {
		if erand48() < p {
			f = f.scale(1.0 / p);
		} else {
			return obj.emission;
		}
	}
	if depth > 100 {
		return obj.emission;
	}
	match obj.material {
		Material::Diffuse => {
			let r1 = 2.0 * PI * erand48();
			let r2 = erand48();
			let r2s = r2.sqrt();
			let w = nl;
			let u = ((if w.x.abs() > 0.1 {
				Vector3::<f64>::new(0.0, 1.0, 0.0)
			} else {
				Vector3::<f64>::new(1.0, 0.0, 0.0)
			}).rem2(w)).normalize();
			let v = w.rem2(u);
//			Vec d=(u*Math.Cos(r1)*r2s+v*Math.Sin(r1)*r2s+w*Math.Sqrt(1-r2)).norm();
			let d = (u.scale(r1.cos() * r2s) +
				v.scale(r1.sin() * r2s) +
				w.scale((1.0 - r2).sqrt())).normalize();
			return obj.emission + f.mul_element_wise(radiance(&Ray::new(x, d), depth));
		}
		Material::Specular => {
			return obj.emission + f
				.mul_element_wise(radiance(&Ray::new(x, ray.direct - n.scale(2.0 * n.dot(ray.direct))), depth))
			;
		}
		Material::Refract => {
			let reflRay = Ray::new(x, ray.direct - n.scale(2_f64 * n.dot(ray.direct)));
			let into = n.dot(nl) > 0_f64;
			let nc = 1_f64;
			let nt = 1.5_f64;
			let nnt = if into { nc / nt } else { nt / nc };
			let ddn = ray.direct.dot(nl);
			let cos2t = 1_f64 - nnt * nnt * (1_f64 - ddn - ddn);
			if cos2t < 0_f64 {
				return obj.emission + f.mul_element_wise(radiance(&reflRay, depth));
			}
			let tdir = (ray.direct.scale(nnt) - n.scale((if into { 1.0 } else { -1.0 }) * (ddn * nnt + cos2t.sqrt()))).normalize();
			let a = nt - nc;
			let b = nt + nc;
			let R0 = a * a / (b * b);
			let c = 1.0 - (if into { -ddn } else { tdir.dot(n) });
			let Re = R0 + (1.0 - R0) * c * c * c * c * c;
			let Tr = 1.0 - Re;
			let P = 0.25 + 0.5 * Re;
			let RP = Re / P;
			let TP = Tr / (1.0 - p);
			return obj.emission + f.mul_element_wise((if depth > 2 {
				if erand48() < P {
					radiance(&reflRay, depth).scale(RP)
				} else {
					radiance(&Ray::new(x, tdir), depth).scale(TP)
				}
			} else {
				radiance(&reflRay, depth).scale(Re) + (radiance(&Ray::new(x, tdir), depth).scale(Tr))
			}));
		}
	}
}

fn clamp(x: f64) -> f64 {
	if x < 0.0 {
		0.0
	} else if x > 1.0 {
		1.0
	} else {
		x
	}
}

fn toInt(x: f64) -> i32 { return (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i32; }

fn main() {
	let t1 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
	let text = "Hello, World!";
	let width = 256;
	let height = 256;
	let samples = 25;

	let camera = Ray::new(
		Vector3::<f64>::new(50.0, 52.0, 295.6),
		Vector3::<f64>::new(0.0, -0.042612, -1.0).normalize(),
	);
	let mut c: Vec<Vector3<f64>> = vec![Vector3::<f64>::default(); width * height];
	let cx = Vector3::<f64>::new(width as f64 * 0.5135 / height as f64, 0.0, 0.0);
	let cy = (cx.rem2(camera.direct)).normalize().scale(0.5135);

	for y in 0..height {
		for x in 0..width {
			for sy in 0..2 {
				for sx in 0..2 {
					let i = (height - y - 1) * width + x;
					let mut r = Vector3::<f64>::default();
					for s in 0..samples {
						let r1 = 2.0 * erand48();
						let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
						let r2 = 2.0 * erand48();
						let dy = if r2 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
						let d = cx.scale(((sx as f64 + 0.5 + dx) / 2.0 + x as f64) / width as f64 - 0.5) +
							cy.scale(((sy as f64 + 0.5 + dy) / 2.0 + y as f64) / height as f64 - 0.5) + camera.direct;
						r = r + radiance(&Ray::new(camera.origin + d.scale(140.0), d.normalize()), 0)
							.scale(1.0 / samples as f64);
					}

					c[i] = c[i] + Vector3::<f64>::new(clamp(r.x), clamp(r.y), clamp(r.z)).scale(0.25);
				}
			}
		}
	}

	let mut res = "".to_string();
	res += &format!("P3\r\n{} {}\r\n{}\r\n", width, height, 255);
	for i in 0..(width * height) {
		res += &format!("{} {} {}\r\n", toInt(c[i].x), toInt(c[i].y), toInt(c[i].z));
	}
	std::fs::write("image.ppm", res);

	let t2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
	println!("Hello, world!,{}", t2 - t1);
}
