use image::{GenericImageView, Pixel, Rgba};
use ultraviolet::{Vec2, Vec3};

pub trait Texture {
    fn sample(&self, uv: Vec2, point: &Vec3) -> Vec3;
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
    fn sample(&self, _uv: Vec2, _point: &Vec3) -> Vec3 {
        self.color
    }
}

pub struct CheckerTexture {
    pub odd: Box<dyn Texture + Sync>,
    pub even: Box<dyn Texture + Sync>,
    pub scale: f32,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture + Sync>, even: Box<dyn Texture + Sync>, scale: f32) -> Self {
        CheckerTexture { odd, even, scale }
    }

    pub fn with_colors(odd: Vec3, even: Vec3, scale: f32) -> Self {
        CheckerTexture::new(
            Box::new(ConstantTexture::new(odd)),
            Box::new(ConstantTexture::new(even)),
            scale,
        )
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: Vec2, point: &Vec3) -> Vec3 {
        // TODO: Actually use proper uv coordinates
        let iter: [f32; 3] = (*point).into();
        if iter
            .iter()
            .map(|x| (self.scale * x).sin())
            .product::<f32>()
            .is_sign_positive()
        {
            self.even.sample(uv, point)
        } else {
            self.odd.sample(uv, point)
        }
    }
}

pub struct PerlinNoiseTexture {
    scale: f32,
}

static P: [usize; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

impl PerlinNoiseTexture {
    pub fn new(scale: f32) -> PerlinNoiseTexture {
        PerlinNoiseTexture { scale }
    }

    fn noise(p: &Vec3) -> f32 {
        let x0 = p.x.floor() as usize & 255;
        let y0 = p.y.floor() as usize & 255;
        let z0 = p.z.floor() as usize & 255;

        let x = p.x - p.x.floor();
        let y = p.y - p.y.floor();
        let z = p.z - p.z.floor();

        let u = fade(x);
        let v = fade(y);
        let w = fade(z);

        let a = P[x0] + y0;
        let aa = P[a] + z0;
        let ab = P[a + 1] + z0;
        let b = P[x0 + 1] + y0;
        let ba = P[b] + z0;
        let bb = P[b + 1] + z0;

        lerp(
            w,
            lerp(
                v,
                lerp(u, grad(P[aa], x, y, z), grad(P[ba], x - 1.0, y, z)),
                lerp(
                    u,
                    grad(P[ab], x, y - 1.0, z),
                    grad(P[bb], x - 1.0, y - 1.0, z),
                ),
            ),
            lerp(
                v,
                lerp(
                    u,
                    grad(P[aa + 1], x, y, z - 1.0),
                    grad(P[ba + 1], x - 1.0, y, z - 1.0),
                ),
                lerp(
                    u,
                    grad(P[ab + 1], x, y - 1.0, z - 1.0),
                    grad(P[bb + 1], x - 1.0, y - 1.0, z - 1.0),
                ),
            ),
        )
    }
}

impl Texture for PerlinNoiseTexture {
    fn sample(&self, _uv: Vec2, point: &Vec3) -> Vec3 {
        let a = PerlinNoiseTexture::noise(&(*point * self.scale));
        Vec3::one() * (a + 0.5).min(1.)
        //Vec3::new(-0.5, 0., 0.)
    }
}

fn fade(t: f32) -> f32 {
    //t * t * t * (t * (t * 6. - 15.) + 10.)
    t * t * (3. - 2. * t)
}

fn grad(hash: usize, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    let u = if h & 1 == 0 { u } else { -u };
    let v = if h & 2 == 0 { v } else { -v };
    u + v
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

pub struct TurbulenceTexture {
    depth: usize,
    scale: f32,
}

impl TurbulenceTexture {
    pub fn new(depth: usize, scale: f32) -> Self {
        TurbulenceTexture { depth, scale }
    }

    fn turb(depth: usize, point: Vec3) -> f32 {
        let mut accum = 0.;
        let mut p = point;
        let mut weight = 1.;
        for _ in 0..depth {
            let a = PerlinNoiseTexture::noise(&p);
            accum += weight * a;
            weight *= 0.5;
            p *= 2.;
        }
        accum
    }
}

impl Texture for TurbulenceTexture {
    fn sample(&self, _uv: Vec2, point: &Vec3) -> Vec3 {
        Vec3::one() * TurbulenceTexture::turb(self.depth, *point * self.scale)
    }
}

pub struct MarbleTexture {
    depth: usize,
    scale: f32,
}

impl MarbleTexture {
    pub fn new(depth: usize, scale: f32) -> Self {
        MarbleTexture { depth, scale }
    }
}

impl Texture for MarbleTexture {
    fn sample(&self, _uv: Vec2, point: &Vec3) -> Vec3 {
        Vec3::one()
            * 0.5
            * (1.
                + f32::sin(
                    self.scale * point.z + 10. * TurbulenceTexture::turb(self.depth, *point),
                ))
    }
}

pub struct ImageTexture<T> {
    image: T,
}

impl<'a, T> ImageTexture<T>
where
    T: GenericImageView,
{
    pub fn new(image: T) -> ImageTexture<T> {
        ImageTexture { image }
    }
}

impl<T> Texture for ImageTexture<T>
where
    T: GenericImageView<Pixel = Rgba<u8>>,
{
    fn sample(&self, uv: Vec2, _point: &Vec3) -> Vec3 {
        let (w, h) = self.image.dimensions();
        let i = uv.x * self.image.dimensions().0 as f32;
        let j = (1. - uv.y) * self.image.dimensions().1 as f32;

        let i = (i as u32).clamp(0, w - 1);
        let j = (j as u32).clamp(0, h - 1);

        let c: Rgba<u8> = self.image.get_pixel(i as u32, j as u32).to_rgba();

        //println!("{:?} ({}, {}) : {:?}", uv, i, j, c);

        Vec3::new(c[0].into(), c[1].into(), c[2].into()) / 255.
    }
}
