use ultraviolet::Vec3;

pub trait Texture {
    fn sample(&self, u: f32, v: f32, point: &Vec3) -> Vec3;
}

pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn sample(&self, _u: f32, _v: f32, _point: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Box<dyn Texture + Sync>,
    pub even: Box<dyn Texture + Sync>,
    pub scale: f32
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture + Sync>, even: Box<dyn Texture + Sync>, scale: f32) -> Self {
        CheckerTexture { odd, even, scale}
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, u: f32, v: f32, point: &Vec3) -> Vec3 {
        let iter: [f32; 3] = (*point).into();
        if iter.iter().map(|x| (self.scale*x).sin()).product::<f32>().is_sign_positive() {
            self.even.sample(u, v, point)
        } else {
            self.odd.sample(u, v, point)
        }
    }
}
