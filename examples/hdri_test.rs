use firework::camera::CameraSettings;
use firework::material::{DielectricMat, LambertianMat, MetalMat};
use firework::objects::{Sphere, XZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::window::RenderWindow;
use std::f32::consts::PI;
use std::time;
use ultraviolet::Vec3;

pub fn sphere_uv(point: &Vec3) -> (f32, f32) {
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    (u, v)
}

// TODO: Properly create an `environment` module in firework that handles all this
// NOTE: Currently, there isn't importance sampling, so even with insane numbers of samples, it's impossible to get accurate HDR lighting that isn't noisy.
pub fn hdri_test() -> Scene<'static> {
    use image::hdr::HDRDecoder;
    use std::fs::File;
    use std::io::BufReader;

    let mut scene = Scene::new();

    let hdri = File::open("urban_street_04_4k.hdr").unwrap();
    let hdri = BufReader::new(hdri);
    let hdri = HDRDecoder::new(hdri).unwrap();

    let hdri_width = hdri.metadata().width as f32;
    let hdri_height = hdri.metadata().height as f32;

    let pixels = hdri.read_image_hdr().unwrap();

    scene.set_environment(move |dir| {
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

fn main() {
    let scene = hdri_test();

    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(0., 2., -10.))
        .look_at(Vec3::zero());

    let renderer = Renderer::default()
        .width(200)
        .height(100)
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
