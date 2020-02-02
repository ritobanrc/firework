use crate::ray::Ray;
use crate::{HEIGHT, WIDTH};
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
