#![feature(array_value_iter)]
#![feature(exclusive_range_pattern)]
#![feature(clamp)]
#![feature(const_generics)]

#[macro_use]
extern crate itertools;

use crate::ray::Ray;
use crate::scenes::*;
use crate::render::Renderer;
use crate::camera::CameraSettings;
use image::{save_buffer_with_format, ColorType, ImageFormat};
use minifb::{Key, Window, WindowOptions};
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

fn main() {
    let scene = cornell_box();
    let start = time::Instant::now();


    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(278., 278., -800.))
        .look_at(Vec3::new(278., 278., 0.))
        .field_of_view(40.);
    let renderer = Renderer::default()
        .width(500)
        .height(500)
        .samples(500)
        .camera(camera);

    let render = renderer.render(&scene);

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let buffer: Vec<u32> = render.iter().map(|c| u32::from(*c)).collect();

    let mut window = Window::new("Firework", renderer.width, renderer.height, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed -- {}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        if window.is_key_released(Key::F3) {
            println!("Saving image to ./render.png");
            let new_buf: Vec<u8> = render
                .iter()
                .flat_map(|&x| {
                    std::array::IntoIter::new([
                                              x.0, x.1, x.2
                        //((x & ((1 << 24) - 1)) >> 16) as u8,
                        //((x & ((1 << 16) - 1)) >> 8) as u8,
                        //(x & ((1 << 8) - 1)) as u8,
                    ])
                })
                .collect();
            save_buffer_with_format(
                "./render.png",
                &new_buf,
                renderer.width as u32,
                renderer.height as u32,
                ColorType::RGB(8),
                ImageFormat::PNG,
            )
            .expect("Failed to save to ./render.png");
        }
        window.update_with_buffer(&buffer, renderer.width, renderer.height).unwrap();
    }
}
