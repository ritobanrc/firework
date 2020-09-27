use firework::camera::CameraSettings;
use firework::environment::SkyEnv;
use firework::material::{DielectricMat, EmissiveMat, LambertianMat, MetalMat};
use firework::objects::{ConstantMedium, Rect3d, Sphere, XZRect, YZRect};
use firework::render::Renderer;
use firework::texture::{ConstantTexture, ImageTexture, TurbulenceTexture};
use firework::window::RenderWindow;
use firework::{RenderObject, Scene};
use ultraviolet::{Rotor3, Vec3};

pub fn volume_scene() -> Scene {
    let mut scene = Scene::new();

    let glass = scene.add_material(DielectricMat::new(1.5));
    let diffuse = scene.add_material(LambertianMat::with_color(Vec3::new(0.8, 0.8, 0.8)));
    let metal = scene.add_material(MetalMat::new(Vec3::new(0.7, 0.7, 0.7), 0.0));

    //scene.add_object(RenderObject::new(Sphere::new(1.0, glass)).position(0., 1., 0.));
    scene.add_volume(
        RenderObject::new(Sphere::new(1.0, diffuse)).position(0., 1., 0.),
        0.5,
        ConstantTexture::from_rgb(0.5, 0.0, 0.8),
    );

    scene.add_object(RenderObject::new(Sphere::new(1.01, glass)).position(0., 1., 1.));

    //scene.add_object(RenderObject::new(Sphere::new(1.0, metal)).position(4., 1., 0.));

    scene.add_object(RenderObject::new(XZRect::new(
        -100., 100., -100., 100., 0., diffuse,
    )));

    let light = scene.add_material(EmissiveMat::with_color(Vec3::broadcast(8.)));
    scene.add_object(
        RenderObject::new(YZRect::new(0., 20., 0., 10., -3., light))
            .rotate(Rotor3::from_rotation_xz(-30.))
            .position(0., 0., -10.),
    );

    scene.set_environment(SkyEnv::default());

    scene
}

fn main() {
    let scene = volume_scene();

    use std::io::Write;
    let mut file = std::fs::File::create("volume.yml").unwrap();
    file.write_all(serde_yaml::to_string(&scene).unwrap().as_bytes())
        .unwrap();

    let mut a = String::new();
    std::io::stdin().read_line(&mut a).unwrap();

    let file = std::fs::File::open("volume.yml").unwrap();
    let scene: Scene = serde_yaml::from_reader(file).unwrap();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(0., 2., -10.))
        .look_at(Vec3::zero());

    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(2048)
        .camera(camera);

    let render = renderer.render(scene);

    let window = RenderWindow::new(
        "volume",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
