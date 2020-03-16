use crate::material::{ConstantMat, DielectricMat, LambertianMat, MetalMat};
use crate::objects::{ConstantMedium, Rect3d, Sphere, XYRect, XZRect, YZRect};
use crate::render::{RenderObject, Scene};
use crate::texture::*;
use crate::util::InRange;
use image::open;
use tiny_rng::Rand;
use ultraviolet::Vec3;

pub fn cornell_smoke() -> Scene<'static> {
    let mut world = Scene::new();

    let red = world.add_material(LambertianMat::with_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = world.add_material(LambertianMat::with_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = world.add_material(LambertianMat::with_color(Vec3::new(0.12, 0.45, 0.15)));
    let light = world.add_material(ConstantMat::with_color(Vec3::new(7., 7., 7.)));

    world.add_object(RenderObject::new(XZRect::new(
        113., 443., 127., 432., 554., light,
    )));
    world
        .add_object(RenderObject::new(YZRect::new(0., 555., 0., 555., 555., green)).flip_normals());
    world.add_object(RenderObject::new(YZRect::new(0., 555., 0., 555., 0., red)));
    world.add_object(RenderObject::new(XZRect::new(
        0., 555., 0., 555., 0., white,
    )));
    world.add_object(RenderObject::new(XZRect::new(
        0., 555., 0., 555., 555., white,
    )));
    world.add_object(RenderObject::new(XYRect::new(
        0., 555., 0., 555., 555., white,
    )));

    let volume1 = RenderObject::new(ConstantMedium::new(
        Rect3d::with_size(Vec3::new(165., 165., 165.), 0),
        0.01,
        Box::new(ConstantTexture::new(Vec3::one())),
        &mut world,
    ))
    .rotate_y(-18.)
    .position(130., 0., 65.);
    world.add_object(volume1);

    let volume2 = RenderObject::new(ConstantMedium::new(
        Rect3d::with_size(Vec3::new(165., 330., 165.), 0),
        0.01,
        Box::new(ConstantTexture::new(Vec3::zero())),
        &mut world,
    ))
    .rotate_y(15.)
    .position(265., 0., 295.);

    world.add_object(volume2);

    world
}

pub fn cornell_box() -> Scene<'static> {
    let mut world = Scene::new();

    let red = world.add_material(LambertianMat::with_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = world.add_material(LambertianMat::with_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = world.add_material(LambertianMat::with_color(Vec3::new(0.12, 0.45, 0.15)));

    let light = world.add_material(ConstantMat::with_color(Vec3::new(7., 7., 7.)));

    world.add_object(RenderObject::new(XZRect::new(
        113., 443., 127., 432., 554., light,
    )));
    world
        .add_object(RenderObject::new(YZRect::new(0., 555., 0., 555., 555., green)).flip_normals());
    world.add_object(RenderObject::new(YZRect::new(0., 555., 0., 555., 0., red)));
    world.add_object(RenderObject::new(XZRect::new(
        0., 555., 0., 555., 0., white,
    )));
    world
        .add_object(RenderObject::new(XZRect::new(0., 555., 0., 555., 555., white)).flip_normals());
    world
        .add_object(RenderObject::new(XYRect::new(0., 555., 0., 555., 555., white)).flip_normals());
    world.add_object(
        RenderObject::new(Rect3d::with_size(Vec3::new(165., 165., 165.), white))
            .rotate_y(-18.)
            .position(130., 0., 65.),
    );
    world.add_object(
        RenderObject::new(Rect3d::with_size(Vec3::new(165., 330., 165.), white))
            .rotate_y(15.)
            .position(265., 0., 295.),
    );

    world
}

/*

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

*/

pub fn random_scene(rand: &mut impl Rand) -> Scene {
    let mut scene = Scene::new();

    let checker_mat = scene.add_material(LambertianMat::new(CheckerTexture::with_colors(
        Vec3::new(0.2, 0.4, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
        10.,
    )));
    scene.add_object(RenderObject::new(Sphere::new(1000., checker_mat)).position(0., -1000., -1.));

    for x in -11..11 {
        for y in -11..11 {
            let center = Vec3::new(
                x as f32 + 0.9 * rand.rand_f32(),
                0.2,
                y as f32 + 0.9 * rand.rand_f32(),
            );
            if (center - Vec3::new(4., 0.2, 0.9)).mag() > 0.9 {
                let mat = match rand.rand_f32() {
                    x if x.in_range(0.0, 0.8) => {
                        scene.add_material(LambertianMat::with_color(Vec3::new(
                            rand.rand_f32() * rand.rand_f32(),
                            rand.rand_f32() * rand.rand_f32(),
                            rand.rand_f32() * rand.rand_f32(),
                        )))
                    }
                    x if x.in_range(0.8, 0.95) => scene.add_material(MetalMat::new(
                        Vec3::new(
                            0.5 * (1. + rand.rand_f32()),
                            0.5 * (1. + rand.rand_f32()),
                            0.5 * (1. + rand.rand_f32()),
                        ),
                        0.5 * rand.rand_f32(),
                    )),
                    x if x.in_range(0.95, 1.) => scene.add_material(DielectricMat::new(1.5)),
                    _ => unreachable!(),
                };
                scene.add_object(RenderObject::new(Sphere::new(0.2, mat)).position_vec(center));
            }
        }
    }

    let glass = scene.add_material(DielectricMat::new(1.5));
    let diffuse = scene.add_material(LambertianMat::with_color(Vec3::new(0.4, 0.2, 0.1)));
    let metal = scene.add_material(MetalMat::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    scene.add_object(RenderObject::new(Sphere::new(1.0, glass)).position(0., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(1.0, diffuse)).position(-4., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(1.0, metal)).position(4., 1., 0.));

    scene
}
