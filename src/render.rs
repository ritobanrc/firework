use crate::aabb::AABB;
use crate::material::{Material, MaterialIdx, MaterialLibrary};
use crate::Ray;
use tiny_rng::LcRng;
use ultraviolet::Vec3;

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

fn sky_color(r: &Ray) -> Vec3 {
    let dir = r.direction().normalized();
    // Take the y (from -1 to +1) and map it to 0..1
    let t = 0.5 * (dir.y + 1.0);
    (1. - t) * SKY_WHITE + t * SKY_BLUE
}

/// Performs the ray tracing for a given ray in the world and returns it's color.
pub fn color(r: &Ray, world: &dyn Hitable, materials: &MaterialLibrary, depth: usize, rand: &mut LcRng) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, 2e9, rand) {
        let emit = materials.get_material(hit.material).emit(hit.uv, &hit.point);
        if depth < 10 {
            if let Some(result) = materials.get_material(hit.material).scatter(r, &hit, rand) {
                emit + result.attenuation * color(&result.scattered, world, materials, depth + 1, rand)
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

pub struct FlipNormals {
    obj: Box<dyn Hitable + Sync>,
}

impl FlipNormals {
    pub fn new(obj: Box<dyn Hitable + Sync>) -> Self {
        FlipNormals { obj }
    }
}

impl Hitable for FlipNormals {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        if let Some(mut hit) = self.obj.hit(r, t_min, t_max, rand) {
            hit.normal = -hit.normal;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj.bounding_box()
    }
}

pub struct Translate {
    offset: Vec3,
    obj: Box<dyn Hitable + Sync>,
}

impl Translate {
    pub fn new(obj: Box<dyn Hitable + Sync>, offset: Vec3) -> Translate {
        Translate { obj, offset }
    }
}

impl Hitable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        let new_ray = Ray::new(*r.origin() - self.offset, *r.direction());
        if let Some(mut hit) = self.obj.hit(&new_ray, t_min, t_max, rand) {
            hit.point += self.offset;
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj
            .bounding_box()
            .map(|bb| AABB::new(bb.min + self.offset, bb.max + self.offset))
    }
}

pub struct RotateY {
    angle: f32,
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

        // TODO: Make code look like Rust instead of C
        let new_bbox = if let Some(bbox) = bbox {
            let mut min = 10e9 * Vec3::one();
            let mut max = -10e9 * Vec3::one();
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f32;
                        let j = j as f32;
                        let k = k as f32;
                        let x = i * bbox.max.x + (1. - i) * bbox.min.x;
                        let y = j * bbox.max.y + (1. - j) * bbox.min.y;
                        let z = k * bbox.max.z + (1. - k) * bbox.min.z;

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = Vec3::new(newx, y, newz);

                        for c in 0..3 {
                            if tester[c] > max[c] {
                                max[c] = tester[c]
                            }
                            if tester[c] < min[c] {
                                min[c] = tester[c]
                            }
                        }
                    }
                }
            }
            Some(AABB::new(min, max))
        } else {
            None
        };

        RotateY {
            angle,
            sin_theta,
            cos_theta,
            aabb: new_bbox,
            obj,
        }
    }
}

impl Hitable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
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

pub struct HitableList(Vec<Box<dyn Hitable + Sync>>);

impl HitableList {
    pub fn new() -> HitableList {
        HitableList(Vec::new())
    }

    pub fn _list(&self) -> &Vec<Box<dyn Hitable + Sync>> {
        &self.0
    }

    pub fn list_mut(&mut self) -> &mut Vec<Box<dyn Hitable + Sync>> {
        &mut self.0
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        let mut last_hit = None;
        let mut closest = t_max;
        for hitable in self.0.iter() {
            let new_hit = hitable.hit(r, t_min, closest, rand);
            if let Some(hit) = new_hit {
                closest = hit.t;
                last_hit = Some(hit);
            }
        }
        last_hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut result: Option<AABB> = None;
        for hitable in self.0.iter() {
            let next_box = hitable.bounding_box();
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
