#![feature(array_value_iter)]
#![feature(exclusive_range_pattern)]
#![feature(clamp)]
#![feature(const_generics)]
#![allow(incomplete_features)]

mod aabb;
mod bvh;
mod ray;
mod serde_compat;
mod util;

pub mod camera;
pub mod environment;
pub mod material;
pub mod objects;
pub mod render;
pub mod scene;
pub mod texture;
pub mod window;

pub use crate::render::Renderer;
pub use crate::scene::{RenderObject, Scene};
pub use crate::window::RenderWindow;
