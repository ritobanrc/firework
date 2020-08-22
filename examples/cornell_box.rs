use firework::camera::CameraSettings;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::{Rect3d, XYRect, XZRect, YZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::window::RenderWindow;
use std::time;
use ultraviolet::{Rotor3, Vec3};

pub fn cornell_box() -> Scene {
    //let cam_pos = Vec3::new(278., 278., -800.);
    //let look_at = Vec3::new(278., 278., 0.);
    //let camera = Camera::new(cam_pos, look_at, Vec3::unit_y(), 40.0, 0.0, 10.);

    let mut world = Scene::new();

    let red = world.add_material(LambertianMat::with_color(Vec3::new(0.65, 0.05, 0.05)));
    let white = world.add_material(LambertianMat::with_color(Vec3::new(0.73, 0.73, 0.73)));
    let green = world.add_material(LambertianMat::with_color(Vec3::new(0.12, 0.45, 0.15)));

    let light = world.add_material(EmissiveMat::with_color(Vec3::new(15., 15., 15.)));

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

fn main() {
    let scene = cornell_box();
    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(278., 278., -800.))
        .look_at(Vec3::new(278., 278., 0.))
        .field_of_view(40.);
    let renderer = Renderer::default()
        .width(300)
        .height(300)
        .samples(300)
        .camera(camera);

    let render = renderer.render(&scene);

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let window = RenderWindow::new(
        "Cornell Box",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
