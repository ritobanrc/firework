use crate::aabb::AABB;
use crate::material::{IsotropicMat, Material};
use crate::ray::Ray;
use crate::render::{FlipNormals, Hitable, HitableList, RaycastHit};
use crate::texture::Texture;
use crate::util::{sphere_uv, Axis};
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec2, Vec3};

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
    flip_normal: bool,
    material: Box<dyn Material + Sync>,
}
impl<const A1: Axis, const A2: Axis> AARect<{ A1 }, { A2 }> {
    // Note, this assumes `flip_normal` is false -- just so I don't have to change all the code
    // that used the old `FlipNormals` struct.
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
            flip_normal: false,
            k,
            material,
        }
    }

    fn flip_normal(mut self) -> AARect<{ A1 }, { A2 }> {
        self.flip_normal = true;
        self
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
        let normal = Axis::other(A1, A2).unit_vec();
        Some(RaycastHit {
            t,
            point,
            normal: if self.flip_normal { -normal } else { normal },
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

pub enum Rect {
    XY(XYRect),
    XZ(XZRect),
    YZ(YZRect),
}

impl From<XYRect> for Rect {
    fn from(rect: XYRect) -> Rect {
        Rect::XY(rect)
    }
}

impl From<XZRect> for Rect {
    fn from(rect: XZRect) -> Rect {
        Rect::XZ(rect)
    }
}

impl From<YZRect> for Rect {
    fn from(rect: YZRect) -> Rect {
        Rect::YZ(rect)
    }
}

impl Hitable for Rect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        match self {
            Rect::XY(rect) => rect.hit(r, t_min, t_max, rand),
            Rect::XZ(rect) => rect.hit(r, t_min, t_max, rand),
            Rect::YZ(rect) => rect.hit(r, t_min, t_max, rand),
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        match self {
            Rect::XY(rect) => rect.bounding_box(),
            Rect::XZ(rect) => rect.bounding_box(),
            Rect::YZ(rect) => rect.bounding_box(),
        }
    }
}

pub struct Rect3d {
    pos: Vec3,
    size: Vec3,
    faces: Vec<Rect>,
}

impl Rect3d {
    pub fn new(pos: Vec3, size: Vec3, material: impl Fn() -> Box<dyn Material + Sync>) -> Rect3d {
        let faces: Vec<Rect> = vec![
            XYRect::new(
                pos.x,
                pos.x + size.x,
                pos.y,
                pos.y + size.y,
                pos.z + size.z,
                material(),
            )
            .into(),
            XYRect::new(
                pos.x,
                pos.x + size.x,
                pos.y,
                pos.y + size.y,
                pos.z,
                material(),
            )
            .flip_normal()
            .into(),
            XZRect::new(
                pos.x,
                pos.x + size.x,
                pos.z,
                pos.z + size.z,
                pos.y + size.y,
                material(),
            )
            .into(),
            XZRect::new(
                pos.x,
                pos.x + size.x,
                pos.z,
                pos.z + size.z,
                pos.y,
                material(),
            )
            .flip_normal()
            .into(),
            YZRect::new(
                pos.y,
                pos.y + size.y,
                pos.z,
                pos.z + size.z,
                pos.x + size.x,
                material(),
            )
            .into(),
            YZRect::new(
                pos.y,
                pos.y + size.y,
                pos.z,
                pos.z + size.z,
                pos.x,
                material(),
            )
            .flip_normal()
            .into(),
        ];

        Rect3d { pos, size, faces }
    }

    pub fn with_size(size: Vec3, material: impl Fn() -> Box<dyn Material + Sync>) -> Rect3d {
        Rect3d::new(Vec3::zero(), size, material)
    }
}

impl Hitable for Rect3d {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rand: &mut LcRng) -> Option<RaycastHit> {
        let mut last_hit = None;
        let mut closest = t_max;
        for rect in self.faces.iter() {
            let new_hit = rect.hit(r, t_min, closest, rand);
            if let Some(hit) = new_hit {
                closest = hit.t;
                last_hit = Some(hit);
            }
        }
        last_hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.pos, self.pos + self.size))
    }
}

pub struct ConstantMedium {
    obj: Box<dyn Hitable + Sync>,
    density: f32,
    material: Box<dyn Material + Sync>,
}

impl ConstantMedium {
    pub fn new(
        obj: Box<dyn Hitable + Sync>,
        density: f32,
        texture: Box<dyn Texture + Sync>,
    ) -> Self {
        ConstantMedium {
            obj,
            density,
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
                    return None;
                }
                rec1.t = rec1.t.max(0.);
                let dist_inside_boundary = (rec2.t - rec1.t) * r.direction().mag();
                let hit_distance = -(1. / self.density) * rand.rand_f32().log10();

                if hit_distance < dist_inside_boundary {
                    let t = rec1.t + hit_distance / r.direction().mag();
                    return Some(RaycastHit {
                        t,
                        point: r.point(t),
                        normal: Vec3::unit_y(), // arbitrary
                        material: self.material.as_ref(),
                        uv: (0., 0.),
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj.bounding_box()
    }
}
