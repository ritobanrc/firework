mod sphere;
mod cylinder;
mod disk;
mod cone;
mod rect;
mod rect3d;
mod volume;

pub use sphere::Sphere;
pub use cylinder::Cylinder;
pub use disk::Disk;
pub use cone::Cone;
pub use rect::{XYRect, YZRect, XZRect};
pub use rect3d::Rect3d;
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

