use super::vec3::Vec3;
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}
impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Vec3::new(),
            dir: Vec3::new(),
        }
    }
    pub fn from(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}
