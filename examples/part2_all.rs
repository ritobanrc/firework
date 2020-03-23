use firework::camera::CameraSettings;
use firework::render::Renderer;
use firework::scene::Scene;
use firework::window::RenderWindow;
use std::time;
use tiny_rng::{Rng, Rand};
use ultraviolet::Vec3;

fn final_scene(rand: &mut impl Rand) -> Scene<'static> {
    let mut scene = Scene::new();
    scene
}

fn main() {
    let mut rng = Rng::new(12345);
    let scene = final_scene(&mut rng);


    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(13., 2., 3.))
        .look_at(Vec3::zero())
        .aperture(0.1);

    let renderer = Renderer::default()
        .width(1920)
        .height(1080)
        .samples(100)
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

