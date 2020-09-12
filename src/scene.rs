use crate::aabb::AABB;
use crate::environment::{ColorEnv, Environment};
use crate::material::Material;
use crate::objects::{Triangle, TriangleMesh};
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::serde_compat::{Rotor3Def, SerializableShape, Vec3Def};
use itertools::iproduct;
use serde::{Deserialize, Serialize};
use tiny_rng::LcRng;
use ultraviolet::{Mat3, Rotor3, Vec3};

/// Used to index `Material`s in a `Scene`
pub type MaterialIdx = usize;

/// Used to index `Material`s in a `Scene`
pub type RenderObjectIdx = usize;

/// Represents a Scene
#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub render_objects: Vec<RenderObject>,
    pub materials: Vec<Box<dyn Material + 'static>>, // TODO: Remove the layer of indirection here
    pub environment: Box<dyn Environment + 'static>,
}

impl Scene {
    /// Creates an empty scene, with the given camera.
    /// ```
    /// use firework::Scene;
    /// let mut scene = Scene::new();
    /// ```
    pub fn new() -> Self {
        Scene {
            render_objects: Vec::new(),
            materials: Vec::new(),
            environment: Box::new(ColorEnv::default()),
        }
    }

    /// Adds a material to the `Scene` and returns it's `MaterialIdx`
    pub fn add_object(&mut self, obj: RenderObject) -> RenderObjectIdx {
        self.render_objects.push(obj);
        self.render_objects.len() - 1
    }

    //pub fn add_mesh(&mut self, mesh: TriangleMesh) {
    //use std::sync::Arc;
    //let mesh = Arc::new(mesh);
    //for tri in 0..mesh.num_tris() {
    //self.add_object(RenderObject::new(Triangle::new(Arc::clone(&mesh), tri)));
    //}
    //}

    /// Returns a reference to the `RenderObject` stored at the given `RenderObjectIdx`
    pub fn get_object(&self, idx: RenderObjectIdx) -> &RenderObject {
        &self.render_objects[idx]
    }

    /// Adds a material to the `Scene` and returns it's `MaterialIdx`
    /// ```
    /// use firework::Scene;
    /// use firework::material::LambertianMat;
    /// use ultraviolet::Vec3;
    /// let mut scene = Scene::new();
    /// let red = scene.add_material(LambertianMat::with_color(Vec3::new(1., 0., 0.)));
    /// ```
    pub fn add_material<T: Material + Sync + 'static>(&mut self, mat: T) -> MaterialIdx {
        self.materials.push(Box::new(mat));
        self.materials.len() - 1
    }

    /// Returns a reference to `Material` stored at the given `MaterialIdx`
    pub fn get_material(&self, idx: MaterialIdx) -> &dyn Material {
        self.materials[idx].as_ref()
    }

    /// Sets the closure for the "environment"
    pub fn set_environment(&mut self, env: impl Environment + Sync + 'static) {
        self.environment = Box::new(env);
    }
}

pub(crate) struct SceneInternal {
    pub render_objects: Vec<RenderObjectInternal>,
    pub materials: Vec<Box<dyn Material + 'static>>, // TODO: Remove the layer of indirection here
    pub environment: Box<dyn Environment + 'static>,
}

impl SceneInternal {
    /// Returns a reference to the `RenderObject` stored at the given `RenderObjectIdx`
    pub fn get_object(&self, idx: RenderObjectIdx) -> &RenderObjectInternal {
        &self.render_objects[idx]
    }

    /// Returns a reference to `Material` stored at the given `MaterialIdx`
    pub fn get_material(&self, idx: MaterialIdx) -> &dyn Material {
        self.materials[idx].as_ref()
    }
}

impl From<Scene> for SceneInternal {
    fn from(scene: Scene) -> Self {
        SceneInternal {
            render_objects: scene.render_objects.into_iter().map(|x| x.into()).collect(),
            materials: scene.materials,
            environment: scene.environment,
        }
    }
}

impl Hitable for SceneInternal {
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

#[derive(Deserialize)]
#[serde(from = "RenderObject")]
pub(crate) struct RenderObjectInternal {
    pub(crate) obj: Box<dyn Hitable + 'static>,
    pub(crate) position: Vec3,
    pub(crate) rotation_mat: Mat3,
    pub(crate) inv_rotation_mat: Mat3,
    pub(crate) flip_normals: bool,
    pub(crate) aabb: Option<AABB>,
}

impl From<RenderObject> for RenderObjectInternal {
    fn from(s: RenderObject) -> RenderObjectInternal {
        let mut obj = RenderObjectInternal {
            obj: s.obj.to_hitable(),
            position: s.position,
            rotation_mat: s.rotation.into_matrix(),
            inv_rotation_mat: s.rotation.reversed().into_matrix(),
            flip_normals: s.flip_normals,
            aabb: None,
        };
        obj.update_bounding_box();
        obj
    }
}

impl RenderObjectInternal {
    pub(crate) fn update_bounding_box(&mut self) {
        self.aabb = if let Some(bbox) = self.obj.bounding_box() {
            // First, rotate the bounding box
            // If there is a signficant rotation
            let cos_trace = {
                let trace =
                    self.rotation_mat[0][0] + self.rotation_mat[1][1] + self.rotation_mat[2][2];
                0.5 * (trace - 1.) // .acos()
            };
            let rotated_aabb = if cos_trace < 0.999 {
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

/// A struct representing an object that can be rendered. Contains the base `Hitable` as well as
/// any transformations on it.
#[derive(Serialize, Deserialize)]
pub struct RenderObject {
    obj: Box<dyn SerializableShape>,
    #[serde(with = "Vec3Def")]
    position: Vec3,
    #[serde(with = "Rotor3Def")]
    rotation: Rotor3,
    flip_normals: bool,
}

impl RenderObject {
    /// Creates a new RenderObject
    pub fn new<T: SerializableShape + 'static>(obj: T) -> Self {
        RenderObject {
            obj: Box::new(obj),
            position: Vec3::zero(),
            rotation: Rotor3::identity(),
            flip_normals: false,
        }
    }

    /// Sets the position of the `RenderObject`
    #[inline(always)]
    pub fn position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.position = Vec3::new(x, y, z);
        self
    }

    /// Sets the position of the `RenderObject`
    #[inline(always)]
    pub fn position_vec(mut self, pos: Vec3) -> Self {
        self.position = pos;
        self
    }

    /// Sets the rotation of the `RenderObject` on the Y Axis
    #[inline(always)]
    pub fn rotate(mut self, rotor: Rotor3) -> Self {
        self.rotation = rotor;
        //self.rotation_mat = rotor.into_matrix();
        //self.inv_rotation_mat = rotor.reversed().into_matrix();
        self
    }

    /// Sets the `flip_normals` value to the opposite of what it was previously
    #[inline(always)]
    pub fn flip_normals(mut self) -> Self {
        self.flip_normals = !self.flip_normals;
        self
    }
}

impl Hitable for RenderObjectInternal {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        let cos_trace = {
            let trace = self.rotation_mat[0][0] + self.rotation_mat[1][1] + self.rotation_mat[2][2];
            0.5 * (trace - 1.) // .acos()
        };
        let new_ray = if cos_trace < 0.999 {
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
