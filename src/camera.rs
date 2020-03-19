use crate::ray::Ray;
use crate::util::random_in_unit_disk;
use std::f32::consts::PI;
use tiny_rng::Rand;
use ultraviolet::Vec3;

pub struct Camera {
    position: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

pub struct CameraSettings {
    cam_pos: Vec3,
    look_at: Vec3,
    vfov: f32,
    aperture: f32,
    focus_dist: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        CameraSettings {
            cam_pos: Vec3::new(0., 0., -10.),
            look_at: Vec3::zero(),
            vfov: 30.,
            aperture: 0.0,
            focus_dist: 10.,
        }
    }
}

impl CameraSettings {
    pub fn create_camera(&self, width: usize, height: usize) -> Camera {
        Camera::new(self.cam_pos, self.look_at, self.vfov, self.aperture, self.focus_dist, width, height)
    }

    pub fn cam_pos(mut self, cam_pos: Vec3) -> CameraSettings {
        self.cam_pos = cam_pos;
        self
    }
    pub fn look_at(mut self, look_at: Vec3) -> CameraSettings {
        self.look_at = look_at;
        self
    }
    pub fn field_of_view(mut self, vfov: f32) -> CameraSettings {
        self.vfov = vfov;
        self
    }
    pub fn aperture(mut self, aperture: f32) -> CameraSettings {
        self.aperture = aperture;
        self
    }
    pub fn focus_dist(mut self, focus_dist: f32) -> CameraSettings {
        self.focus_dist = focus_dist;
        self
    }
}

impl Camera {
    pub fn new(
        cam_pos: Vec3,
        look_at: Vec3,
        vfov: f32,
        aperture: f32,
        focus_dist: f32,
        width: usize,
        height: usize,
    ) -> Camera {
        let theta = vfov * PI / 180.;

        let w = (cam_pos - look_at).normalized();
        let u = Vec3::unit_y().cross(w).normalized();
        let v = w.cross(u);

        let half_height = (theta / 2.0).tan();
        let half_width = half_height * (width as f32) / (height as f32);

        let lower_left =
            cam_pos - half_width * focus_dist * u - half_height * focus_dist * v - w * focus_dist;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            position: cam_pos,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            _w: w,
            lens_radius: aperture / 2.,
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
