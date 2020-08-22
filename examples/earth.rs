use firework::camera::CameraSettings;
use firework::environment::SkyEnv;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::{Sphere, XZRect, YZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::texture::ImageTexture;
use firework::window::RenderWindow;
use image::open;
use ultraviolet::{Rotor3, Vec3};

pub fn earth_scene() -> Scene {
    let mut scene = Scene::new();
    let image = open("./earthmap.jpg").unwrap();
    let earth_mat = scene.add_material(LambertianMat::new(ImageTexture::new(image)));

    let uvmap = open("uvmap.png").unwrap();
    let uv_image_mat = scene.add_material(LambertianMat::new(ImageTexture::new(uvmap)));

    scene.add_object(RenderObject::new(Sphere::new(0.25, earth_mat)));

    scene.add_object(RenderObject::new(Sphere::new(0.25, uv_image_mat)).position(1., 0., 0.));
    scene.add_object(RenderObject::new(Sphere::new(0.25, earth_mat)).position(0., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(0.25, earth_mat)).position(0., 0., 1.));

    let grey = scene.add_material(LambertianMat::with_color(Vec3::broadcast(0.5)));
    scene.add_object(RenderObject::new(XZRect::new(
        -100., 100., -100., 100., 0., grey,
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
    let scene = earth_scene();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(5., 5., 5.))
        .look_at(Vec3::zero())
        .field_of_view(30.);
    let renderer = Renderer::default()
        .width(800)
        .height(800)
        .samples(128)
        .camera(camera);

    let render = renderer.render(&scene);

    let window = RenderWindow::new("Earth", Default::default(), renderer.width, renderer.height);

    window.display(&render);
}
