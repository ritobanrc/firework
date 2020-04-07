use crate::aabb::AABB;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::{MaterialIdx, Scene};
use crate::texture::Texture;
use tiny_rng::{LcRng, Rand};
use crate::material::IsotropicMat;
use ultraviolet::Vec3;

pub struct ConstantMedium {
    obj: Box<dyn Hitable + Sync>,
    density: f32,
    material: MaterialIdx,
}

impl ConstantMedium {
    pub fn new<T: Hitable + Sync + 'static>(
        obj: T,
        density: f32,
        texture: Box<dyn Texture + Sync>,
        scene: &mut Scene,
    ) -> Self {
        ConstantMedium {
            obj: Box::new(obj),
            density,
            material: scene.add_material(IsotropicMat::new(texture)),
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
                        material: self.material,
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
