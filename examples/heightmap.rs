use firework::camera::CameraSettings;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::TriangleMesh;
use firework::objects::YZRect;
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::window::RenderWindow;
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

fn heighmap_scene() -> Scene<'static> {
    let mut scene = Scene::new();

    scene.set_environment(sky_color);
    let light = scene.add_material(EmissiveMat::with_color(Vec3::broadcast(8.)));
    scene.add_object(
        RenderObject::new(YZRect::new(0., 20., 0., 10., -3., light))
            .rotate(Rotor3::from_rotation_xz(-30.))
            .position(0., 0., -10.),
    );

    let green = scene.add_material(LambertianMat::with_color(Vec3::new(0., 0.5, 0.3)));

    let mut verts = Vec::new();
    let mut indicies = Vec::new();

    let size = 20;

    for y in 0..size {
        for x in 0..size {
            let height = (x as f32).cos() + (y as f32).sin();
            //let height = 2.;
            verts.push(Vec3::new(x as f32, 0.2 * height, y as f32));
            if x != 0 && y != 0 {
                // Triangles:
                // verts[x - 1][y - 1], verts[x][y], verts[x][y - 1]
                // verts[x - 1][y - 1], verts[x - 1][y], verts[x][y]
                indicies.push((y - 1) * size + x - 1);
                indicies.push(y * size + x);
                indicies.push((y - 1) * size + x);

                indicies.push((y - 1) * size + x - 1);
                indicies.push(y * size + x - 1);
                indicies.push(y * size + x);
            }
        }
    }

    scene.add_mesh(TriangleMesh::new(verts, indicies, None, None, green).unwrap());

    scene
}

fn main() {
    let scene = heighmap_scene();
    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(10., 6., -3.))
        //.look_at(Vec3::new(0., 0., 0.))
        .look_at(Vec3::new(10., 0., 10.))
        .field_of_view(40.);
    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(32)
        .use_bvh(true)
        .camera(camera);

    let start = std::time::Instant::now();

    let render = renderer.render(&scene);

    let end = std::time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let window = RenderWindow::new(
        "heightmap",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
