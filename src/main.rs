#![feature(array_value_iter)]
#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate lazy_static;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::render::color;
use crate::scenes::{two_spheres_perlin, random_scene};
use crate::util::*;
use image::{save_buffer_with_format, ColorType, ImageFormat};
use minifb::{Key, Window, WindowOptions};
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time;
use tiny_rng::{LcRng, Rand};
use ultraviolet::Vec3;

mod aabb;
mod bvh;
mod camera;
mod material;
mod ray;
mod render;
mod scenes;
mod texture;
mod util;

const WIDTH: usize = 480;
const HEIGHT: usize = 270;

const SAMPLES: usize = 25;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let cam_pos = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let camera = Camera::new(
        cam_pos,
        look_at,
        Vec3::unit_y(),
        20.0,
        0.1,
        (cam_pos - look_at).mag(),
    );

    // We're seeding this rng with buffer.len(), because each idx of the buffer is used as the seed
    // for that pixel.
    let mut rng = LcRng::new(buffer.len() as u64);
    let mut world = two_spheres_perlin();

    let root_bvh = BVHNode::new(&mut world);

    let start = time::Instant::now();

    let completed = AtomicUsize::new(0);

    buffer.par_iter_mut().enumerate().for_each(|(idx, pix)| {
        // NOTE: I have no idea if seeding the Rng with the idx is valid.
        let mut rng = LcRng::new(idx as u64);
        let pos: Coord = idx.into();

        let mut total_color = Vec3::zero();

        for _ in 0..SAMPLES {
            let (u, v): (f32, f32) = pos.into_f32s_with_offset(rng.rand_f32(), rng.rand_f32());
            let ray = camera.ray(u, v, &mut rng);
            total_color += color(&ray, &root_bvh, 0, &mut rng);
        }

        total_color /= SAMPLES as f32;
        total_color = total_color.map(f32::sqrt);

        let colori: Color = total_color.into();
        *pix = colori.into();

        let count = completed.fetch_add(1, Ordering::SeqCst);
        if idx % 1000 == 0 {
            println!("Completed {}/{}", count / 1000, WIDTH * HEIGHT / 1000)
        }
    });

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let mut window = Window::new("Firework", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("Window creation failed -- {}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        if window.is_key_released(Key::F3) {
            println!("Saving image to ./render.png");
            let new_buf: Vec<u8> = buffer
                .iter()
                .flat_map(|&x| {
                    std::array::IntoIter::new([
                        ((x & ((1 << 24) - 1)) >> 16) as u8,
                        ((x & ((1 << 16) - 1)) >> 8) as u8,
                        (x & ((1 << 8) - 1)) as u8,
                    ])
                })
                .collect();
            save_buffer_with_format(
                "./render.png",
                &new_buf,
                WIDTH as u32,
                HEIGHT as u32,
                ColorType::RGB(8),
                ImageFormat::PNG,
            )
            .expect("Failed to save to ./render.png");
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
