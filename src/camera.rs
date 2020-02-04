use crate::ray::Ray;
use crate::{HEIGHT, WIDTH};
use ultraviolet::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    position: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3
}

impl Camera {
    pub fn new(cam_pos: Vec3, look_at: Vec3, vup: Vec3, vfov: f32) -> Camera {
        let theta = vfov*PI/180.;

        let w = (cam_pos - look_at).normalized();
        let u = vup.cross(w).normalized();
        let v = w.cross(u);

        let half_height = (theta / 2.0).tan();
        let half_width = half_height * (WIDTH as f32) / (HEIGHT as f32);

        let lower_left = cam_pos - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera {
            position: cam_pos,
            horizontal,
            vertical,
            lower_left
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.position,
            self.lower_left + u * self.horizontal + v * self.vertical - self.position,
        )
    }
}
