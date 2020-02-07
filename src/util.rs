use crate::drawing::HitableList;
use crate::{HEIGHT, WIDTH};
use tiny_rng::Rand;
use ultraviolet::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(u8, u8, u8);

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        (color.0 as u32) << 16 | (color.1 as u32) << 8 | (color.2 as u32)
    }
}

impl From<Vec3> for Color {
    /// Creates a `Color` from 3 rgb float values in the range 0..1
    fn from(c: Vec3) -> Color {
        Color(
            (c.x * 255.99) as u8,
            (c.y * 255.99) as u8,
            (c.z * 255.99) as u8,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(usize, usize);

impl Coord {
    pub fn into_f32s_with_offset(&self, xoff: f32, yoff: f32) -> (f32, f32) {
        (
            (self.0 as f32 + xoff) / WIDTH as f32,
            (self.1 as f32 + yoff) / HEIGHT as f32,
        )
    }
}

impl From<usize> for Coord {
    /// Using the global `const`s WIDTH and HEIGHT, convert an index (x + (HEIGHT - y) * WIDTH) back an (x, y)
    /// `Coord`. Note that this function assumes that as the index increases, the y values decrease
    /// (i.e. `idx = 0` is at the bottom left at (0, HEIGHT))
    fn from(idx: usize) -> Coord {
        Coord(idx % WIDTH, HEIGHT - (idx / WIDTH))
    }
}

impl Into<(f32, f32)> for Coord {
    /// Converts the coord into a (f64, f64) tuple, where both values are in the range 0..1
    fn into(self) -> (f32, f32) {
        (self.0 as f32 / WIDTH as f32, self.1 as f32 / HEIGHT as f32)
    }
}

pub(crate) fn random_in_unit_sphere(rng: &mut impl Rand) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rng.rand_f32(), rng.rand_f32(), rng.rand_f32()) - Vec3::one();
        if p.mag_sq() < 1.0 {
            return p;
        }
    }
}

pub(crate) fn random_in_unit_disk(rng: &mut impl Rand) -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rng.rand_f32(), rng.rand_f32(), 0.) - Vec3::new(1., 1., 0.);
        if p.dot(p) < 1. {
            return p;
        }
    }
}

pub(crate) fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * v.dot(*n) * *n
}

pub(crate) fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalized();
    let dt = uv.dot(*n);
    let disc = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if disc > 0. {
        Some(ni_over_nt * (uv - *n * dt) - *n * disc.sqrt())
    } else {
        None
    }
}

pub(crate) fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

pub trait InRange {
    fn in_range(self, begin: Self, end: Self) -> bool;
}

impl InRange for f32 {
    fn in_range(self, begin: f32, end: f32) -> bool {
        self >= begin && self < end
    }
}
