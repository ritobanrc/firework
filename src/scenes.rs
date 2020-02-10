use crate::material::{DielectricMat, LambertianMat, MetalMat};
use crate::render::{HitableList, Sphere};
use crate::util::InRange;
use crate::texture::{CheckerTexture, ConstantTexture, PerlinNoiseTexture};
use tiny_rng::Rand;
use ultraviolet::Vec3;

pub fn two_spheres_checker() -> HitableList {
    let mut world = HitableList::new();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -10., -1.),
        10.,
        Box::new(LambertianMat::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            10.
        )))),
    )));
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 10., -1.),
        10.,
        Box::new(LambertianMat::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            10.
        )))),
    )));
    world
}


pub fn two_spheres_perlin() -> HitableList {
    let mut world = HitableList::new();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Box::new(LambertianMat::new(Box::new(PerlinNoiseTexture::new(5.)))),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Box::new(LambertianMat::new(Box::new(PerlinNoiseTexture::new(10.)))),
    )));
    world
}

pub fn random_scene(rand: &mut impl Rand) -> HitableList {
    let mut world = HitableList::new();

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -1000., -1.),
        1000.,
        Box::new(LambertianMat::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            10.
        )))),
    )));

    for x in -11..11 {
        for y in -11..11 {
            let center = Vec3::new(
                x as f32 + 0.9 * rand.rand_f32(),
                0.2,
                y as f32 + 0.9 * rand.rand_f32(),
            );
            if (center - Vec3::new(4., 0.2, 0.9)).mag() > 0.9 {
                match rand.rand_f32() {
                    x if x.in_range(0.0, 0.8) => {
                        world.list_mut().push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Box::new(LambertianMat::with_color(Vec3::new(
                                rand.rand_f32() * rand.rand_f32(),
                                rand.rand_f32() * rand.rand_f32(),
                                rand.rand_f32() * rand.rand_f32(),
                            ))),
                        )));
                    }
                    x if x.in_range(0.8, 0.95) => {
                        world.list_mut().push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Box::new(MetalMat::new(
                                Vec3::new(
                                    0.5 * (1. + rand.rand_f32()),
                                    0.5 * (1. + rand.rand_f32()),
                                    0.5 * (1. + rand.rand_f32()),
                                ),
                                0.5 * rand.rand_f32(),
                            )),
                        )));
                    }
                    x if x.in_range(0.95, 1.) => {
                        world.list_mut().push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Box::new(DielectricMat::new(1.5)),
                        )));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.0,
        Box::new(DielectricMat::new(1.5)),
    )));
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.0,
        Box::new(LambertianMat::with_color(Vec3::new(0.4, 0.2, 0.1))),
    )));
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(4., 1., 0.),
        1.0,
        Box::new(MetalMat::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}
