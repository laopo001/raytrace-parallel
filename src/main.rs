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
	let obj: &Sphere = &spheres[id];
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
	if depth > 100 {
		return obj.emission;
	}
	match obj.material {
		Material::Diffuse => {
			let r1 = 2.0 * PI * erand48();
			let r2 = erand48();
			let r2s = 2.0_f32.sqrt();
			let w = nl;
			let u = ((if w.x.abs() > 0.1 {
				Vec3::new(0.0, 1.0, 0.0)
			} else {
				Vec3::new(0.0, 0.0, 1.0)
			}) % w).normalize();
			let v = w % u;
//			Vec d=(u*Math.Cos(r1)*r2s+v*Math.Sin(r1)*r2s+w*Math.Sqrt(1-r2)).norm();
			let d = (u.scale(r1.cos() * r2s) +
				v.scale(r1.sin() * r2s) +
				w.scale((1.0 - r2).sqrt())).normalize();
			return obj.emission + f * radiance(&Ray::new(x, d), depth);
		}
		Material::Specular => {
			return obj.emission + f * radiance(&Ray::new(x, ray.direct - n.scale(2.0 * n.dot(&ray.direct)),
			), depth);
		}
		_ => {
			let reflRay = Ray::new(x, ray.direct - n.scale(2_f32 * n.dot(&ray.direct)));
			let into = n.dot(&nl) > 0_f32;
			let nc = 1_f32;
			let nt = 1.5_f32;
			let nnt = if into { nc / nt } else { nt / nc };
			let ddn = ray.direct.dot(&nl);
			let cos2t = 1_f32 - nnt * nnt * (1_f32 - ddn - ddn);
			if cos2t < 0_f32 {
				return obj.emission + f * radiance(&reflRay, depth);
			}
			let tdir = ray.direct.scale(nnt) - n.scale((if into { 1.0 } else { -1.0 }) * (ddn * nnt + cos2t.sqrt())).normalize();
			let a = nt - nc;
			let b = nt + nc;
			let R0 = a * a / (b * b);
			let c = 1.0 - (if into { -ddn } else { tdir.dot(&n) });
			let Re = R0 + (1.0 - R0) * c * c * c * c * c;
			let Tr = 1.0 - Re;
			let P = 0.25 + 0.5 * Re;
			let RP = Re / P;
			let TP = Tr / (1.0 - p);
			return obj.emission + f * (if depth > 2 {
				if erand48() < P {
					radiance(&reflRay, depth).scale(RP)
				} else {
					radiance(&Ray::new(x, tdir), depth).scale(TP)
				}
			} else {
				radiance(&reflRay, depth).scale(Re) + radiance(&Ray::new(x, tdir), depth).scale(Tr)
			});
		}
	}
}

fn clamp(x: f32) -> f32 {
	if x < 0.0 {
		0.0
	} else if x > 1.0 {
		1.0
	} else {
		x
	}
}

fn toInt(x: f32) -> i32 { return (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i32; }

fn main() {
	let mut rng = rand::thread_rng();
	let width = 256;
	let height = 256;
	let samples = 25;
	let mut direct = Vec3::new(0.0, -0.042612, -1.0);
	direct.normalize();
	let camera = Ray::new(
		Vec3::new(50.0, 52.0, 295.6),
		direct,
	);
	let mut c: Vec<Vec3> = vec![Vec3::default(); width * height];
	let cx = Vec3::new(width as f32 * 0.5135 / height as f32, 0.0, 0.0);
	let mut cy = (cx * camera.direct).normalize().scale(0.5135);

	for y in 0..height {
		for x in 0..width {
			for sy in 0..2 {
				for sx in 0..2 {
					let i = (height - y - 1) * width + x;
					let mut r = Vec3::default();
					for s in 0..samples {
						let r1 = 2.0 * erand48();
						let dx = if r1 < 1.0 { r1.sqrt() - 1.0 } else { 1.0 - (2.0 - r1).sqrt() };
						let r2 = 2.0 * erand48();
						let dy = if r1 < 1.0 { r2.sqrt() - 1.0 } else { 1.0 - (2.0 - r2).sqrt() };
						let d = cx.scale(((sx as f32 + 0.5 + dx) / 2.0 + x as f32) / width as f32 - 0.5) +
							cy.scale(((sy as f32 + 0.5 + dy) / 2.0 + y as f32) / height as f32 - 0.5) + camera.direct;
						r = r + radiance(&Ray::new(camera.origin + d.scale(140.0), d.normalize()), 0)
							.scale(1.0 / samples as f32);
					}
					c[i] = c[i] + Vec3::new(clamp(r.x), clamp(r.y), clamp(r.z)).scale(0.25);
				}
			}
		}
	}

	let mut res = "".to_string();
	for i in 0..(width * height) {
		res += &format!("{} {} {}", toInt(c[i].x), toInt(c[i].y), toInt(c[i].z));
	}
	std::fs::write("image.ppm", res);


	println!("Hello, world!,{}", rng.gen::<f32>());
}
