#![feature(exclusive_range_pattern)]
#![feature(generic_const_exprs)]
#![feature(adt_const_params)]
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
