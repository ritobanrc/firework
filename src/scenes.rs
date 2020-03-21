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

pub fn cornell_box() -> Scene<'static> {
    //let cam_pos = Vec3::new(278., 278., -800.);
    //let look_at = Vec3::new(278., 278., 0.);
    //let camera = Camera::new(cam_pos, look_at, Vec3::unit_y(), 40.0, 0.0, 10.);

    let mut world = Scene::new();

    let red = world.add_material(LambertianMat::with_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = world.add_material(LambertianMat::with_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = world.add_material(LambertianMat::with_color(Vec3::new(0.12, 0.45, 0.15)));

    let light = world.add_material(ConstantMat::with_color(Vec3::new(15., 15., 15.)));

    world.add_object(RenderObject::new(XZRect::new(
        213., 343., 227., 332., 554., light,
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
            .rotate(Rotor3::from_rotation_xz(18_f32.to_radians()))
            .position(130., 0., 65.),
    );
    world.add_object(
        RenderObject::new(Rect3d::with_size(Vec3::new(165., 330., 165.), white))
            .rotate(Rotor3::from_rotation_xz(-15_f32.to_radians()))
            .position(265., 0., 295.),
    );

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

pub fn random_scene(rand: &mut impl Rand) -> Scene {
    //let cam_pos = Vec3::new(13., 2., 3.);
    //let look_at = Vec3::new(0., 0., 0.);
    //let camera = Camera::new(cam_pos, look_at, Vec3::unit_y(), 40.0, 0.0, 10.);
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

    scene.set_environment(sky_color);

    scene
}

/// A function that creates a basic sky gradient between SKY_BLUE and SKY_WHITE
/// TODO: Don't have hardcoded SKY_BLUE and SKY_WHITE colors.
fn sky_color(r: &Ray) -> Vec3 {
    const SKY_BLUE: Vec3 = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.0,
    };
    const SKY_WHITE: Vec3 = Vec3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    let dir = r.direction().normalized();
    // Take the y (from -1 to +1) and map it to 0..1
    let t = 0.5 * (dir.y + 1.0);
    (1. - t) * SKY_WHITE + t * SKY_BLUE
}
