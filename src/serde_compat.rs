use crate::render::Hitable;
use crate::scene::RenderObject;
use serde::{Deserialize, Serialize};
use ultraviolet::{Bivec3, Rotor3, Vec2, Vec3};

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedRenderObject {
    obj: Box<dyn SerializableShape>,
    #[serde(with = "Vec3Def")]
    position: Vec3,
    #[serde(with = "Rotor3Def")]
    rotation: Rotor3,
    flip_normals: bool,
}

impl From<SerializedRenderObject> for RenderObject {
    fn from(s: SerializedRenderObject) -> RenderObject {
        let mut obj = RenderObject {
            obj: s.obj.to_hitable(),
            position: s.position,
            rotation_mat: s.rotation.into_matrix(),
            inv_rotation_mat: s.rotation.reversed().into_matrix(),
            flip_normals: s.flip_normals,
            aabb: None,
        };
        obj.update_bounding_box();
        obj
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vec3")]
pub(crate) struct Vec3Def {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vec2")]
pub(crate) struct Vec2Def {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Rotor3")]
struct Rotor3Def {
    s: f32,
    #[serde(with = "Bivec3Def")]
    bv: Bivec3,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Bivec3")]
struct Bivec3Def {
    xy: f32,
    xz: f32,
    yz: f32,
}

#[typetag::serde(tag = "object_type")]
trait SerializableShape: AsHitable {}

trait AsHitable: Hitable {
    fn as_hitable(&self) -> &dyn Hitable;
    fn to_hitable(self: Box<Self>) -> Box<dyn Hitable>
    where
        Self: 'static;
}

impl<T: SerializableShape> AsHitable for T {
    fn as_hitable(&self) -> &dyn Hitable {
        self
    }

    fn to_hitable(self: Box<Self>) -> Box<dyn Hitable>
    where
        Self: 'static,
    {
        self
    }
}

#[typetag::serde]
impl SerializableShape for crate::objects::Cone {}

#[typetag::serde]
impl SerializableShape for crate::objects::Sphere {}

#[typetag::serde]
impl SerializableShape for crate::objects::Disk {}

#[typetag::serde]
impl SerializableShape for crate::objects::Cylinder {}

#[typetag::serde]
impl SerializableShape for crate::objects::Rect3d {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialize_one() {
        use crate::objects::Cylinder;

        let cylinder = SerializedRenderObject {
            obj: Box::new(Cylinder::partial(1.49, 3., 300., 0)),
            position: Vec3::new(3.0, 1.5, 1.),
            rotation: Rotor3::from_euler_angles(
                90f32.to_radians(),
                30f32.to_radians(),
                -35f32.to_radians(),
            ),
            flip_normals: true,
        };

        println!("{}", serde_yaml::to_string(&cylinder).unwrap());
    }

    #[test]
    fn serialize_multiple() {
        use crate::material::LambertianMat;
        use crate::objects::{Cone, Cylinder};
        use crate::scene::{RenderObject, Scene};

        let cylinder = SerializedRenderObject {
            obj: Box::new(Cylinder::partial(1.49, 3., 300., 0)),
            position: Vec3::new(3.0, 1.5, 1.),
            rotation: Rotor3::from_euler_angles(
                90f32.to_radians(),
                30f32.to_radians(),
                -35f32.to_radians(),
            ),
            flip_normals: true,
        };

        let cone = SerializedRenderObject {
            obj: Box::new(Cone::new(2., 3., 0)),
            position: Vec3::new(-1., 0., -4.),
            rotation: Rotor3::identity(),
            flip_normals: false,
        };

        let objects = vec![cylinder, cone];

        let yaml = serde_yaml::to_string(&objects).unwrap();

        println!("{}", yaml);

        use crate::camera::CameraSettings;
        use crate::environment::SkyEnv;
        use crate::render::Renderer;
        use crate::window::RenderWindow;

        let mut deserialized: Vec<RenderObject> = serde_yaml::from_str(&yaml).unwrap();
        let mut scene = Scene::new();
        scene.add_object(deserialized.pop().unwrap());
        scene.add_object(deserialized.pop().unwrap());

        scene.add_material(LambertianMat::with_color(Vec3::new(0., 0.2, 0.4)));
        scene.set_environment(SkyEnv::default());

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
            "serialize_multiple_test",
            Default::default(),
            renderer.width,
            renderer.height,
        );

        window.display(&render);
    }
}
