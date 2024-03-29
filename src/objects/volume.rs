use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::MaterialIdx;
use crate::serde_compat::{AsHitable, SerializableShape};
use serde::{Deserialize, Serialize};
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec2, Vec3};

#[derive(Serialize, Deserialize)]
pub struct ConstantMedium<T> {
    obj: T,
    density: f32,
    material: MaterialIdx,
}

impl ConstantMedium<Box<dyn SerializableShape>> {
    pub(crate) fn _new<T: SerializableShape + 'static>(
        obj: T,
        density: f32,
        material: crate::scene::MaterialIdx,
    ) -> Self {
        ConstantMedium {
            obj: Box::new(obj),
            density,
            material,
        }
    }

    pub(crate) fn from_boxed(
        obj: Box<dyn SerializableShape>,
        density: f32,
        material: crate::scene::MaterialIdx,
    ) -> Self {
        ConstantMedium {
            obj,
            density,
            material,
        }
    }
}

impl AsHitable for ConstantMedium<Box<dyn SerializableShape>> {
    fn to_hitable(self: Box<Self>) -> Box<dyn Hitable>
    where
        Self: 'static,
    {
        Box::new(ConstantMedium {
            obj: self.obj.to_hitable(),
            density: self.density,
            material: self.material,
        })
    }
}

impl<T: Hitable> Hitable for ConstantMedium<T> {
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
                        material: self.material,
                        uv: Vec2::new(0., 0.),
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        self.obj.bounding_box()
    }
}
