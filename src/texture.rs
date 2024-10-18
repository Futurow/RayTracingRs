use super::rtweekend::*;
use super::vec3::Vec3;
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
