use crate::aabb::AABB;
use crate::camera::{Camera, CameraSettings};
use crate::ray::Ray;
use crate::scene::{MaterialIdx, Scene};
use crate::util::Color;
use std::sync::atomic::{AtomicUsize, Ordering};
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec2, Vec3};

/// Performs the ray tracing for a given ray in the world and returns it's color.
/// TODO: Solve the inconsistency between `scene` and `bvh_root` arguments
pub fn color(r: &Ray, scene: &Scene, root: &impl Hitable, depth: usize, rand: &mut LcRng) -> Vec3 {
    if let Some(hit) = root.hit(r, 0.001, 2e9, rand) {
        let emit = scene.get_material(hit.material).emit(hit.uv, &hit.point);
        if depth < 10 {
            if let Some(result) = scene.get_material(hit.material).scatter(r, &hit, rand) {
                emit + result.attenuation * color(&result.scattered, scene, root, depth + 1, rand)
            } else {
                emit
            }
        } else {
            emit
        }
    } else {
        (scene.environment)(r.direction().normalized())
    }
}

pub struct RaycastHit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: MaterialIdx,
    pub uv: Vec2,
}

/// Trait that allows something to be ray-tracing, i.e. something that can be hit by a ray.
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub struct Renderer {
    /// The width of the render (in pixels)
    pub width: usize,
    /// The height of the render (in pixels)
    pub height: usize,
    /// The number of samples for each pixel
    pub samples: usize,
    /// If true, this will use rayon for multithreading
    /// TODO: Make this a cargo feature or something, so we don't pull rayon in as a dependency
    /// unless we must to
    pub multithreaded: bool,
    /// Whether or not to us a bounding volume hierarchy. Recommended only for scenes with a
    /// large number of objects
    pub use_bvh: bool,
    /// The gamma correction applied, i.e. the output from the renderer is raised to the 1/gamma power before returning
    pub gamma: f32,
    /// The settings to create the camera
    camera: CameraSettings,
}

impl Renderer {
    pub fn width(mut self, width: usize) -> Renderer {
        self.width = width;
        self
    }
    pub fn height(mut self, height: usize) -> Renderer {
        self.height = height;
        self
    }
    pub fn samples(mut self, samples: usize) -> Renderer {
        self.samples = samples;
        self
    }
    pub fn multithreaded(mut self, multithreaded: bool) -> Renderer {
        self.multithreaded = multithreaded;
        self
    }
    pub fn use_bvh(mut self, use_bvh: bool) -> Renderer {
        self.use_bvh = use_bvh;
        self
    }
    pub fn gamma(mut self, gamma: f32) -> Renderer {
        self.gamma = gamma;
        self
    }
    pub fn camera(mut self, settings: CameraSettings) -> Renderer {
        self.camera = settings;
        self
    }

    pub fn render(&self, scene: &Scene) -> Vec<Color> {
        use crate::bvh::BVHNode;
        use rayon::prelude::*;

        let mut buffer = vec![Color(0, 0, 0); self.width * self.height];

        let bvh = if self.use_bvh {
            Some(BVHNode::new(scene))
        } else {
            None
        };

        let camera = self.camera.create_camera(self.width, self.height);

        if self.multithreaded {
            let completed = AtomicUsize::new(0);
            buffer.par_iter_mut().enumerate().for_each(|(idx, pix)| {
                if let Some(bvh) = &bvh {
                    *pix = self.render_pixel(scene, bvh, &camera, idx)
                } else {
                    *pix = self.render_pixel(scene, scene, &camera, idx)
                }
                let count = completed.fetch_add(1, Ordering::SeqCst);
                if count % 10000 == 0 {
                    println!(
                        "Completed {}/{}",
                        count / 10000,
                        self.width * self.height / 10000
                    )
                }
            })
        } else {
            buffer.iter_mut().enumerate().for_each(|(idx, pix)| {
                if let Some(bvh) = &bvh {
                    *pix = self.render_pixel(scene, bvh, &camera, idx)
                } else {
                    *pix = self.render_pixel(scene, scene, &camera, idx)
                }

                if idx % 10000 == 0 {
                    println!(
                        "Completed {}/{}",
                        idx / 10000,
                        self.width * self.height / 10000
                    )
                }
            })
        }

        buffer
    }

    fn render_pixel(
        &self,
        scene: &Scene,
        root: &impl Hitable,
        camera: &Camera,
        idx: usize,
    ) -> Color {
        use crate::util::Coord;
        // NOTE: I have no idea if seeding the Rng with the idx is valid.
        let mut rng = LcRng::new(idx as u64);
        let pos = Coord::from_index(idx, self.width, self.height);

        let mut total_color = Vec3::zero();

        for _ in 0..self.samples {
            let u = (pos.0 as f32 + rng.rand_f32()) / self.width as f32;
            let v = (pos.1 as f32 + rng.rand_f32()) / self.height as f32;
            let ray = camera.ray(u, v, &mut rng);
            total_color += color(&ray, &scene, root, 0, &mut rng);
        }

        total_color /= self.samples as f32;
        total_color = total_color
            .map(|x| x.powf(1. / self.gamma))
            .map(|x| x.clamp(0., 1.));

        let colori: Color = total_color.into();
        colori

        //let count = completed.fetch_add(1, Ordering::SeqCst);
        //if idx % 10000 == 0 {
        //println!("Completed {}/{}", count / 10000, WIDTH * HEIGHT / 10000)
        //}
    }
}

impl Default for Renderer {
    /// Creates a default renderer with
    /// width: 1920
    /// height: 1080
    /// samples: 128
    /// multithreaded: true
    /// use_bvh: false
    /// gamma: 2.2
    fn default() -> Self {
        Renderer {
            width: 1920,
            height: 1080,
            samples: 128,
            multithreaded: true,
            use_bvh: false,
            gamma: 2.2,
            camera: Default::default(),
        }
    }
}
