use std::ops::{Add, Mul};
use tiny_rng::Rand;
use ultraviolet::Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

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
pub struct Coord(pub usize, pub usize);

impl Coord {
    /// `Coord`. Note that this function assumes that as the index increases, the y values decrease
    /// (i.e. `idx = 0` is at the bottom left at (0, height))
    pub fn from_index(idx: usize, width: usize, height: usize) -> Coord {
        Coord(idx % width, height - (idx / width))
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum Axis {
    X = 0usize,
    Y = 1usize,
    Z = 2usize,
}

impl Axis {
    #[inline(always)]
    pub fn other(a: Axis, b: Axis) -> Axis {
        match (a, b) {
            (Axis::X, Axis::Y) | (Axis::Y, Axis::X) => Axis::Z,
            (Axis::Y, Axis::Z) | (Axis::Z, Axis::Y) => Axis::X,
            (Axis::X, Axis::Z) | (Axis::Z, Axis::X) => Axis::Y,
            _ => panic!("Axis::other called, but a == b"),
        }
    }

    #[inline(always)]
    pub fn unit_vec(self) -> Vec3 {
        match self {
            Axis::X => Vec3::unit_x(),
            Axis::Y => Vec3::unit_y(),
            Axis::Z => Vec3::unit_z(),
        }
    }
}

pub fn max_component_idx(vec: Vec3) -> usize {
    if vec.x > vec.y {
        if vec.z > vec.x {
            2
        } else {
            0
        }
    } else {
        if vec.z > vec.y {
            2
        } else {
            1
        }
    }
}

pub fn _lerp<T>(a: T, b: T, t: f32) -> T
where
    T: Add<T, Output = T>,
    T: Mul<f32, Output = T>,
{
    a * (1. - t) + b * t
}

/// A `CoordinateSystem` is described by 3 linearly independent basis vectors
#[derive(Debug)]
pub struct CoordinateSystem {
    // NOTE: The Book is generic -- I don't think that's necessary,
    pub v1: Vec3,
    pub v2: Vec3,
    pub v3: Vec3,
}

impl CoordinateSystem {
    /// Creates a new `CoordinateSystem` from 3 vectors. Does not check for linear independence.
    pub fn _new(v1: Vec3, v2: Vec3, v3: Vec3) -> CoordinateSystem {
        CoordinateSystem { v1, v2, v3 }
    }

    /// Creates a coordinate system from 1 vector. First, `v2` is created by zeroing one of the
    /// components, swapping the other two, and negating one, and `v3` is created from the cross
    /// product of the first two.
    /// `v1` should be normalized before calling this function.
    /// Note that these values are unique only up to rotation around the vector `v1`.
    /// See The PBR Book Section 2.2.4 for more details.
    pub fn _from_one_vec(v1: &Vec3) -> CoordinateSystem {
        let v2 = if v1.x.abs() > v1.y.abs() {
            Vec3::new(-v1.z, 0., v1.x).normalized()
        } else {
            Vec3::new(0., v1.z, -v1.y).normalized()
        };

        let v3 = -v1.cross(v2);

        CoordinateSystem { v1: *v1, v2, v3 }
    }
}
