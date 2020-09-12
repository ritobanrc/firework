use firework::camera::CameraSettings;
use firework::environment::SkyEnv;
use firework::material::LambertianMat;
use firework::objects::TriangleMesh;
use firework::render::Renderer;
use firework::scene::{MaterialIdx, Scene};
use firework::window::RenderWindow;
use std::convert::AsRef;
use std::fmt;
use std::path::Path;
use ultraviolet::Vec3;

pub fn add_obj<P>(scene: &mut Scene, file_name: P, material: MaterialIdx)
where
    P: AsRef<Path> + fmt::Debug,
{
    let obj = tobj::load_obj(file_name);
    assert!(obj.is_ok());
    let (models, materials) = obj.unwrap();

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!("Size of model[{}].indices: {}", i, mesh.indices.len());
        for f in 0..mesh.indices.len() / 3 {
            println!(
                "    idx[{}] = {}, {}, {}.",
                f,
                mesh.indices[3 * f],
                mesh.indices[3 * f + 1],
                mesh.indices[3 * f + 2]
            );
        }

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            println!(
                "    v[{}] = ({}, {}, {})",
                v,
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2]
            );
        }

        let triangle_mesh = TriangleMesh::new(
            mesh.positions
                .chunks(3)
                .map(|arr| Vec3::new(arr[0], arr[1], arr[2]))
                .collect(),
            mesh.indices.iter().map(|x| *x as usize).collect(),
            None,
            None,
            material,
        )
        .unwrap();
        scene.add_mesh(triangle_mesh);
    }
}

fn teapot_scene() -> Scene {
    let mut scene = Scene::new();

    let diffuse = scene.add_material(LambertianMat::with_color(Vec3::broadcast(0.8)));
    add_obj(&mut scene, "teapot.obj", diffuse);
    scene.set_environment(SkyEnv::default());

    scene
}

fn main() {
    let scene = teapot_scene();
    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(0., 30., 50.))
        //.look_at(Vec3::new(0., 0., 0.))
        .look_at(Vec3::new(0., 0., 0.))
        .field_of_view(40.);

    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(32)
        .use_bvh(true)
        .camera(camera);

    let start = std::time::Instant::now();

    let render = renderer.render(scene);

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
