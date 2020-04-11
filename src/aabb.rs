use crate::ray::Ray;
use std::mem;
use ultraviolet::Vec3;

/// An axis aligned bounding box, represented with a minimum and maximum points
#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    /// Creates a new axis aligned bounding box with a min and max point
    /// Note that this will produce unexpected results if min is not less than max on each
    /// axis
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    /// Creates a new axis aligned bounding box that contains two points (p0 need not be
    /// stricly less than p1, componentwise)
    pub fn from_two_points(p0: Vec3, p1: Vec3) -> Self {
        let min = p0.min_by_component(p1);
        let max = p0.max_by_component(p1);

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

    pub fn center(&self) -> Vec3 {
        0.5 * self.min + 0.5 * self.max
    }

    pub fn expand_to_point(&self, point: Vec3) -> Self {
        let min = self.min.min_by_component(point);
        let max = self.max.max_by_component(point);

        AABB { min, max }
    }
}
