#![feature(array_chunks)]

use firework::camera::CameraSettings;
use firework::environment::SkyEnv;
use firework::material::{EmissiveMat, LambertianMat};
use firework::objects::TriangleMesh;
use firework::objects::{XZRect, YZRect};
use firework::render::Renderer;
use firework::scene::{MaterialIdx, RenderObject, Scene};
use firework::texture::ConstantTexture;
use firework::window::RenderWindow;
use std::convert::AsRef;
use std::fmt;
use std::path::Path;
use ultraviolet::{Rotor3, Vec3};

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

        // Normals and texture coordinates are also loaded, but not printed in this exampl
        // e
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);

        let normals = if !mesh.normals.is_empty() {
            Some(
                mesh.normals
                    .array_chunks::<3>()
                    .map(|x| Vec3::from(x))
                    .collect(),
            )
        } else {
            None
        };

        let triangle_mesh = TriangleMesh::new(
            mesh.positions
                .chunks(3)
                .map(|arr| Vec3::new(arr[0], arr[1], arr[2]))
                .collect(),
            mesh.indices.iter().map(|x| *x as usize).collect(),
            normals,
            None,
            material,
        )
        .unwrap();

        scene.add_object(RenderObject::new(triangle_mesh).rotate(Rotor3::from_rotation_xz(90.)));
    }
}

fn teapot_scene() -> Scene {
    let mut scene = Scene::new();

    let diffuse = scene.add_material(LambertianMat::new(ConstantTexture::new(Vec3::new(
        0.2, 0.8, 0.3,
    ))));
    add_obj(&mut scene, "teapot.obj", diffuse);

    scene.set_environment(SkyEnv::default());

    let grey = scene.add_material(LambertianMat::with_color(Vec3::broadcast(0.5)));
    scene.add_object(RenderObject::new(XZRect::new(
        -20., 20., -20., 20., 0., grey,
    )));

    let light = scene.add_material(EmissiveMat::with_color(Vec3::broadcast(20.)));
    scene.add_object(
        RenderObject::new(YZRect::new(0., 4., 0., 4., -0.6, light))
            .rotate(Rotor3::from_rotation_xz(-30.))
            .position(0., 4., 10.),
    );

    scene
}

fn main() {
    let scene = teapot_scene();

    use std::io::Write;
    let mut file = std::fs::File::create("scenes/teapot.yml").unwrap();
    file.write_all(serde_yaml::to_string(&scene).unwrap().as_bytes())
        .unwrap();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(1., 4., 8.))
        .look_at(Vec3::new(0., 1., 0.))
        .field_of_view(40.);

    let renderer = Renderer::default()
        .width(1920)
        .height(1080)
        .samples(512)
        .use_bvh(true)
        .camera(camera);

    let start = std::time::Instant::now();

    let render = renderer.render(scene);

    let end = std::time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let window = RenderWindow::new(
        "teapot",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
