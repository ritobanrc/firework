use firework::camera::CameraSettings;
use firework::environment::Environment;
use firework::material::{DielectricMat, LambertianMat, MetalMat};
use firework::objects::{Sphere, XZRect};
use firework::render::Renderer;
use firework::scene::{RenderObject, Scene};
use firework::window::RenderWindow;
use std::convert::TryFrom;
use std::f32::consts::PI;
use std::path::{Path, PathBuf};
use std::time;
use ultraviolet::{Vec2, Vec3};

pub fn sphere_uv(point: &Vec3) -> Vec2 {
    let phi = point.z.atan2(point.x);
    let theta = point.y.asin();
    let u = 1. - (phi + PI) / (2. * PI);
    let v = (theta + PI / 2.) / PI;
    Vec2::new(u, v)
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(try_from = "PathBuf")]
#[serde(into = "PathBuf")]
struct HdrEnvironment {
    pixels: Vec<image::Rgb<f32>>,
    path: PathBuf,
    width: f32,
    height: f32,
}

impl HdrEnvironment {
    pub fn from_path(path: impl AsRef<Path>) -> Result<HdrEnvironment, Box<dyn std::error::Error>> {
        let path_buf = path.as_ref().to_owned();
        HdrEnvironment::try_from(path_buf)
    }
}

impl Into<PathBuf> for HdrEnvironment {
    fn into(self) -> PathBuf {
        self.path
    }
}

impl TryFrom<PathBuf> for HdrEnvironment {
    type Error = Box<dyn std::error::Error>;
    fn try_from(path: PathBuf) -> Result<HdrEnvironment, Self::Error> {
        use image::hdr::HdrDecoder;
        use std::fs::File;
        use std::io::BufReader;

        let hdri = File::open("urban_street_04_4k.hdr")?;
        let hdri = BufReader::new(hdri);
        let hdri = HdrDecoder::new(hdri)?;

        let hdri_width = hdri.metadata().width as f32;
        let hdri_height = hdri.metadata().height as f32;

        let pixels = hdri.read_image_hdr()?;

        Ok(HdrEnvironment {
            pixels,
            path,
            width: hdri_width,
            height: hdri_height,
        })
    }
}

#[typetag::serde]
impl Environment for HdrEnvironment {
    fn sample(&self, dir: Vec3) -> Vec3 {
        let uv = sphere_uv(&dir);

        let x = (uv.x * self.width) as usize;
        let y = ((1. - uv.y) * self.height) as usize;

        let idx = (y as f32 * self.width) as usize + x;

        self.pixels[idx].0.into()
    }
}

// NOTE: Currently, there isn't importance sampling, so even with insane numbers of samples, it's impossible to get accurate HDR lighting that isn't noisy.
pub fn hdri_test() -> Scene {
    let mut scene = Scene::new();

    scene.set_environment(HdrEnvironment::from_path("urban_street_04_4k.hdr").unwrap());

    let glass = scene.add_material(DielectricMat::new(1.5));
    let diffuse = scene.add_material(LambertianMat::with_color(Vec3::new(0.8, 0.8, 0.8)));
    let metal = scene.add_material(MetalMat::new(Vec3::new(0.7, 0.7, 0.7), 0.0));

    scene.add_object(RenderObject::new(Sphere::new(1.0, glass)).position(0., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(1.0, diffuse)).position(-4., 1., 0.));
    scene.add_object(RenderObject::new(Sphere::new(1.0, metal)).position(4., 1., 0.));

    scene.add_object(RenderObject::new(XZRect::new(
        -100., 100., -100., 100., 0., diffuse,
    )));

    scene
}

fn main() {
    let scene = hdri_test();

    let start = time::Instant::now();

    let camera = CameraSettings::default()
        .cam_pos(Vec3::new(0., 2., -10.))
        .look_at(Vec3::zero());

    let renderer = Renderer::default()
        .width(200)
        .height(100)
        .samples(50000)
        .camera(camera);

    let render = renderer.render(scene);

    let end = time::Instant::now();
    println!("Finished Rendering in {} s", (end - start).as_secs());

    let window = RenderWindow::new(
        "HDRI Test",
        Default::default(),
        renderer.width,
        renderer.height,
    );

    window.display(&render);
}
