use super::vec3::Vec3;
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
    tm: f64,
}
impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Vec3::new(),
            dir: Vec3::new(),
            tm: 0.0,
        }
    }
    pub fn from(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}
