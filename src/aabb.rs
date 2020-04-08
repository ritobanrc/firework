use crate::ray::Ray;
use std::mem;
use ultraviolet::Vec3;

#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for i in 0..3 {
            let inv_dir = 1. / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * inv_dir;
            let mut t1 = (self.max[i] - ray.origin()[i]) * inv_dir;

            if inv_dir < 0. {
                mem::swap(&mut t0, &mut t1);
            }

            tmin = tmin.max(t0);
            tmax = tmax.min(t1);
            if tmax <= tmin {
                return false;
            }
        }

        true
    }

    pub fn expand(&self, other: &Self) -> Self {
        let min = self.min.min_by_component(other.min);
        let max = self.max.max_by_component(other.max);

        AABB { min, max }
    }

    pub fn expand_to_point(&self, point: Vec3) -> Self {
        let min = self.min.min_by_component(point);
        let max = self.max.max_by_component(point);

        AABB { min, max }
    }
}
