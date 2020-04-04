use firework::camera::CameraSettings;
use firework::material::LambertianMat;
use firework::objects::Sphere;
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::texture::ImageTexture;
use firework::window::RenderWindow;
use image::open;
use ultraviolet::Vec3;

/// A function that creates a basic sky gradient between SKY_BLUE and SKY_WHITE
/// TODO: Don't have hardcoded SKY_BLUE and SKY_WHITE colors.
fn sky_color(dir: Vec3) -> Vec3 {
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

    // Take the y (from -1 to +1) and map it to 0..1
    let t = 0.5 * (dir.y + 1.0);
    (1. - t) * SKY_WHITE + t * SKY_BLUE
}

pub fn earth_scene() -> Scene<'static> {
    let mut scene = Scene::new();
    let image = open("./earthmap.jpg").unwrap();
    let earth_mat = scene.add_material(LambertianMat::new(ImageTexture::new(image)));
    scene.add_object(RenderObject::new(Sphere::new(2., earth_mat)));

    scene.set_environment(sky_color);
    scene
}

fn main() {
    let scene = earth_scene();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(0., 0., -10.))
        .look_at(Vec3::zero())
        .field_of_view(30.);
    let renderer = Renderer::default()
        .width(300)
        .height(300)
        .samples(32)
        .camera(camera);

    let render = renderer.render(&scene);

    let window = RenderWindow::new("Earth", Default::default(), renderer.width, renderer.height);

    window.display(&render);
}
