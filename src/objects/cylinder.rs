use crate::aabb::AABB;
use crate::objects::solve_quadratic;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use tiny_rng::LcRng;
use ultraviolet::{Vec2, Vec3};

/// A vertically oriented cylinder, with a given radius and height
pub struct Cylinder {
    radius: f32,
    height: f32,
    max_phi: f32,
    material: MaterialIdx,
}

impl Cylinder {
    /// Creates a cylinder with the given radius and height
    pub fn new(radius: f32, height: f32, material: MaterialIdx) -> Self {
        Cylinder {
            radius,
            height,
            material,
            max_phi: 360f32.to_radians(),
        }
    }

    /// Creates a cylinder with the given radius and height, that only goes around for `phi` degrees.
    pub fn partial(radius: f32, height: f32, phi: f32, material: MaterialIdx) -> Self {
        Cylinder {
            radius,
            height,
            material,
            max_phi: phi.to_radians(),
        }
    }
}

impl Hitable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let o = r.origin();
        let d = r.direction();
        let a = d.x * d.x + d.z * d.z;
        let b = 2. * (d.x * o.x + d.z * o.z);
        let c = o.x * o.x + o.z * o.z - self.radius * self.radius;

        let disc = b * b - 4. * a * c;
        if disc > 0.0 {
            if let [Some(t1), t2] = solve_quadratic(a, b, c) {
                // define a closure to check if any t results in a hit
                let check_solution = |t| {
                    if t > t_max || t < t_min {
                        return None; // this is returning from teh closure
                    }
                    let point = r.point(t);
                    let phi = {
                        let phi = point.z.atan2(point.x);
                        if phi < 0. {
                            phi + std::f32::consts::PI * 2.
                        } else {
                            phi
                        }
                    };
                    if point.y > 0. && point.y < self.height && phi < self.max_phi {
                        let u = phi / self.max_phi;
                        let v = point.y / self.height;
                        //let dpdu = Vec3::new(-self.max_phi * point.z, 0., self.max_phi * point.x);
                        //let dpdv = self.height * Vec3::unit_y();
                        Some(RaycastHit {
                            t,
                            point,
                            //normal: dpdu.cross(dpdv).normalized(),
                            normal: Vec3::new(point.x / self.radius, 0., point.z / self.radius),
                            material: self.material,
                            uv: Vec2::new(u, v),
                        })
                    } else {
                        None
                    }
                };
                if let Some(hit) = check_solution(t1) {
                    return Some(hit);
                } else if let Some(t2) = t2 {
                    return check_solution(t2);
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(-self.radius, 0., -self.radius),
            Vec3::new(self.radius, self.height, self.radius),
        ))
    }
}
