use crate::ray::Ray;
use crate::{HEIGHT, WIDTH};
use ultraviolet::Vec3;

pub struct Camera {
    position: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, visible_width: f32) -> Camera {
        let horizontal = Vec3::new(visible_width, 0., 0.);
        let vertical = Vec3::new(0., visible_width * HEIGHT as f32 / WIDTH as f32, 0.);

        Camera {
            position,
            horizontal,
            vertical,
        }
    }

    fn lower_left(&self) -> Vec3 {
        self.position - (self.horizontal + self.vertical) / 2.0 - Vec3::unit_z()
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.position,
            self.lower_left() + u * self.horizontal + v * self.vertical,
        )
    }
}
