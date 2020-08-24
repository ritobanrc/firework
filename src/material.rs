use crate::ray::Ray;
use crate::render::RaycastHit;
use crate::serde_compat::Vec3Def;
use crate::texture::{ConstantTexture, Texture};
use crate::util::{random_in_unit_sphere, reflect, refract, schlick};
use serde::{Deserialize, Serialize};
use tiny_rng::{LcRng, Rand};
use ultraviolet::{Vec2, Vec3};

#[typetag::serde(tag = "material")]
pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult>;

    fn emit(&self, _uv: Vec2, _point: &Vec3) -> Vec3 {
        Vec3::zero()
    }
}

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

/// Represents a diffuse (Lambertian) material.
/// This is a completely "flat" material, like a piece of paper, because light is equally likely
/// to be scattered in all directions.
#[derive(Serialize, Deserialize)]
pub struct LambertianMat {
    albedo: Box<dyn Texture + Sync>,
}

impl LambertianMat {
    /// Crates a new Lambertian Material with a given albedo texture.
    /// ```
    /// use firework::material::LambertianMat;
    /// use firework::texture::ConstantTexture;
    /// use ultraviolet::Vec3;
    ///
    /// let red_material = LambertianMat::new(ConstantTexture::new(Vec3::new(1., 0., 0.)));
    /// ```
    pub fn new<T: Texture + Sync + 'static>(albedo: T) -> LambertianMat {
        LambertianMat {
            albedo: Box::new(albedo),
        }
    }

    /// Crates a new Lambertian Material with a given albedo color. Equivalent to
    /// a `ConstantTexture`
    /// color)
    /// ```
    /// use firework::material::LambertianMat;
    /// use firework::texture::ConstantTexture;
    /// use ultraviolet::Vec3;
    ///
    /// let red_material = LambertianMat::with_color(Vec3::new(1., 0., 0.));
    /// ```
    pub fn with_color(albedo: Vec3) -> LambertianMat {
        LambertianMat {
            albedo: Box::new(ConstantTexture::new(albedo)),
        }
    }
}

#[typetag::serde]
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

#[derive(Serialize, Deserialize)]
pub struct MetalMat {
    #[serde(with = "Vec3Def")]
    albedo: Vec3,
    roughness: f32,
}

impl MetalMat {
    pub fn new(albedo: Vec3, roughness: f32) -> MetalMat {
        MetalMat { albedo, roughness }
    }
}

#[typetag::serde]
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

#[derive(Serialize, Deserialize)]
pub struct DielectricMat {
    ref_idx: f32,
}

impl DielectricMat {
    pub fn new(ref_idx: f32) -> DielectricMat {
        DielectricMat { ref_idx }
    }
}

#[typetag::serde]
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

#[derive(Serialize, Deserialize)]
pub struct EmissiveMat {
    albedo: Box<dyn Texture + Sync>,
}

impl EmissiveMat {
    pub fn new<T: Texture + Sync + 'static>(albedo: T) -> EmissiveMat {
        EmissiveMat {
            albedo: Box::new(albedo),
        }
    }

    pub fn with_color(albedo: Vec3) -> EmissiveMat {
        EmissiveMat {
            albedo: Box::new(ConstantTexture::new(albedo)),
        }
    }
}

#[typetag::serde]
impl Material for EmissiveMat {
    fn scatter(&self, _r_in: &Ray, _hit: &RaycastHit, _rand: &mut LcRng) -> Option<ScatterResult> {
        None
    }

    fn emit(&self, uv: Vec2, point: &Vec3) -> Vec3 {
        self.albedo.sample(uv, point)
    }
}

#[derive(Serialize, Deserialize)]
pub struct IsotropicMat {
    texture: Box<dyn Texture + Sync>,
}

impl IsotropicMat {
    pub fn new(texture: Box<dyn Texture + Sync>) -> Self {
        IsotropicMat { texture }
    }
}

#[typetag::serde]
impl Material for IsotropicMat {
    fn scatter(&self, _r_in: &Ray, hit: &RaycastHit, rand: &mut LcRng) -> Option<ScatterResult> {
        Some(ScatterResult {
            attenuation: self.texture.sample(hit.uv, &hit.point),
            scattered: Ray::new(hit.point, random_in_unit_sphere(rand)),
        })
    }
}
