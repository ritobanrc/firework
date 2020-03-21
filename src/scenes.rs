use crate::material::{ConstantMat, DielectricMat, LambertianMat, MetalMat};
use crate::objects::{ConstantMedium, Rect3d, Sphere, XYRect, XZRect, YZRect};
use crate::ray::Ray;
use crate::render::{RenderObject, Scene};
use crate::texture::*;
use crate::util::InRange;
use image::open;
use tiny_rng::Rand;
use ultraviolet::{Rotor3, Vec3};

pub fn hdri_test() -> Scene<'static> {
    use crate::util::sphere_uv;
    use image::hdr::HDRDecoder;
    use std::fs::File;
    use std::io::BufReader;

    //let cam_pos = Vec3::new(0., 2., -10.);
    //let look_at = Vec3::zero();
    //let camera = Camera::new(cam_pos, look_at, Vec3::unit_y(), 60., 0.0, 15.);

    let mut scene = Scene::new();

    let hdri = File::open("urban_street_04_4k.hdr").unwrap();
    let hdri = BufReader::new(hdri);
    let hdri = HDRDecoder::new(hdri).unwrap();

    let hdri_width = hdri.metadata().width as f32;
    let hdri_height = hdri.metadata().height as f32;

    let pixels = hdri.read_image_hdr().unwrap();

    scene.set_environment(move |r| {
        let dir = r.direction().normalized();
        let uv = sphere_uv(&dir);

        let x = (uv.0 * hdri_width) as usize;
        let y = ((1. - uv.1) * hdri_height) as usize;

        let idx = (y as f32 * hdri_width) as usize + x;

        pixels[idx].0.into()
    });

    let glass = scene.add_material(DielectricMat::new(1.5));
    let diffuse = scene.add_material(LambertianMat::with_color(Vec3::new(0.8, 0.8, 0.8)));
    let metal = scene.add_material(MetalMat::new(Vec3::new(0.7, 0.7, 0.7), 0.0));

    scene.add_object(RenderObject::new(Sphere::new(1.0, glass)).position(0., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(1.0, diffuse)).position(-4., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(1.0, metal)).position(4., 1., 0.));

    scene.add_object(RenderObject::new(XZRect::new(
        -100., 100., -100., 100., 0., diffuse,
    )));

    scene
}

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
    world
        .add_object(RenderObject::new(XZRect::new(0., 555., 0., 555., 555., white)).flip_normals());
    world
        .add_object(RenderObject::new(XYRect::new(0., 555., 0., 555., 555., white)).flip_normals());

    let volume1 = RenderObject::new(ConstantMedium::new(
        Rect3d::with_size(Vec3::new(165., 165., 165.), 0),
        0.01,
        Box::new(ConstantTexture::new(Vec3::one())),
        &mut world,
    ))
    .rotate(Rotor3::from_rotation_xz(18_f32.to_radians()))
    .position(130., 0., 65.);
    world.add_object(volume1);

    let volume2 = RenderObject::new(ConstantMedium::new(
        Rect3d::with_size(Vec3::new(165., 330., 165.), 0),
        0.01,
        Box::new(ConstantTexture::new(Vec3::zero())),
        &mut world,
    ))
    .rotate(Rotor3::from_rotation_xz(-15_f32.to_radians()))
    .position(265., 0., 295.);

    world.add_object(volume2);

    world
}

pub fn earth_scene() -> Scene<'static> {
    //let cam_pos = Vec3::new(13., 2., 3.);
    //let look_at = Vec3::new(0., 0., 0.);
    //let camera = Camera::new(cam_pos, look_at, Vec3::unit_y(), 40.0, 0.0, 10.);
    let mut scene = Scene::new();
    let image = open("./earthmap.jpg").unwrap();
    let earth_mat = scene.add_material(LambertianMat::new(ImageTexture::new(image)));
    scene.add_object(RenderObject::new(Sphere::new(2., earth_mat)));

    scene.set_environment(sky_color);
    scene
}

