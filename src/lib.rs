#![feature(array_value_iter)]
#![feature(exclusive_range_pattern)]
#![feature(clamp)]
#![feature(const_generics)]

#![allow(incomplete_features)]

#[macro_use]
extern crate itertools;

mod aabb;
mod bvh;
mod ray;
mod util;


pub mod camera;
pub mod material;
pub mod objects;
pub mod window;
pub mod render;
pub mod texture;
pub mod scene;
