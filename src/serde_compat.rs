use crate::render::Hitable;
use serde::{Deserialize, Serialize};
use ultraviolet::{Bivec3, Rotor3};

// These still need to exist while we wait on #74.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Rotor3")]
pub(crate) struct Rotor3Def {
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
pub trait SerializableShape: AsHitable {}

pub trait AsHitable: Hitable {
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

#[typetag::serde]
impl SerializableShape for crate::objects::XYRect {}

#[typetag::serde]
impl SerializableShape for crate::objects::YZRect {}

#[typetag::serde]
impl SerializableShape for crate::objects::XZRect {}

#[typetag::serde]
impl SerializableShape for crate::objects::ConstantMedium {}
