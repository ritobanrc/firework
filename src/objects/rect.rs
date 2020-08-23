use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use crate::serde_compat::Vec2Def;
use crate::util::Axis;
use tiny_rng::LcRng;
use ultraviolet::Vec2;

pub type XYRect = AARect<{ Axis::X }, { Axis::Y }>;
pub type YZRect = AARect<{ Axis::Y }, { Axis::Z }>;
pub type XZRect = AARect<{ Axis::X }, { Axis::Z }>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AARect<const A1: Axis, const A2: Axis> {
    #[serde(with = "Vec2Def")]
    min: Vec2,
    #[serde(with = "Vec2Def")]
    max: Vec2,
    k: f32,
    flip_normal: bool, // TODO: Shift this responsibility into the RenderObject
    material: MaterialIdx,
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
        material: MaterialIdx,
    ) -> Self {
        AARect {
            min: Vec2::new(a1_min, a2_min),
            max: Vec2::new(a1_max, a2_max),
            flip_normal: false,
            k,
            material,
        }
    }

    pub(crate) fn flip_normal(mut self) -> AARect<{ A1 }, { A2 }> {
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
            material: self.material,
            uv: Vec2::new(
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

#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) enum Rect {
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
