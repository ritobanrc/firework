#![feature(array_value_iter)]

use crate::camera::Camera;
use crate::drawing::{color, HitableList, LambertianMat, MetalMat, Sphere};
use crate::ray::Ray;
use crate::util::*;
use anyhow::Result;
use minifb::{Key, Window, WindowOptions};
use std::rc::Rc;
use std::time;
use tiny_rng::{LcRng, Rand};
use ultraviolet::Vec3;
use image::{save_buffer_with_format, ColorType, ImageFormat};

mod camera;
mod drawing;
mod ray;
mod util;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const SAMPLES: usize = 100;

fn main() -> Result<()> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let seed: u64 = 0;
    let mut rng = LcRng::new(seed);

    let camera = Camera::new(Vec3::zero(), 4.);

    let mut world = HitableList::new();

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Rc::new(LambertianMat::new(Vec3::new(0.3, 0.8, 0.3))),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Rc::new(LambertianMat::new(Vec3::new(0.8, 0.5, 0.8))),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Rc::new(MetalMat::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Rc::new(MetalMat::new(Vec3::new(0.8, 0.8, 0.8), 0.3)),
    )));

    let start = time::Instant::now();

    for (idx, pix) in buffer.iter_mut().enumerate() {
        let pos: Coord = idx.into();

        let mut total_color = Vec3::zero();

        for _ in 0..SAMPLES {
            let (u, v): (f32, f32) = pos.into_f32s_with_offset(rng.rand_f32(), rng.rand_f32());
            let ray = camera.ray(u, v);
            total_color += color(&ray, &world, 0, &mut rng);
        }

        total_color /= SAMPLES as f32;
        total_color = total_color.map(f32::sqrt);

        let colori: Color = total_color.into();
        *pix = colori.into();

        if idx % 1000 == 0 {
            println!("Completed {}/{}", idx / 1000, WIDTH * HEIGHT / 1000)
        }
    }

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
            let new_buf: Vec<u8> = buffer.iter().flat_map(|&x| {
                std::array::IntoIter::new([((x & ((1 << 24) - 1)) >> 16) as u8,
                                           ((x & ((1 << 16) - 1)) >> 8) as u8,
                                            (x & ((1 << 8) - 1)) as u8])
            }).collect();
            save_buffer_with_format("./render.png", &new_buf, WIDTH as u32, HEIGHT as u32, ColorType::RGB(8), ImageFormat::PNG)?;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    Ok(())
}
