#![feature(array_value_iter)]
#![feature(exclusive_range_pattern)]
#![feature(clamp)]
#![feature(const_generics)]

#[macro_use]
extern crate itertools;

use crate::camera::CameraSettings;
use crate::ray::Ray;
use crate::render::Renderer;
use crate::scenes::*;
use crate::window::RenderWindow;
use std::time;
use tiny_rng::{LcRng, Rand};
use ultraviolet::Vec3;

mod aabb;
mod bvh;
mod camera;
mod material;
mod objects;
mod ray;
mod render;
mod scenes;
mod texture;
mod util;
mod window;

fn main() {
    let scene = cornell_box();
    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(278., 278., -800.))
        .look_at(Vec3::new(278., 278., 0.))
        .field_of_view(40.);
    let renderer = Renderer::default()
        .width(300)
        .height(300)
        .samples(300)
        .camera(camera);

    let render = renderer.render(&scene);

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let window = RenderWindow::new(
        "Cornell Box",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
