use firework::camera::CameraSettings;
use firework::render::Renderer;
use firework::objects::{Sphere, Rect3d, XZRect, ConstantMedium};
use firework::texture::{ConstantTexture, ImageTexture, TurbulenceTexture};
use firework::{Scene, RenderObject};
use firework::material::{LambertianMat, EmissiveMat, DielectricMat, MetalMat};
use firework::window::{save_image, RenderWindow};
use std::time;
use tiny_rng::{Rng, Rand};
use ultraviolet::{Vec3, Rotor3};
use image::open;

fn final_scene(rand: &mut impl Rand) -> Scene<'static> {
    let mut scene = Scene::new();

    //let mut subscene = Scene::new();

    let ground = scene.add_material(LambertianMat::with_color(Vec3::new(0.48, 0.83, 0.53)));

    let origin = Vec3::new(-10., 0., -10.);
    for x in 0..20 {
        for z in 0..20 {
            let pos = origin + Vec3::new(x as f32, 0., z as f32);
            let size = Vec3::new(1., rand.rand_f32() + 0.01, 1.);
            scene.add_object(
                RenderObject::new(Rect3d::with_size(size, ground))
                    .position_vec(pos)
            );
        }
    }

    let light = scene.add_material(EmissiveMat::with_color(7. * Vec3::one()));
    scene.add_object(RenderObject::new(XZRect::new(1.23, 4.23, 1.47, 4.12, 5.54, light)));

    let brown = scene.add_material(LambertianMat::with_color(Vec3::new(0.7, 0.3, 0.1)));
    scene.add_object(RenderObject::new(Sphere::new(0.5, brown)).position(4., 4., 2.));

    let glass = scene.add_material(DielectricMat::new(1.5));
    scene.add_object(RenderObject::new(Sphere::new(0.5, glass)).position(2.6, 1.5, 0.45));

    let metal = scene.add_material(MetalMat::new(Vec3::new(0.8, 0.8, 0.9), 10.));
    scene.add_object(RenderObject::new(Sphere::new(0.5, metal)).position(0., 1.5, 1.45));

    scene.add_object(RenderObject::new(Sphere::new(0.7, glass)).position(3.6, 1.5, 1.45));
    let volume = ConstantMedium::new(
                RenderObject::new(Sphere::new(0.7, glass)).position(3.6, 1.5, 1.45),
                0.2, 
                Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
                &mut scene
                );
    scene.add_object(RenderObject::new(volume));

    let image = open("./earthmap.jpg").unwrap();
    let earth_mat = scene.add_material(LambertianMat::new(ImageTexture::new(image)));
    scene.add_object(RenderObject::new(Sphere::new(1., earth_mat)).position(4., 2., 4.));

    let noise = TurbulenceTexture::new(5, 10.);
    let noise = scene.add_material(LambertianMat::new(noise));
    scene.add_object(RenderObject::new(Sphere::new(0.8, noise)).position(2.2,2.8, 3.0));


    let mut subscene = Scene::new();
    let white = subscene.add_material(LambertianMat::with_color(0.73 * Vec3::one()));
    for _ in 0..1000 {
        let pos = 1.65 * Vec3::new(rand.rand_f32(), rand.rand_f32(), rand.rand_f32());
        subscene.add_object(RenderObject::new(Sphere::new(0.1, white)).position_vec(pos));
    }

    scene.add_object(RenderObject::new(subscene).rotate(Rotor3::from_rotation_xz(15.)).position(1., 2.7, 3.95));

    let volume = ConstantMedium::new(
                RenderObject::new(Sphere::new(5000., 0)),
                0.0001, 
                Box::new(ConstantTexture::new(Vec3::one())),
                &mut scene
                );

    scene.add_object(RenderObject::new(volume));


    scene
}

fn main() {
    let mut rng = Rng::new(12345);
    let scene = final_scene(&mut rng);


    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(-9., 3., -9.))
        .look_at(Vec3::new(1., 3., 2.))
        .field_of_view(25.);

    let renderer = Renderer::default()
        .width(600)
        .height(800)
        .samples(10000)
        .use_bvh(true)
        .camera(camera);

    let render = renderer.render(&scene);

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    save_image(&render, "part2_final.png", renderer.width, renderer.height);


    let window = RenderWindow::new(
        "Part 2 Final Scene",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}

