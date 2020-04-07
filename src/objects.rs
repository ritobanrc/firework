use crate::aabb::AABB;
use crate::material::IsotropicMat;
use crate::ray::Ray;
use crate::render::{Hitable, RaycastHit};
use crate::scene::{MaterialIdx, Scene};
use crate::texture::Texture;
use crate::util::Axis;
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec2, Vec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    radius: f32,
    material: MaterialIdx,
}

impl Sphere {
    pub fn new(radius: f32, material: MaterialIdx) -> Sphere {
        Sphere { radius, material }
    }
}

pub fn sphere_uv(point: &Vec3) -> (f32, f32) {
    use std::f32::consts::PI;
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    (u, v)
}

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> [Option<f32>; 2] {
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

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let o = *r.origin();
        let d = *r.direction();
        let a = d.dot(d);
        let b = 2. * o.dot(d);
        let c = o.dot(o) - self.radius * self.radius;

        if let [Some(t1), t2] = solve_quadratic(a, b, c) {
            let t = if t1 < t_max && t1 > t_min {
                t1
            } else {
                match t2 {
                    Some(t2) if t2 < t_max && t2 > t_min => t2,
                    _ => return None,
                }
            };

            let point = r.point(t);
            Some(RaycastHit {
                t,
                point,
                normal: point / self.radius,
                material: self.material,
                uv: sphere_uv(&(point / self.radius)),
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            -Vec3::one() * self.radius,
            Vec3::one() * self.radius,
        ))
    }
}

/// A vertically oriented cylinder, with a given radius and height
pub struct Cylinder {
    radius: f32,
    height: f32,
    max_phi: f32,
    material: MaterialIdx,
}

impl Cylinder {
    /// Creates a cylinder with the given radius and height
    pub fn new(radius: f32, height: f32, material: MaterialIdx) -> Self {
        Cylinder {
            radius,
            height,
            material,
            max_phi: 360f32.to_radians(),
        }
    }

    /// Creates a cylinder with the given radius and height, that only goes around for `phi` degrees.
    pub fn partial(radius: f32, height: f32, phi: f32, material: MaterialIdx) -> Self {
        Cylinder {
            radius,
            height,
            material,
            max_phi: phi.to_radians(),
        }
    }
}

impl Hitable for Cylinder {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let o = r.origin();
        let d = r.direction();
        let a = d.x * d.x + d.z * d.z;
        let b = 2. * (d.x * o.x + d.z * o.z);
        let c = o.x * o.x + o.z * o.z - self.radius * self.radius;

        let disc = b * b - 4. * a * c;
        if disc > 0.0 {
            if let [Some(t1), t2] = solve_quadratic(a, b, c) {
                // define a closure to check if any t results in a hit
                let check_solution = |t| {
                    if t > t_max || t < t_min {
                        return None; // this is returning from teh closure
                    }
                    let point = r.point(t);
                    let phi = {
                        let phi = point.z.atan2(point.x);
                        if phi < 0. {
                            phi + std::f32::consts::PI * 2.
                        } else {
                            phi
                        }
                    };
                    if point.y > 0. && point.y < self.height && phi < self.max_phi {
                        let u = phi / self.max_phi;
                        let v = point.y / self.height;
                        //let dpdu = Vec3::new(-self.max_phi * point.z, 0., self.max_phi * point.x);
                        //let dpdv = self.height * Vec3::unit_y();
                        Some(RaycastHit {
                            t,
                            point,
                            //normal: dpdu.cross(dpdv).normalized(),
                            normal: Vec3::new(point.x / self.radius, 0., point.z / self.radius),
                            material: self.material,
                            uv: (u, v),
                        })
                    } else {
                        None
                    }
                };
                if let Some(hit) = check_solution(t1) {
                    return Some(hit);
                } else if let Some(t2) = t2 {
                    if let Some(hit) = check_solution(t2) {
                        return Some(hit);
                    }
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(-self.radius, 0., -self.radius),
            Vec3::new(self.radius, self.height, self.radius),
        ))
    }
}

/// Creates a disk facing upwards with a given radius.
/// The `phi_max` parameter can be used to create a sector with the given angle.
/// The `inner_radius` parameter can be used to create an annulus (2D donut).
pub struct Disk {
    radius: f32,
    phi_max: f32,
    inner_radius: f32,
    material: MaterialIdx,
}

impl Disk {
    pub fn new(radius: f32, material: MaterialIdx) -> Disk {
        Disk {
            radius,
            phi_max: 2. * std::f32::consts::PI,
            inner_radius: 0.,
            material,
        }
    }

