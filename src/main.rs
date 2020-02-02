use anyhow::Result;
use crate::util::*;
use crate::ray::Ray;
use minifb::{Key, Window, WindowOptions};
use ultraviolet::Vec3;

mod ray;
mod util;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;



/// A `Vec3` showing the in-world screen width
const HORIZONTAL: Vec3 = Vec3 { x: 4.0, y: 0.0, z: 0.0 };

/// A `Vec3` showing the in-world screen height.
const VERTICAL: Vec3 = Vec3 { x: 0.0, y: HORIZONTAL.x * HEIGHT as f32 / WIDTH as f32, z: 0.0 };

/// A `Vec3` for the camera location
const CAMERA: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

fn main() -> Result<()> {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let lower_left = CAMERA - (HORIZONTAL + VERTICAL)/2.0 - Vec3::unit_z();

    for (idx, pix) in buffer.iter_mut().enumerate() {
        let pos: Coord = idx.into();
        let (u, v): (f32, f32) = pos.into();
        let ray = Ray::new(CAMERA, lower_left + u * HORIZONTAL + v * VERTICAL);
        let color = sky_color(&ray);
        let colori: Color = color.into();
        *pix = colori.into();
    }


    let mut window = Window::new(
        "Firework",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Window creation failed -- {}", e);
    });


    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));



    while window.is_open() && !window.is_key_down(Key::Escape) {

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }

    Ok(())
}
