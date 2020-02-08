use crate::util::{random_in_unit_sphere, reflect, refract, schlick};
use crate::render::RaycastHit;
use crate::Ray;
use tiny_rng::{Rand, LcRng};
use ultraviolet::Vec3;


pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult>;
}

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub struct LambertianMat {
    albedo: Vec3,
}

impl LambertianMat {
    pub fn new(albedo: Vec3) -> LambertianMat {
        LambertianMat { albedo }
    }
}

impl Material for LambertianMat {
    fn scatter(&self, _r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rand);
        let scattered = Ray::new(hit.point, target - hit.point);
        let attenuation = self.albedo;
        Some(ScatterResult {
            scattered,
            attenuation,
        })
    }
}

pub struct MetalMat {
    albedo: Vec3,
    roughness: f32,
}

impl MetalMat {
    pub fn new(albedo: Vec3, roughness: f32) -> MetalMat {
        MetalMat { albedo, roughness }
    }
}

impl Material for MetalMat {
    fn scatter(&self, r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult> {
        let reflected = reflect(r_in.direction(), &hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.roughness * random_in_unit_sphere(rand),
        );
        let attenuation = self.albedo;
        if scattered.direction().dot(hit.normal) > 0. {
            Some(ScatterResult {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}

pub struct DielectricMat {
    ref_idx: f32,
}

impl DielectricMat {
    pub fn new(ref_idx: f32) -> DielectricMat {
        DielectricMat { ref_idx }
    }
}

impl Material for DielectricMat {
    fn scatter(&self, r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult> {
        let reflected = reflect(r_in.direction(), &hit.normal);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction().dot(hit.normal) > 0. {
            (
                -hit.normal,
                self.ref_idx,
                self.ref_idx * r_in.direction().dot(hit.normal) / r_in.direction().mag(),
            )
        } else {
            (
                hit.normal,
                1.0 / self.ref_idx,
                -r_in.direction().dot(hit.normal) / r_in.direction().mag(),
            )
        };

        if let Some(refracted) = refract(r_in.direction(), &outward_normal, ni_over_nt) {
            if rand.rand_f32() > schlick(cosine, self.ref_idx) {
                return Some(ScatterResult {
                    scattered: Ray::new(hit.point, refracted),
                    attenuation: Vec3::one(),
                });
            }
        }
        Some(ScatterResult {
            scattered: Ray::new(hit.point, reflected),
            attenuation: Vec3::one(),
        })
    }
}
