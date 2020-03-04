use crate::material::{ConstantMat, DielectricMat, LambertianMat, MetalMat};
use crate::render::{
    FlipNormals, HitableList, Rect3d, RotateY, Sphere, Translate, XYRect, XZRect, YZRect, ConstantMedium,
};
use crate::texture::*;
use crate::util::InRange;
use image::open;
use tiny_rng::Rand;
use ultraviolet::Vec3;



pub fn cornell_smoke() -> HitableList {
    let mut world = HitableList::new();

    let red = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))));
    let white1 = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))));
    let white2 = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))));
    let white3 = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))));
    let green = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15))));

    let light = ConstantMat::new(Box::new(ConstantTexture::new(Vec3::new(7., 7., 7.))));

    world
        .list_mut()
        .push(Box::new(FlipNormals::new(Box::new(YZRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            Box::new(green),
        )))));
    world
        .list_mut()
        .push(Box::new(YZRect::new(0., 555., 0., 555., 0., Box::new(red))));

    world.list_mut().push(Box::new(XZRect::new(
        113.,
        443.,
        127.,
        432.,
        554.,
        Box::new(light),
    )));

    world.list_mut().push(Box::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        Box::new(white1),
    )));
    world
        .list_mut()
        .push(Box::new(FlipNormals::new(Box::new(XZRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            Box::new(white2),
        )))));

    world
        .list_mut()
        .push(Box::new(FlipNormals::new(Box::new(XYRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            Box::new(white3),
        )))));

    let b1 = Box::new(Translate::new(
        Box::new(RotateY::new(
            -18.,
            Box::new(Rect3d::with_size(Vec3::new(165., 165., 165.), || {
                Box::new(LambertianMat::new(Box::new(ConstantTexture::new(
                    Vec3::new(0.73, 0.73, 0.73),
                ))))
            })),
        )),
        Vec3::new(130., 0., 65.),
    ));


    let b2 = Box::new(Translate::new(
        Box::new(RotateY::new(
            15.,
            Box::new(Rect3d::with_size(Vec3::new(165., 330., 165.), || {
                Box::new(LambertianMat::new(Box::new(ConstantTexture::new(
                    Vec3::new(0.73, 0.73, 0.73),
                ))))
            })),
        )),
        Vec3::new(265., 0., 295.),
    ));

    world.list_mut().push(Box::new(ConstantMedium::new(b1, 0.01, Box::new(ConstantTexture::new(Vec3::one())))));
    world.list_mut().push(Box::new(ConstantMedium::new(b2, 0.01, Box::new(ConstantTexture::new(Vec3::zero())))));

    world
}


pub fn cornell_box() -> HitableList {
    let mut world = HitableList::new();

    let red = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05))));
    let white1 = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))));
    let white2 = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))));
    let white3 = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))));
    //let white3 = MetalMat::new(Vec3::new(0.73, 0.73, 0.73), 0.5);
    let green = LambertianMat::new(Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15))));

    let light = ConstantMat::new(Box::new(ConstantTexture::new(Vec3::new(15., 15., 15.))));

    world
        .list_mut()
        .push(Box::new(FlipNormals::new(Box::new(YZRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            Box::new(green),
        )))));
    world
        .list_mut()
        .push(Box::new(YZRect::new(0., 555., 0., 555., 0., Box::new(red))));

    world.list_mut().push(Box::new(XZRect::new(
        213.,
        343.,
        227.,
        332.,
        554.,
        Box::new(light),
    )));

    world.list_mut().push(Box::new(XZRect::new(
        0.,
        555.,
        0.,
        555.,
        0.,
        Box::new(white1),
    )));
    world
        .list_mut()
        .push(Box::new(FlipNormals::new(Box::new(XZRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            Box::new(white2),
        )))));

    world
        .list_mut()
        .push(Box::new(FlipNormals::new(Box::new(XYRect::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            Box::new(white3),
        )))));

    world.list_mut().push(Box::new(Translate::new(
        Box::new(RotateY::new(
            -18.,
            Box::new(Rect3d::with_size(Vec3::new(165., 165., 165.), || {
                Box::new(LambertianMat::new(Box::new(ConstantTexture::new(
                    Vec3::new(0.73, 0.73, 0.73),
                ))))
            })),
        )),
        Vec3::new(130., 0., 65.),
    )));

    world.list_mut().push(Box::new(Translate::new(
        Box::new(RotateY::new(
            15.,
            Box::new(Rect3d::with_size(Vec3::new(165., 330., 165.), || {
                Box::new(LambertianMat::new(Box::new(ConstantTexture::new(
                    Vec3::new(0.73, 0.73, 0.73),
                ))))
            })),
        )),
        Vec3::new(265., 0., 295.),
    )));

    world
}

