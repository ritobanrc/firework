use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use crate::objects::solve_quadratic;
use tiny_rng::LcRng;
use ultraviolet:: Vec3;


pub struct Cone {
    radius: f32,
    height: f32,
    material: MaterialIdx,
}

impl Cone {
    pub fn new(radius: f32, height: f32, material: MaterialIdx) -> Cone {
        Cone { radius, height, material }
    }
}

impl Hitable for Cone {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let o = *r.origin();
        let d = *r.direction();
        // Derivation (using Sympy)
        // > expr = (h*(ox + t*dx)/r)**2 + (h*(oy + t*dy)/r)**2 - (oz + t*dz - h)**2
        // > collected = collect(expand(expr), t)
        // > collected.coeff(t, 2)
        //      2  2     2  2
        //    dx ⋅h    dy ⋅h      2
        //    ────── + ────── - dz
        //       2        2
        //      r        r
        // > collected.coeff(t, 1)
        //          2            2
        //    2⋅dx⋅h ⋅ox   2⋅dy⋅h ⋅oy
        //    ────────── + ────────── + 2⋅dz⋅h - 2⋅dz⋅oz
        //         2            2
        //        r            r
        // > collected.coeff(t, 0)
        //     2   2    2   2
        //    h ⋅ox    h ⋅oy     2              2
        //    ────── + ────── - h  + 2⋅h⋅oz - oz
        //       2        2
        //      r        r
        // Note that y and z are switched (because in the equation, z is the up direction, while
        // in the renderer, y is).
        let r2_div_h2 = self.radius * self.radius / (self.height * self.height);
        let a = d.x * d.x + d.z * d.z  - r2_div_h2 * d.y * d.y;
        let b = 2. * (d.x * o.x + d.z * o.z - r2_div_h2 * d.y * (o.y - self.height));
        let c = o.x * o.x + o.z * o.z - r2_div_h2 * (o.y - self.height) * (o.y - self.height);

        if let [Some(t1), t2] = solve_quadratic(a, b, c) {
            let check_solution = |t| {
                if t > t_max || t < t_min {
                    return None
                }
                let point = r.point(t);
                if point.y < 0. || point.y > self.height {
                    return None
                }
                let v = point.y / self.height;
                let phi = (point.x / (self.radius * (1. - v))).acos();
                let u = phi / (2. * std::f32::consts::PI);
                let dpdu = Vec3::new(-point.z, 0., point.x);
                let dpdv = Vec3::new(
                    -point.x/(1. - v),
                    self.height,
                    -point.z / (1. - v),
                    );
                return Some(RaycastHit {
                    t,
                    point,
                    normal: dpdv.cross(dpdu).normalized(),
                    material: self.material,
                    uv: (u, v),
                })
            };

            if let Some(hit) = check_solution(t1) {
                return Some(hit);
            } else if let Some(t2) = t2 {
                return check_solution(t2)          
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
