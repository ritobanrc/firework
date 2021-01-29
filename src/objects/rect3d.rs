use crate::aabb::AABB;
use crate::objects::{rect::Rect, XYRect, XZRect, YZRect};
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use tiny_rng::LcRng;
use ultraviolet::Vec3;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Rect3d {
    pos: Vec3,
    size: Vec3,
    faces: Vec<Rect>,
}

impl Rect3d {
    // TODO: Remove the position here, it should be handled by `RenderObject`
    fn new(pos: Vec3, size: Vec3, material: MaterialIdx) -> Rect3d {
        let faces: Vec<Rect> = vec![
            XYRect::new(
                pos.x,
                pos.x + size.x,
                pos.y,
                pos.y + size.y,
                pos.z + size.z,
                material,
            )
            .into(),
            XYRect::new(
                pos.x,
                pos.x + size.x,
                pos.y,
                pos.y + size.y,
                pos.z,
                material,
            )
            .flip_normal()
            .into(),
            XZRect::new(
                pos.x,
                pos.x + size.x,
                pos.z,
                pos.z + size.z,
                pos.y + size.y,
                material,
            )
            .into(),
            XZRect::new(
                pos.x,
                pos.x + size.x,
                pos.z,
                pos.z + size.z,
                pos.y,
                material,
            )
            .flip_normal()
            .into(),
            YZRect::new(
                pos.y,
                pos.y + size.y,
                pos.z,
                pos.z + size.z,
                pos.x + size.x,
                material,
            )
            .into(),
            YZRect::new(
                pos.y,
                pos.y + size.y,
                pos.z,
                pos.z + size.z,
                pos.x,
                material,
            )
            .flip_normal()
            .into(),
        ];

        Rect3d { pos, size, faces }
    }

    // TODO: Figure out Transformations
    pub fn with_size(size: Vec3, material: MaterialIdx) -> Rect3d {
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

    fn bounding_box(&self) -> AABB {
        AABB::new(self.pos, self.pos + self.size)
    }
}
