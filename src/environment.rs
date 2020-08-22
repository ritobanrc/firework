use ultraviolet::Vec3;

/// A trait for the world environment
pub trait Environment {
    fn sample(&self, dir: Vec3) -> Vec3;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ColorEnv {
    color: Vec3,
}

impl ColorEnv {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }
}

impl Environment for ColorEnv {
    fn sample(&self, _dir: Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct SkyEnv {
    zenith_color: Vec3,
    horizon_color: Vec3,
}

impl SkyEnv {
    pub fn new(zenith_color: Vec3, horizon_color: Vec3) -> Self {
        SkyEnv {
            zenith_color,
            horizon_color,
        }
    }
}

impl Default for SkyEnv {
    fn default() -> Self {
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

        SkyEnv::new(SKY_BLUE, SKY_WHITE)
    }
}

impl Environment for SkyEnv {
    fn sample(&self, dir: Vec3) -> Vec3 {
        // Take the y (from -1 to +1) and map it to 0..1
        let t = 0.5 * (dir.y + 1.0);
        (1. - t) * self.horizon_color + t * self.zenith_color
    }
}
