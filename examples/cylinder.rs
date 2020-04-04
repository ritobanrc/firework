use firework::camera::CameraSettings;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::{Cylinder, XZRect, YZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::texture::ImageTexture;
use firework::window::RenderWindow;
use image::open;
use ultraviolet::{Rotor3, Vec3};

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

pub fn objects_scene() -> Scene<'static> {
    let mut scene = Scene::new();

    let uvmap = open("uvmap.png").unwrap();
    let uv_image_mat = scene.add_material(LambertianMat::new(ImageTexture::new(uvmap)));
    scene.add_object(RenderObject::new(Cylinder::new(2., 3., uv_image_mat)).position(-2.8, 0., 0.));
    scene.add_object(
        RenderObject::new(Cylinder::new(2., 3., uv_image_mat))
            .position(3.0, 1., 1.)
            .rotate(Rotor3::from_euler_angles(
                90f32.to_radians(),
                0.,
                -35f32.to_radians(),
            )),
    );

    let grey = scene.add_material(LambertianMat::with_color(Vec3::broadcast(0.5)));
    scene.add_object(RenderObject::new(XZRect::new(
        -100., 100., -100., 100., 0., grey,
    )));

    let light = scene.add_material(EmissiveMat::with_color(Vec3::broadcast(20.)));
    scene.add_object(
        RenderObject::new(YZRect::new(0., 20., 0., 10., -3., light))
            .rotate(Rotor3::from_rotation_xz(-30.))
            .position(0., 0., -10.),
    );

    scene.set_environment(sky_color);
    scene
}

fn main() {
    let scene = objects_scene();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(6., 4., -6.))
        .look_at(Vec3::new(0., 1.5, 0.))
        .field_of_view(60.);
    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(1000)
        .camera(camera);

    let render = renderer.render(&scene);

    let window = RenderWindow::new(
        "Cylinder",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
