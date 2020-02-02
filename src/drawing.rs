use crate::Ray;
use crate::CAMERA;
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

pub fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let center = Vec3::new(0., 0., -1.);
    if let Some(hit) = world.hit(r, 0.0, 2e9) {
        0.5 * (hit.normal + Vec3::one())
    } else {
        sky_color(r)
    }
}

pub struct Hit {
    t: f32,
    point: Vec3,
    normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let v = CAMERA - self.center;
        let a = r.direction().dot(*r.direction());
        let b = v.dot(*r.direction());
        let c = v.dot(v) - self.radius * self.radius;
        let disc = b * b - a * c;

        if disc > 0.0 {
            let t = {
                let temp = (-b - (b*b - a*c).sqrt())/a;
                if temp < t_max && temp > t_min {
                    Some(temp)
                } else {
                    let temp = (-b + (b*b - a*c).sqrt())/a;
                    if temp < t_max && temp > t_min {
                        Some(temp)
                    } else {
                        None
                    }
                }
            };

            if let Some(t) = t {
                let point = r.point(t);
                Some(Hit {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
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
