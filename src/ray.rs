use ultraviolet::vec::Vec3;

/// Represents a Ray
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3)  -> Ray {
        Ray { origin, dir }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn point(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }

}
