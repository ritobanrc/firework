use firework::camera::CameraSettings;
use firework::environment::SkyEnv;
use firework::material::{DielectricMat, LambertianMat, MetalMat};
use firework::objects::Sphere;
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::texture::CheckerTexture;
use firework::window::RenderWindow;
use std::time;
use tiny_rng::{Rand, Rng};
use ultraviolet::Vec3;

/// The famous scence on the cover of the "Raytracing in a Weekend Book"
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
                    x if x > 0.0 && x < 0.8 => {
                        scene.add_material(LambertianMat::with_color(Vec3::new(
                            rand.rand_f32() * rand.rand_f32(),
                            rand.rand_f32() * rand.rand_f32(),
                            rand.rand_f32() * rand.rand_f32(),
                        )))
                    }
                    x if x > 0.8 && x < 0.95 => scene.add_material(MetalMat::new(
                        Vec3::new(
                            0.5 * (1. + rand.rand_f32()),
                            0.5 * (1. + rand.rand_f32()),
                            0.5 * (1. + rand.rand_f32()),
                        ),
                        0.5 * rand.rand_f32(),
                    )),
                    x if x > 0.95 && x < 1. => scene.add_material(DielectricMat::new(1.5)),
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

    scene.set_environment(SkyEnv::default());

    scene
}

fn main() {
    let mut rng = Rng::new(12345);
    let scene = random_scene(&mut rng);

    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(13., 2., 3.))
        .look_at(Vec3::zero())
        .aperture(0.1);

    let renderer = Renderer::default()
        .width(400)
        .height(200)
        .samples(32)
        .use_bvh(true)
        .camera(camera);

    let render = renderer.render(&scene);

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let window = RenderWindow::new(
        "Random Spheres",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
