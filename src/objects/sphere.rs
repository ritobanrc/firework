use crate::aabb::AABB;
use crate::objects::solve_quadratic;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use tiny_rng::LcRng;
use ultraviolet::{Vec2, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    radius: f32,
    material: MaterialIdx,
}

impl Sphere {
    pub fn new(radius: f32, material: MaterialIdx) -> Sphere {
        Sphere { radius, material }
    }
}

pub fn sphere_uv(point: &Vec3) -> Vec2 {
    use std::f32::consts::PI;
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    Vec2::new(u, v)
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let o = *r.origin();
        let d = *r.direction();
        let a = d.dot(d);
        let b = 2. * o.dot(d);
        let c = o.dot(o) - self.radius * self.radius;

        if let [Some(t1), t2] = solve_quadratic(a, b, c) {
            let t = if t1 < t_max && t1 > t_min {
                t1
            } else {
                match t2 {
                    Some(t2) if t2 < t_max && t2 > t_min => t2,
                    _ => return None,
                }
            };

            let point = r.point(t);
            Some(RaycastHit {
                t,
                point,
                normal: point / self.radius,
                material: self.material,
                uv: sphere_uv(&(point / self.radius)),
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            -Vec3::one() * self.radius,
            Vec3::one() * self.radius,
        ))
    }
}
