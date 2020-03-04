use crate::aabb::AABB;
use crate::material::{Material, IsotropicMat};
use crate::texture::Texture;
use crate::util::{sphere_uv, Axis};
use crate::Ray;
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec2, Vec3};

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

pub fn color(r: &Ray, world: &dyn Hitable, depth: usize, rand: &mut LcRng) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, 2e9, rand) {
        let emit = hit.material.emit(hit.uv, &hit.point);
        if depth < 10 {
            if let Some(result) = hit.material.scatter(r, &hit, rand) {
                emit + result.attenuation * color(&result.scattered, world, depth + 1, rand)
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

pub struct RaycastHit<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    pub uv: (f32, f32),
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit>;
    fn bounding_box(&self) -> Option<AABB>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Material + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Material + Sync>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit<'_>> {
        let v = *r.origin() - self.center;
        let a = r.direction().dot(*r.direction());
        let b = v.dot(*r.direction());
        let c = v.dot(v) - self.radius * self.radius;
        let disc = b * b - a * c;

        if disc > 0.0 {
            fn roots(a: f32, b: f32, c: f32, t_max: f32, t_min: f32) -> Option<f32> {
                let lhs = -b;
                let rhs = (b * b - a * c).sqrt();
                let temp = (lhs - rhs) / a;
                if temp < t_max && temp > t_min {
                    return Some(temp);
                }
                let temp = (lhs + rhs) / a;
                if temp < t_max && temp > t_min {
                    return Some(temp);
                }

                None
            }

            if let Some(t) = roots(a, b, c, t_max, t_min) {
                let point = r.point(t);
                Some(RaycastHit {
                    t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.as_ref(),
                    uv: sphere_uv(&((point - self.center) / self.radius)),
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::one() * self.radius,
            self.center + Vec3::one() * self.radius,
        ))
    }
}

pub type XYRect = AARect<{ Axis::X }, { Axis::Y }>;
pub type YZRect = AARect<{ Axis::Y }, { Axis::Z }>;
pub type XZRect = AARect<{ Axis::X }, { Axis::Z }>;

pub struct AARect<const A1: Axis, const A2: Axis> {
    min: Vec2,
    max: Vec2,
    k: f32,
    material: Box<dyn Material + Sync>,
}
impl<const A1: Axis, const A2: Axis> AARect<{ A1 }, { A2 }> {
    pub fn new(
        a1_min: f32,
        a1_max: f32,
        a2_min: f32,
        a2_max: f32,
        k: f32,
        material: Box<dyn Material + Sync>,
    ) -> Self {
        AARect {
            min: Vec2::new(a1_min, a2_min),
            max: Vec2::new(a1_max, a2_max),
            k,
            material,
        }
    }
}

impl<const A1: Axis, const A2: Axis> Hitable for AARect<{ A1 }, { A2 }> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let t = (self.k - r.origin()[Axis::other(A1, A2) as usize])
            / r.direction()[Axis::other(A1, A2) as usize];
        if t < t_min || t > t_max {
            return None;
        }
        let point = r.point(t);
        if point[A1 as usize] < self.min.x
            || point[A1 as usize] > self.max.x
            || point[A2 as usize] < self.min.y
            || point[A2 as usize] > self.max.y
        {
            return None;
        }
        Some(RaycastHit {
            t,
            point,
            normal: Axis::other(A1, A2).unit_vec(),
            material: self.material.as_ref(),
            uv: (
                (point[A1 as usize] - self.min.x) / (self.max.x - self.min.x),
                (point[A2 as usize] - self.min.y) / (self.max.y - self.min.y),
            ),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut min = [0f32; 3];
        min[A1 as usize] = self.min.x;
        min[A2 as usize] = self.min.y;
        min[Axis::other(A1, A2) as usize] = self.k - 0.01;
        let mut max = [0f32; 3];
        max[A1 as usize] = self.max.x;
        max[A2 as usize] = self.max.y;
        max[Axis::other(A1, A2) as usize] = self.k + 0.01;
        Some(AABB::new(min.into(), max.into()))
    }
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

pub struct Rect3d {
    pos: Vec3,
    size: Vec3,
    faces: HitableList,
}

impl Rect3d {
    pub fn new(pos: Vec3, size: Vec3, material: impl Fn() -> Box<dyn Material + Sync>) -> Rect3d {
        let mut faces = HitableList::new();
        faces.list_mut().push(Box::new(XYRect::new(
            pos.x,
            pos.x + size.x,
            pos.y,
            pos.y + size.y,
            pos.z + size.z,
            material(),
        )));
        faces
            .list_mut()
            .push(Box::new(FlipNormals::new(Box::new(XYRect::new(
                pos.x,
                pos.x + size.x,
                pos.y,
                pos.y + size.y,
                pos.z,
                material(),
            )))));
        faces.list_mut().push(Box::new(XZRect::new(
            pos.x,
            pos.x + size.x,
            pos.z,
            pos.z + size.z,
            pos.y + size.y,
            material(),
        )));
        faces
            .list_mut()
            .push(Box::new(FlipNormals::new(Box::new(XZRect::new(
                pos.x,
                pos.x + size.x,
                pos.z,
                pos.z + size.z,
                pos.y,
                material(),
            )))));
        faces.list_mut().push(Box::new(YZRect::new(
            pos.y,
            pos.y + size.y,
            pos.z,
            pos.z + size.z,
            pos.x + size.x,
            material(),
        )));
        faces
            .list_mut()
            .push(Box::new(FlipNormals::new(Box::new(YZRect::new(
                pos.y,
                pos.y + size.y,
                pos.z,
                pos.z + size.z,
                pos.x,
                material(),
            )))));

        Rect3d { pos, size, faces }
    }

    pub fn with_size(size: Vec3, material: impl Fn() -> Box<dyn Material + Sync>) -> Rect3d {
        Rect3d::new(Vec3::zero(), size, material)
    }
}

impl Hitable for Rect3d {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        self.faces.hit(r, t_min, t_max, rand)
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.pos, self.pos + self.size))
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

pub struct ConstantMedium {
    obj: Box<dyn Hitable + Sync>,
    density: f32,
    material: Box<dyn Material + Sync>,
}

impl ConstantMedium {
    pub fn new(obj: Box<dyn Hitable + Sync>, density: f32, texture: Box<dyn Texture + Sync>) -> Self {
        ConstantMedium {
            obj, density, 
            material: Box::new(IsotropicMat::new(texture)),
        }
    }
}

impl Hitable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        if let Some(mut rec1) = self.obj.hit(r, -std::f32::MAX, std::f32::MAX, rand) {
            if let Some(mut rec2) = self.obj.hit(r, rec1.t + 0.0001, std::f32::MAX, rand) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);
                if rec1.t >= rec2.t {
                    return None
                }
                rec1.t = rec1.t.max(0.);
                let dist_inside_boundary = (rec2.t - rec1.t)*r.direction().mag();
                let hit_distance = -(1. / self.density) * rand.rand_f32().log10();

                if hit_distance < dist_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction().mag();
                    return Some(RaycastHit {
                        t, 
                        point: r.point(t),
                        normal: Vec3::unit_y(), // arbitrary
                        material: self.material.as_ref(),
                        uv: (0., 0.)
                    })
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj.bounding_box()
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
