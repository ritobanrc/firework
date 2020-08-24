use firework::camera::CameraSettings;
use firework::environment::SkyEnv;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::{Cone, Cylinder, Disk, XZRect, YZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::texture::ImageTexture;
use firework::window::RenderWindow;
use ultraviolet::{Rotor3, Vec3};

pub fn objects_scene() -> Scene {
    let mut scene = Scene::new();

    let uv_image_mat = scene.add_material(LambertianMat::new(
        ImageTexture::from_path("uvmap.png").unwrap(),
    ));
    scene.add_object(RenderObject::new(Cylinder::new(2., 3., uv_image_mat)).position(-4.2, 0., 0.));

    let blue = scene.add_material(LambertianMat::with_color(Vec3::new(0., 0.2, 0.4)));
    scene.add_object(RenderObject::new(Disk::new(2., blue)).position(-4.2, 3., 0.));

    scene.add_object(RenderObject::new(Cone::new(2., 3., uv_image_mat)).position(-1., 0., -4.));

    // NOTE: The cylinder normals face outward by default, but we want the lighting to be correct
    // from both sides, at least on the cylinder where we can see quite a lot on both sides.
    // Therefore, I'm creating two copies of the same cylinder, scaled slightly differently, with
    // the normals flipped on the inner one.
    scene.add_object(
        RenderObject::new(Cylinder::partial(1.5, 3., 300., uv_image_mat))
            .position(3.0, 1.5, 1.)
            .rotate(Rotor3::from_euler_angles(
                90f32.to_radians(),
                30f32.to_radians(),
                -35f32.to_radians(),
            )),
    );
    scene.add_object(
        RenderObject::new(Cylinder::partial(1.49, 3., 300., uv_image_mat))
            .position(3.0, 1.5, 1.)
            .rotate(Rotor3::from_euler_angles(
                90f32.to_radians(),
                30f32.to_radians(),
                -35f32.to_radians(),
            ))
            .flip_normals(),
    );

    scene.add_object(
        RenderObject::new(Disk::partial(1.5, 300., 0.8, uv_image_mat))
            .position(3.0, 1.5, 1.)
            .rotate(Rotor3::from_euler_angles(
                90f32.to_radians(),
                30f32.to_radians(),
                -35f32.to_radians(),
            )),
    );

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
    let scene = objects_scene();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(6., 4., -7.))
        .look_at(Vec3::new(0., 1.5, 0.))
        .field_of_view(60.);
    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(128)
        .camera(camera);

    let render = renderer.render(&scene);

    let window = RenderWindow::new(
        "conics",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
