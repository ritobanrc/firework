mod cone;
mod cylinder;
mod disk;
mod mesh;
mod rect;
mod rect3d;
mod sphere;
mod volume;

pub use cone::Cone;
pub use cylinder::Cylinder;
pub use disk::Disk;
pub use mesh::{Triangle, TriangleMesh};
pub use rect::{XYRect, XZRect, YZRect};
pub use rect3d::Rect3d;
pub use sphere::Sphere;
pub use volume::ConstantMedium;

pub(crate) fn solve_quadratic(a: f32, b: f32, c: f32) -> [Option<f32>; 2] {
    let disc = b * b - 4. * a * c;
    if disc < 0. {
        [None, None]
    } else if disc == 0. {
        [Some(-b / (2. * a)), None]
    } else {
        [
            Some((-b - disc.sqrt()) / (2. * a)),
            Some((-b + disc.sqrt()) / (2. * a)),
        ]
    }
}