    pub fn partial(radius: f32, phi: f32, inner_radius: f32, material: MaterialIdx) -> Disk {
        Disk {
            radius,
            phi_max: phi.to_radians(),
            inner_radius,
            material,
        }
    }
}

impl Hitable for Disk {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        // Ignore rays parallel to disk, to avoid divide by zero errors
        if r.direction().y == 0. {
            return None;
        }
        // Solve for t. This is the same thing as
        // x = (y - b)/m, if the ray is a line y = mx + b,
        // exxcept b is 0 (because the ray has already been transformed to object
        // coordinates)
        // This just finds the intersection of the ray and the XZ plane
        let t = -r.origin().y / r.direction().y;
        if t < t_min || t > t_max {
            return None;
        }
        let point = r.point(t);
        // Check if the point on the plane is inside the circle (and outside the inner
        // circle)
        let dist2 = point.x * point.x + point.z * point.z;
        if dist2 > self.radius * self.radius || dist2 < self.inner_radius * self.inner_radius {
            return None;
        }
        let phi = {
            let phi = point.z.atan2(point.x);
            if phi < 0. {
                phi + 2. * std::f32::consts::PI
            } else {
                phi
            }
        };
        if phi > self.phi_max {
            return None;
        }
        let u = phi / self.phi_max;
        let dist = dist2.sqrt();
        let v = 1. - (dist - self.inner_radius) / (self.radius - self.inner_radius);

        Some(RaycastHit {
            t,
            point,
            normal: Vec3::unit_y(),
            material: self.material,
            uv: (u, v),
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(-self.radius, 0., self.radius),
            Vec3::new(-self.radius, 0.001, self.radius),
        ))
    }
}

pub struct Cone {
    radius: f32,
    height: f32,
    material: MaterialIdx,
}

impl Cone {
    pub fn new(radius: f32, height: f32, material: MaterialIdx) -> Cone {
        Cone { radius, height, material }
    }
}

impl Hitable for Cone {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rand: &mut LcRng) -> Option<RaycastHit> {
        let o = *r.origin();
        let d = *r.direction();
        // Derivation (using Sympy)
        // > expr = (h*(ox + t*dx)/r)**2 + (h*(oy + t*dy)/r)**2 - (oz + t*dz - h)**2
        // > collected = collect(expand(expr), t)
        // > collected.coeff(t, 2)
        //      2  2     2  2
        //    dx ⋅h    dy ⋅h      2
        //    ────── + ────── - dz
        //       2        2
        //      r        r
        // > collected.coeff(t, 1)
        //          2            2
        //    2⋅dx⋅h ⋅ox   2⋅dy⋅h ⋅oy
        //    ────────── + ────────── + 2⋅dz⋅h - 2⋅dz⋅oz
        //         2            2
        //        r            r
        // > collected.coeff(t, 0)
        //     2   2    2   2
        //    h ⋅ox    h ⋅oy     2              2
        //    ────── + ────── - h  + 2⋅h⋅oz - oz
        //       2        2
        //      r        r
        // Note that y and z are switched (because in the equation, z is the up direction, while
        // in the renderer, y is).
        let r2_div_h2 = self.radius * self.radius / (self.height * self.height);
        let a = d.x * d.x + d.z * d.z  - r2_div_h2 * d.y * d.y;
        let b = 2. * (d.x * o.x + d.z * o.z - r2_div_h2 * d.y * (o.y - self.height));
        let c = o.x * o.x + o.z * o.z - r2_div_h2 * (o.y - self.height) * (o.y - self.height);

        if let [Some(t1), t2] = solve_quadratic(a, b, c) {
            let check_solution = |t| {
                if t > t_max || t < t_min {
                    return None
                }
                let point = r.point(t);
                if point.y < 0. || point.y > self.height {
                    return None
                }
                let v = point.y / self.height;
                let phi = (point.x / (self.radius * (1. - v))).acos();
                let u = phi / (2. * std::f32::consts::PI);
                let dpdu = Vec3::new(-point.z, 0., point.x);
                let dpdv = Vec3::new(
                    -point.x/(1. - v),
                    self.height,
                    -point.z / (1. - v),
                    );
                return Some(RaycastHit {
                    t,
                    point,
                    normal: dpdv.cross(dpdu).normalized(),
                    material: self.material,
                    uv: (u, v),
                })
            };

            if let Some(hit) = check_solution(t1) {
                return Some(hit);
            } else if let Some(t2) = t2 {
                return check_solution(t2)          
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(-self.radius, 0., -self.radius),
            Vec3::new(self.radius, self.height, self.radius),
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
            material: self.material,
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

enum Rect {
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

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(self.pos, self.pos + self.size))
    }
}

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
