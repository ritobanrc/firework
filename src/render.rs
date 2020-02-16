use crate::aabb::AABB;
use crate::material::Material;
use crate::util::{sphere_uv, Axis};
use crate::Ray;
use tiny_rng::LcRng;
use ultraviolet::{Vec2, Vec3};

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

pub fn color(r: &Ray, world: &dyn Hitable, depth: usize, rand: &mut LcRng) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, 2e9) {
        if depth < 4 {
            if let Some(result) = hit.material.scatter(r, &hit, rand) {
                if result.scattered.direction().mag_sq() < 0.01 {
                    result.attenuation
                } else {
                    result.attenuation * color(&result.scattered, world, depth + 1, rand)
                }
            } else {
                Vec3::zero()
            }
        } else {
            Vec3::zero()
        }
    } else {
        //sky_color(r)
        Vec3::zero()
    }
}

pub struct RaycastHit<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub uv: (f32, f32),
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RaycastHit>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RaycastHit<'_>> {
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
                Some(RaycastHit {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.as_ref(),
                    uv: sphere_uv(&((point - self.center) / self.radius)),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::one() * self.radius,
            self.center + Vec3::one() * self.radius,
        ))
    }
}

pub type XYRect = AARect<{ Axis::X }, { Axis::Y }>;
pub type YZRect = AARect<{ Axis::Y }, { Axis::Z }>;
pub type XZRect = AARect<{ Axis::X }, { Axis::Z }>;

pub struct AARect<const A1: Axis, const A2: Axis> {
    min: Vec2,
    max: Vec2,
    k: f32,
    material: Box<dyn Material + Sync>,
}

impl<const A1: Axis, const A2: Axis> AARect<{ A1 }, { A2 }> {
    pub fn new(
        a1_min: f32,
        a1_max: f32,
        a2_min: f32,
        a2_max: f32,
        k: f32,
        material: Box<dyn Material + Sync>,
    ) -> Self {
        AARect {
            min: Vec2::new(a1_min, a2_min),
            max: Vec2::new(a1_max, a2_max),
            k,
            material,
        }
    }
}

impl<const A1: Axis, const A2: Axis> Hitable for AARect<{ A1 }, { A2 }> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RaycastHit> {
        let t = (self.k - r.origin()[Axis::other(A1, A2) as usize])
            / r.direction()[Axis::other(A1, A2) as usize];
        if t < t_min || t > t_max {
            return None;
        }
        let point = r.point(t);
        if point[A1 as usize] < self.min.x
            || point[A1 as usize] > self.max.x
            || point[A2 as usize] < self.min.y
            || point[A2 as usize] > self.max.y
        {
            return None;
        }
        Some(RaycastHit {
            t,
            point,
            normal: Axis::other(A1, A2).unit_vec(),
            material: self.material.as_ref(),
            uv: (
                (point[A1 as usize] - self.min.x) / (self.max.x - self.min.x),
                (point[A2 as usize] - self.min.y) / (self.max.y - self.min.y),
            ),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.min.x, self.min.y, self.k),
            Vec3::new(self.max.x, self.max.y, self.k),
        ))
    }
}

pub struct HitableList(Vec<Box<dyn Hitable + Sync>>);

impl HitableList {
    pub fn new() -> HitableList {
        HitableList(Vec::new())
    }

    pub fn _list(&self) -> &Vec<Box<dyn Hitable + Sync>> {
        &self.0
    }

    pub fn list_mut(&mut self) -> &mut Vec<Box<dyn Hitable + Sync>> {
        &mut self.0
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<RaycastHit> {
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

    fn bounding_box(&self) -> Option<AABB> {
        let mut result: Option<AABB> = None;
        for hitable in self.0.iter() {
            let next_box = hitable.bounding_box();
            if let Some(next_box) = next_box {
                if let Some(aabb) = result {
                    result = Some(aabb.expand(&next_box));
                } else {
                    result = Some(next_box);
                }
            }
        }
        result
    }
}
