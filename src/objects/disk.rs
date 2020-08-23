use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use tiny_rng::LcRng;
use ultraviolet::{Vec2, Vec3};

/// Creates a disk facing upwards with a given radius.
/// The `phi_max` parameter can be used to create a sector with the given angle.
/// The `inner_radius` parameter can be used to create an annulus (2D donut).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Disk {
    radius: f32,
    phi_max: f32,
    inner_radius: f32,
    material: MaterialIdx,
}

impl Disk {
    pub fn new(radius: f32, material: MaterialIdx) -> Disk {
        Disk {
            radius,
            phi_max: 2. * std::f32::consts::PI,
            inner_radius: 0.,
            material,
        }
    }

    pub fn partial(radius: f32, phi: f32, inner_radius: f32, material: MaterialIdx) -> Disk {
        Disk {
            radius,
            phi_max: phi.to_radians(),
            inner_radius,
            material,
        }
    }
}

impl Hitable for Disk {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        // Ignore rays parallel to disk, to avoid divide by zero errors
        if r.direction().y == 0. {
            return None;
        }
        // Solve for t. This is the same thing as
        // x = (y - b)/m, if the ray is a line y = mx + b,
        // exxcept b is 0 (because the ray has already been transformed to object
        // coordinates)
        // This just finds the intersection of the ray and the XZ plane
        let t = -r.origin().y / r.direction().y;
        if t < t_min || t > t_max {
            return None;
        }
        let point = r.point(t);
        // Check if the point on the plane is inside the circle (and outside the inner
        // circle)
        let dist2 = point.x * point.x + point.z * point.z;
        if dist2 > self.radius * self.radius || dist2 < self.inner_radius * self.inner_radius {
            return None;
        }
        let phi = {
            let phi = point.z.atan2(point.x);
            if phi < 0. {
                phi + 2. * std::f32::consts::PI
            } else {
                phi
            }
        };
        if phi > self.phi_max {
            return None;
        }
        let u = phi / self.phi_max;
        let dist = dist2.sqrt();
        let v = 1. - (dist - self.inner_radius) / (self.radius - self.inner_radius);

        Some(RaycastHit {
            t,
            point,
            normal: Vec3::unit_y(),
            material: self.material,
            uv: Vec2::new(u, v),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(-self.radius, 0., self.radius),
            Vec3::new(-self.radius, 0.001, self.radius),
        ))
    }
}
