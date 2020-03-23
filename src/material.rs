use crate::render::RaycastHit;
use crate::texture::{ConstantTexture, Texture};
use crate::util::{random_in_unit_sphere, reflect, refract, schlick};
use crate::ray::Ray;
use tiny_rng::{LcRng, Rand};
use ultraviolet::Vec3;

// IDEA: Currently, to get a Material from it's index, we need to first get the `MaterialLibrary`
// and then call `get_material` on it. It would be more readable if we could simply call `get` on
// the `MaterialIdx` itself. However, we cannot implement functions on aliased types. So instead,
// we could create a `LibraryIndex` trait, that can `get` in a `Library`, and then implement those
// for `MaterialIdx` and `MaterialLibrary` respectively (and perhaps use the same system for other
// pieces)
pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult>;

    fn emit(&self, _uv: (f32, f32), _point: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub struct LambertianMat {
    albedo: Box<dyn Texture + Sync>,
}

impl LambertianMat {
    pub fn new<T: Texture + Sync + 'static>(albedo: T) -> LambertianMat {
        LambertianMat {
            albedo: Box::new(albedo),
        }
    }

    pub fn with_color(albedo: Vec3) -> LambertianMat {
        LambertianMat {
            albedo: Box::new(ConstantTexture::new(albedo)),
        }
    }
}

impl Material for LambertianMat {
    fn scatter(&self, _r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult> {
        let target = hit.point + hit.normal + random_in_unit_sphere(rand);
        let scattered = Ray::new(hit.point, target - hit.point);
        // TODO: Use proper UV Mapping
        let attenuation = self.albedo.sample(hit.uv, &hit.point);
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

pub struct ConstantMat {
    albedo: Box<dyn Texture + Sync>,
}

impl ConstantMat {
    pub fn new(albedo: Box<dyn Texture + Sync>) -> ConstantMat {
        ConstantMat { albedo }
    }

    pub fn with_color(albedo: Vec3) -> ConstantMat {
        ConstantMat {
            albedo: Box::new(ConstantTexture::new(albedo)),
        }
    }
}

impl Material for ConstantMat {
    fn scatter(&self, _r_in: &Ray, _hit: &RaycastHit, _rand: &mut LcRng) -> Option<ScatterResult> {
        None
    }

    fn emit(&self, uv: (f32, f32), point: &Vec3) -> Vec3 {
        self.albedo.sample(uv, point)
    }
}

pub struct IsotropicMat {
    texture: Box<dyn Texture + Sync>,
}

impl IsotropicMat {
    pub fn new(texture: Box<dyn Texture + Sync>) -> Self {
        IsotropicMat { texture }
    }
}

impl Material for IsotropicMat {
    fn scatter(&self, _r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.texture.sample(hit.uv, &hit.point),
            scattered: Ray::new(hit.point, random_in_unit_sphere(rand)),
        })
    }
}
