use crate::util::{random_in_unit_sphere, reflect};
use crate::Ray;
use std::rc::Rc;
use tiny_rng::LcRng;
use ultraviolet::Vec3;

const SKY_BLUE: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};
const SKY_WHITE: Vec3 = Vec3 {
    x: 1.,
    y: 1.,
    z: 1.,
};

fn sky_color(r: &Ray) -> Vec3 {
    let dir = r.direction().normalized();
    // Take the y (from -1 to +1) and map it to 0..1
    let t = 0.5 * (dir.y + 1.0);
    (1. - t) * SKY_WHITE + t * SKY_BLUE
}

pub fn color(r: &Ray, world: &HitableList, depth: usize, rand: &mut LcRng) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, 2e9) {
        if depth < 50 {
            if let Some(result) = hit.material.as_ref().scatter(r, &hit, rand) {
                result.attenuation * color(&result.scattered, world, depth + 1, rand)
            } else {
                Vec3::zero()
            }
        } else {
            Vec3::zero()
        }
    } else {
        sky_color(r)
    }
}

pub struct Hit {
    t: f32,
    point: Vec3,
    normal: Vec3,
    material: Rc<dyn Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let v = *r.origin() - self.center;
        let a = r.direction().dot(*r.direction());
        let b = v.dot(*r.direction());
        let c = v.dot(v) - self.radius * self.radius;
        let disc = b * b - a * c;

        if disc > 0.0 {
            fn roots(a: f32, b: f32, c: f32, t_max: f32, t_min: f32) -> Option<f32> {
                let lhs = -b;
                let rhs = (b * b - a * c).sqrt();
                let temp = (lhs - rhs) / a;
                if temp < t_max && temp > t_min {
                    return Some(temp);
                }
                let temp = (lhs + rhs) / a;
                if temp < t_max && temp > t_min {
                    return Some(temp);
                }

                None
            }

            if let Some(t) = roots(a, b, c, t_max, t_min) {
                let point = r.point(t);
                Some(Hit {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: Rc::clone(&self.material),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct HitableList(Vec<Box<dyn Hitable>>);

impl HitableList {
    pub fn new() -> HitableList {
        HitableList(Vec::new())
    }

    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut last_hit = None;
        let mut closest = t_max;
        for hitable in self.0.iter() {
            let new_hit = hitable.hit(r, t_min, closest);
            if let Some(hit) = new_hit {
                closest = hit.t;
                last_hit = Some(hit);
            }
        }
        last_hit
    }

    pub fn list(&self) -> &Vec<Box<dyn Hitable>> {
        &self.0
    }

    pub fn list_mut(&mut self) -> &mut Vec<Box<dyn Hitable>> {
        &mut self.0
    }
}

pub struct ScatterResult {
    attenuation: Vec3,
    scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &Hit, rand: &mut LcRng) -> Option<ScatterResult>;
}

pub struct LambertianMat {
    albedo: Vec3,
}

impl LambertianMat {
    pub fn new(albedo: Vec3) -> LambertianMat {
        LambertianMat {
            albedo,
        }
    }
}

impl Material for LambertianMat {
    fn scatter(&self, _r_in: &Ray, hit: &Hit, rand: &mut LcRng) -> Option<ScatterResult> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rand);
        let scattered = Ray::new(hit.point, target - hit.point);
        let attenuation = self.albedo;
        Some(ScatterResult {
            scattered,
            attenuation,
        })
    }
}

pub struct MetalMat {
    albedo: Vec3,
}

impl MetalMat {
    pub fn new(albedo: Vec3) -> MetalMat {
        MetalMat { albedo }
    }
}

impl Material for MetalMat {
    fn scatter(&self, r_in: &Ray, hit: &Hit, _rand: &mut LcRng) -> Option<ScatterResult> {
        let reflected = reflect(r_in.direction(), &hit.normal);
        let scattered = Ray::new(hit.point, reflected);
        let attenuation = self.albedo;
        if scattered.direction().dot(hit.normal) > 0. {
            Some(ScatterResult {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}
