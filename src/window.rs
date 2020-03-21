use crate::util::Color;
use image::{save_buffer, ColorType};
use minifb::{Key, Window, WindowOptions};
use std::convert::AsRef;
use std::path::Path;

pub struct RenderWindow {
    title: &'static str,
    options: WindowOptions,
    width: usize,
    height: usize,
    fps: u64,
}

impl RenderWindow {
    pub fn new(
        title: &'static str,
        options: WindowOptions,
        width: usize,
        height: usize,
    ) -> RenderWindow {
        RenderWindow {
            title,
            options,
            width,
            height,
            fps: 12,
        }
    }

    pub fn set_fps(&mut self, fps: u64) {
        self.fps = fps;
    }

    pub fn display(&self, render: &[Color]) {
        let buffer: Vec<u32> = render.iter().map(|c| u32::from(*c)).collect();
        let mut window = Window::new(self.title, self.width, self.height, self.options)
            .unwrap_or_else(|e| {
                panic!("Window creation failed -- {}", e);
            });

        window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / self.fps)));

        while window.is_open() && !window.is_key_down(Key::Escape) {
            // TODO: Allow the user to do whatever they want here
            if window.is_key_released(Key::F3) {
                save_image(render, "./render.png", self.width, self.height)
            }
            window
                .update_with_buffer(&buffer, self.width, self.height)
                .unwrap();
        }
    }
}

pub fn save_image<P>(render: &[Color], path: P, width: usize, height: usize)
where
    P: AsRef<Path>,
{
    let new_buf: Vec<u8> = render
        .iter()
        .flat_map(|&x| std::array::IntoIter::new([x.0, x.1, x.2]))
        .collect();
    save_buffer(
        path,
        &new_buf,
        width as u32,
        height as u32,
        ColorType::RGB(8),
    )
    .expect("Failed to save");
}
