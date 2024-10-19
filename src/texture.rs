use super::perlin::Pelin;
use super::rtweekend::*;
use super::vec3::Vec3;
use image::RgbImage;
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
pub struct ConstantTexture {
    color: Vec3,
}
impl ConstantTexture {
    pub fn new() -> ConstantTexture {
        ConstantTexture { color: Vec3::new() }
    }
    pub fn from(c: Vec3) -> ConstantTexture {
        ConstantTexture { color: c }
    }
}
impl Texture for ConstantTexture {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        return self.color;
    }
}
#[derive(Clone)]
pub struct CheckerTexture {
    odd: Option<Rc<dyn Texture>>,
    even: Option<Rc<dyn Texture>>,
}

impl CheckerTexture {
    pub fn new() -> CheckerTexture {
        CheckerTexture {
            odd: None,
            even: None,
        }
    }
    pub fn from(t0: Rc<dyn Texture>, t1: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture {
            odd: Some(t1),
            even: Some(t0),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.as_ref().unwrap().value(u, v, p)
        } else {
            self.even.as_ref().unwrap().value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Pelin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        NoiseTexture {
            noise: Pelin::new(),
            scale: 5.0,
        }
    }
    pub fn from(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Pelin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        // let temp_p = Vec3::from(self.scale * p.x(), self.scale * p.y(), self.scale * p.z());
        // return (1.0 + self.noise.noise(&temp_p)) * 0.5 * Vec3::from(1.0, 1.0, 1.0);
        Vec3::from(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(&p, None)).sin())
    }
}

pub struct ImageTexture {
    img: Option<RgbImage>,
}
impl ImageTexture {
    pub fn from(path: &str) -> ImageTexture {
        let img: RgbImage = match image::open(path) {
            Ok(val) => val.to_rgb8(),
            Err(_) => {
                eprint!("读取材质图片错误");
                panic!();
            }
        };
        ImageTexture { img: Some(img) }
    }
    fn clamp(x: f64, low: f64, high: f64) -> f64 {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high - 0.001
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Vec3) -> Vec3 {
        let data = match &self.img {
            Some(val) => val,
            None => return Vec3::from(1.0, 0.0, 0.0),
        };
        let nx = data.width() as f64;
        let ny = data.height() as f64;

        let u = ImageTexture::clamp(u, 0.0, 1.0);
        let v = 1.0 - ImageTexture::clamp(v, 0.0, 1.0);
        let i = (u * nx) as i32;
        let j = (v * ny) as i32;
        let r = data.get_pixel(i as u32, j as u32)[0] as f64 / 255.0;
        let g = data.get_pixel(i as u32, j as u32)[1] as f64 / 255.0;
        let b = data.get_pixel(i as u32, j as u32)[2] as f64 / 255.0;
        Vec3::from(r, g, b)
    }
}