pub fn cubes() -> HitableList {
    let mut world = HitableList::new();

    world.list_mut().push(Box::new(RotateY::new(
        30.,
        Box::new(Rect3d::new(
            Vec3::new(400., 0., 65.),
            Vec3::new(165., 165., 165.),
            || {
                Box::new(LambertianMat::new(Box::new(ConstantTexture::new(
                    Vec3::new(0.73, 0.73, 0.73),
                ))))
            },
        )),
    )));

    world.list_mut().push(Box::new(Rect3d::new(
        Vec3::new(20., 500., 295.),
        Vec3::new(165., 330., 165.),
        || {
            Box::new(LambertianMat::new(Box::new(ConstantTexture::new(
                Vec3::new(0.73, 0.73, 0.73),
            ))))
        },
    )));

    world
}

pub fn two_spheres_checker() -> HitableList {
    let mut world = HitableList::new();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -10., -1.),
        10.,
        Box::new(LambertianMat::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            10.,
        )))),
    )));
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 10., -1.),
        10.,
        Box::new(LambertianMat::new(Box::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
            10.,
        )))),
    )));
    world
}

pub fn two_spheres_perlin() -> HitableList {
    let mut world = HitableList::new();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Box::new(ConstantMat::new(Box::new(PerlinNoiseTexture::new(1.)))),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Box::new(ConstantMat::new(Box::new(PerlinNoiseTexture::new(2.)))),
    )));
    world
}

pub fn two_spheres_turb() -> HitableList {
    let mut world = HitableList::new();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Box::new(LambertianMat::new(Box::new(TurbulenceTexture::new(4, 1.)))),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Box::new(ConstantMat::new(Box::new(MarbleTexture::new(7, 5.)))),
    )));
    world
}

pub fn earth_scene() -> HitableList {
    let mut world = HitableList::new();
    let image = open("./earthmap.jpg").unwrap();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 0., 0.),
        2.,
        Box::new(ConstantMat::new(Box::new(ImageTexture::new(image)))),
    )));
    world
}

pub fn light_scene() -> HitableList {
    let mut world = HitableList::new();
    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        Box::new(LambertianMat::new(Box::new(TurbulenceTexture::new(4, 1.)))),
    )));

    world.list_mut().push(Box::new(Sphere::new(
        Vec3::new(0., 2., 0.),
        2.,
        Box::new(LambertianMat::new(Box::new(MarbleTexture::new(7, 5.)))),
    )));

    world.list_mut().push(Box::new(XYRect::new(
        3.,
        5.,
        1.,
        3.,
        -2.,
        Box::new(ConstantMat::new(Box::new(ConstantTexture::new(Vec3::new(
            4., 4., 4.,
        ))))),
    )));
    world.list_mut().push(Box::new(XZRect::new(
        3.,
        5.,
        1.,
        3.,
        4.,
        Box::new(ConstantMat::new(Box::new(ConstantTexture::new(Vec3::new(
            4., 2., 0.,
        ))))),
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
            10.,
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
