use crate::ray::Ray;
use crate::util::random_in_unit_disk;
use crate::{HEIGHT, WIDTH};
use ultraviolet::Vec3;
use std::f32::consts::PI;
use tiny_rng::Rand;

pub struct Camera {
    position: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3, 
    u: Vec3, 
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(cam_pos: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov*PI/180.;

        let w = (cam_pos - look_at).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let half_height = (theta / 2.0).tan();
        let half_width = half_height * (WIDTH as f32) / (HEIGHT as f32);

        let lower_left = cam_pos - half_width * focus_dist * u - half_height * focus_dist * v - w * focus_dist;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            position: cam_pos,
            horizontal,
            vertical,
            lower_left,
            u, v, w,
            lens_radius: aperture / 2.
        }
    }

    pub fn ray(&self, s: f32, t: f32, rand: &mut impl Rand) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rand);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.position + offset,
            self.lower_left + s * self.horizontal + t * self.vertical - self.position - offset,
        )
    }
}
