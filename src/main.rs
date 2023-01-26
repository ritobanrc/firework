use firework::{camera::CameraSettings, RenderWindow, Renderer, Scene};
use std::path::PathBuf;
use structopt::StructOpt;
use ultraviolet::Vec3;

#[derive(StructOpt, Debug)]
#[structopt(name = "firework")]
struct Opt {
    #[structopt(long)]
    scene_file: PathBuf,

    #[structopt(short, long)]
    name: Option<String>,

    #[structopt(short, long)]
    samples: usize,
}

fn main() -> serde_yaml::Result<()> {
    let opt = Opt::from_args();

    let file = std::fs::File::open(opt.scene_file).unwrap();
    let scene: Scene = serde_yaml::from_reader(file)?;

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(0., 30., 50.))
        .look_at(Vec3::new(0., 0., 0.))
        .field_of_view(40.);

    let renderer = Renderer::default()
        .width(960)
        .height(540)
        .samples(opt.samples)
        .use_bvh(true)
        .camera(camera);

    let start = std::time::Instant::now();

    let render = renderer.render(scene);

    let end = std::time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let name: &str = opt
        .name
        .as_ref()
        .map(|x| x.as_str())
        .unwrap_or("Firework Render");

    let window = RenderWindow::new(name, Default::default(), renderer.width, renderer.height);

    window.display(&render);

    Ok(())
}
