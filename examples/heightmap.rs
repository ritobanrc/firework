use firework::camera::CameraSettings;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::{Triangle, TriangleMesh};
use firework::objects::{YZRect, XZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::window::RenderWindow;
use ultraviolet::{Rotor3, Vec3};

//fn teapot_scene() -> Scene<'static> {
//}


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


fn main() {
    let mut verts = Vec::new();
    let mut indicies = Vec::new();

    let size = 20;

    for y in 0..size { 
        for x in 0..size {
            let height = (x as f32).cos() + (y as f32).sin();
            //let height = 2.;
            verts.push(Vec3::new(x as f32, height, y as f32));
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


    let mesh = TriangleMesh::new(verts, indicies, None, None, 1).unwrap();

    let mut scene = Scene::new();

    let light = scene.add_material(EmissiveMat::with_color(Vec3::broadcast(8.)));
    scene.add_object(
        RenderObject::new(YZRect::new(0., 20., 0., 10., -3., light))
            .rotate(Rotor3::from_rotation_xz(-30.))
            .position(0., 0., -10.),
    );


    let _green = scene.add_material(LambertianMat::with_color(Vec3::new(0., 0.5, 0.3)));

    for tri in 0..size * size {
        scene.add_object(RenderObject::new(Triangle::new(&mesh, tri)));
    }


    scene.set_environment(sky_color);

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(10., 6., -3.))
        .look_at(Vec3::new(10., 0., 10.))
        .field_of_view(60.);
    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(128)
        .use_bvh(true)
        .camera(camera);

    let render = renderer.render(&scene);

    let window = RenderWindow::new(
        "heightmap",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
