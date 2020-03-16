use crate::aabb::AABB;
use crate::camera::Camera;
use crate::material::Material;
use crate::Ray;
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec3, Rotor3, Mat3};

const SKY_BLUE: Vec3 = Vec3 {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};
const SKY_WHITE: Vec3 = Vec3 {
    x: 1.,
    y: 1.,
    z: 1.,
};

/// Used to index `Material`s in a `Scene`
pub type MaterialIdx = usize;

/// Used to index `Material`s in a `Scene`
pub type RenderObjectIdx = usize;

pub struct Scene<'a> {
    pub(crate) render_objects: Vec<RenderObject<'a>>,
    materials: Vec<Box<dyn Material + Sync + 'a>>, // TODO: Remove the layer of indirection here
    camera: Camera,
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera) -> Scene<'a> {
        Scene {
            render_objects: Vec::new(),
            materials: Vec::new(),
            camera
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

    /// Returns the ray for the camera at a given location on the screen
    pub(crate) fn ray(&self, s: f32, t: f32, rand: &mut impl Rand) -> Ray {
        self.camera.ray(s, t, rand)
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
    pub fn new<T: Hitable + Sync + 'a>(obj: T) -> RenderObject<'a> {
        let aabb = obj.bounding_box();
        RenderObject {
            obj: Box::new(obj),
            position: Vec3::zero(),
            rotation: Rotor3::identity(),
            rotation_mat: Mat3::identity(),
            inv_rotation_mat: Mat3::identity(),
            flip_normals: false,
            aabb
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
            Some(AABB::new(rotated_aabb.min + self.position, rotated_aabb.max + self.position))
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
                self.inv_rotation_mat * *r.direction()
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

/// A function that creates a basic sky gradient between SKY_BLUE and SKY_WHITE
/// TODO: Don't have hardcoded SKY_BLUE and SKY_WHITE colors.
fn sky_color(r: &Ray) -> Vec3 {
    let dir = r.direction().normalized();
    // Take the y (from -1 to +1) and map it to 0..1
    let t = 0.5 * (dir.y + 1.0);
    (1. - t) * SKY_WHITE + t * SKY_BLUE
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
        Vec3::zero()
        //sky_color(r)
    }
}

pub struct RaycastHit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: MaterialIdx,
    pub uv: (f32, f32),
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit>;
    fn bounding_box(&self) -> Option<AABB>;
}


pub struct RotateY {
    _angle: f32,
    sin_theta: f32,
    cos_theta: f32,
    aabb: Option<AABB>,
    obj: Box<dyn Hitable + Sync>,
}

impl RotateY {
    pub fn new(angle: f32, obj: Box<dyn Hitable + Sync>) -> RotateY {
        use std::f32::consts::PI;
        let theta = angle * PI / 180.;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let bbox = obj.bounding_box();


        // Create the bounding box for the rotated object.
        let new_bbox = if let Some(bbox) = bbox {
            let mut min = 10e9 * Vec3::one();
            let mut max = -10e9 * Vec3::one();

            // For each corner of the bounding box
            for (i, j, k) in iproduct!(0..2, 0..2, 0..2) {
                // Get the position of the corner
                let x = if i == 0 { bbox.min.x } else { bbox.max.x };
                let y = if j == 0 { bbox.min.y } else { bbox.max.y };
                let z = if k == 0 { bbox.min.z } else { bbox.max.z };

                // Calculate the rotated positions. To rotate something in 2D (i.e., around the Y
                // axis)
                let newx = cos_theta * x + sin_theta * z;
                let newz = -sin_theta * x + cos_theta * z;
                let tester = Vec3::new(newx, y, newz);

                // Make the bounding box as big as possible. It starts "negative", and then gets
                // bigger to include every vertex on each axis.
                for c in 0..3 {
                    max[c] = tester[c].max(max[c]);
                    min[c] = tester[c].min(min[c]);
                }
            }
            Some(AABB::new(min, max))
        } else {
            None
        };

        RotateY {
            _angle: angle,
            sin_theta,
            cos_theta,
            aabb: new_bbox,
            obj,
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        // [ cos x   -sin x]
        // [ sin x    cos x]
        let origin = Vec3::new(
            self.cos_theta * r.origin().x - self.sin_theta * r.origin().z,
            r.origin().y,
            self.sin_theta * r.origin().x + self.cos_theta * r.origin().z,
        );
        let direction = Vec3::new(
            self.cos_theta * r.direction().x - self.sin_theta * r.direction().z,
            r.direction().y,
            self.sin_theta * r.direction().x + self.cos_theta * r.direction().z,
        );
        let new_r = Ray::new(origin, direction);
        if let Some(mut hit) = self.obj.hit(&new_r, t_min, t_max, rand) {
            // [  cos x    sin x]
            // [ -sin x    cos x]
            hit.point = Vec3::new(
                self.cos_theta * hit.point.x + self.sin_theta * hit.point.z,
                hit.point.y,
                -self.sin_theta * hit.point.x + self.cos_theta * hit.point.z,
            );
            hit.normal = Vec3::new(
                self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z,
                hit.normal.y,
                -self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z,
            );

            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.aabb.clone()
    }
}
