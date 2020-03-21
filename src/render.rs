use crate::aabb::AABB;
use crate::camera::{Camera, CameraSettings};
use crate::material::Material;
use crate::util::Color;
use crate::ray::Ray;
use std::sync::atomic::{AtomicUsize, Ordering};
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Mat3, Rotor3, Vec3};

/// Used to index `Material`s in a `Scene`
pub type MaterialIdx = usize;

/// Used to index `Material`s in a `Scene`
pub type RenderObjectIdx = usize;

/// Represents a Scene
pub struct Scene<'a> {
    pub(crate) render_objects: Vec<RenderObject<'a>>,
    materials: Vec<Box<dyn Material + Sync + 'a>>, // TODO: Remove the layer of indirection here
    environment: Box<dyn Fn(Vec3) -> Vec3 + Sync + 'a>,
}

impl<'a> Scene<'a> {
    /// Creates an empty scene, with the given camera.
    pub fn new() -> Scene<'a> {
        Scene {
            render_objects: Vec::new(),
            materials: Vec::new(),
            environment: Box::new(|_: Vec3| Vec3::zero()),
        }
    }

    /// Adds a material to the `Scene` and returns it's `MaterialIdx`
    pub fn add_object(&mut self, obj: RenderObject<'a>) -> RenderObjectIdx {
        self.render_objects.push(obj);
        self.render_objects.len() - 1
    }

    /// Returns a reference to the `RenderObject` stored at the given `RenderObjectIdx`
    pub fn get_object(&self, idx: RenderObjectIdx) -> &RenderObject {
        &self.render_objects[idx]
    }

    /// Adds a material to the `Scene` and returns it's `MaterialIdx`
    pub fn add_material<T: Material + Sync + 'a>(&mut self, mat: T) -> MaterialIdx {
        self.materials.push(Box::new(mat));
        self.materials.len() - 1
    }

    /// Returns a reference to `Material` stored at the given `MaterialIdx`
    pub fn get_material(&self, idx: MaterialIdx) -> &(dyn Material + Sync) {
        self.materials[idx].as_ref()
    }

    /// Sets the closure for the "environment"
    pub fn set_environment<F: Fn(Vec3) -> Vec3 + Sync + 'a>(&mut self, func: F) {
        self.environment = Box::new(func);
    }
}

impl Hitable for Scene<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        let mut last_hit = None;
        let mut closest = t_max;
        for render_obj in &self.render_objects {
            let new_hit = render_obj.hit(r, t_min, closest, rand);
            if let Some(hit) = new_hit {
                closest = hit.t;
                last_hit = Some(hit);
            }
        }
        last_hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut result: Option<AABB> = None;
        for render_obj in &self.render_objects {
            let next_box = render_obj.bounding_box();
            if let Some(next_box) = next_box {
                if let Some(aabb) = result {
                    result = Some(aabb.expand(&next_box));
                } else {
                    result = Some(next_box);
                }
            }
        }
        result
    }
}

/// A struct representing an object that can be rendered. Contains the base `Hitable` as well as
/// any transformations on it.
pub struct RenderObject<'a> {
    obj: Box<dyn Hitable + Sync + 'a>,
    position: Vec3,
    rotation: Rotor3, // TODO: Replace this with ultraviolet::Rotor and/or
    rotation_mat: Mat3,
    inv_rotation_mat: Mat3,
    flip_normals: bool,
    aabb: Option<AABB>,
}

impl<'a> RenderObject<'a> {
    /// Creates a new RenderObject
    pub fn new<T: Hitable + Sync + 'a>(obj: T) -> RenderObject<'a> {
        let aabb = obj.bounding_box();
        RenderObject {
            obj: Box::new(obj),
            position: Vec3::zero(),
            rotation: Rotor3::identity(),
            rotation_mat: Mat3::identity(),
            inv_rotation_mat: Mat3::identity(),
            flip_normals: false,
            aabb,
        }
    }

    /// Sets the position of the `RenderObject`
    #[inline(always)]
    pub fn position(mut self, x: f32, y: f32, z: f32) -> RenderObject<'a> {
        self.position = Vec3::new(x, y, z);
        self.update_bounding_box();
        self
    }

    /// Sets the position of the `RenderObject`
    #[inline(always)]
    pub fn position_vec(mut self, pos: Vec3) -> RenderObject<'a> {
        self.position = pos;
        self.update_bounding_box();
        self
    }

    /// Sets the rotation of the `RenderObject` on the Y Axis
    #[inline(always)]
    pub fn rotate(mut self, rotor: Rotor3) -> RenderObject<'a> {
        self.rotation = rotor;
        self.rotation_mat = rotor.into_matrix();
        self.inv_rotation_mat = rotor.reversed().into_matrix();
        self.update_bounding_box();
        self
    }

    /// Sets the `flip_normals` value to the opposite of what it was previously
    #[inline(always)]
    pub fn flip_normals(mut self) -> RenderObject<'a> {
        self.flip_normals = !self.flip_normals;
        self
    }

    fn update_bounding_box(&mut self) {
        self.aabb = if let Some(bbox) = self.obj.bounding_box() {
            // First, rotate the bounding box
            // If there is a signficant rotation
            let rotated_aabb = if self.rotation.mag_sq() > 0.001 {
                let mut min = 10e9 * Vec3::one();
                let mut max = -10e9 * Vec3::one();
                for (i, j, k) in iproduct!(0..2, 0..2, 0..2) {
                    // Get the position of the corner
                    let x = if i == 0 { bbox.min.x } else { bbox.max.x };
                    let y = if j == 0 { bbox.min.y } else { bbox.max.y };
                    let z = if k == 0 { bbox.min.z } else { bbox.max.z };

                    let new_pos = self.rotation_mat * Vec3::new(x, y, z);
                    for c in 0..3 {
                        max[c] = new_pos[c].max(max[c]);
                        min[c] = new_pos[c].min(min[c]);
                    }
                }
                AABB::new(min, max)
            } else {
                bbox
            };
            // Then translate it
            Some(AABB::new(
                rotated_aabb.min + self.position,
                rotated_aabb.max + self.position,
            ))
        } else {
            None
        }
    }
}

impl Hitable for RenderObject<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        let new_ray = if self.rotation.mag_sq() > 0.001 {
            Ray::new(
                self.inv_rotation_mat * (*r.origin() - self.position),
                self.inv_rotation_mat * *r.direction(),
            )
        } else {
            Ray::new(*r.origin() - self.position, *r.direction())
        };
        if let Some(mut hit) = self.obj.hit(&new_ray, t_min, t_max, rand) {
            hit.point = self.rotation_mat * hit.point;
            hit.point += self.position;

            hit.normal = self.rotation_mat * hit.normal;
            if self.flip_normals {
                hit.normal = -hit.normal;
            }
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.aabb.clone()
    }
}

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
    pub uv: (f32, f32),
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
