use super::{ray::Ray, rtweekend::*, vec3::Vec3};
#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}
impl AABB {
    pub fn new() -> AABB {
        AABB {
            min: Vec3::new(),
            max: Vec3::new(),
        }
    }
    pub fn from(a: Vec3, b: Vec3) -> AABB {
        AABB { min: a, max: b }
    }
    pub fn hit(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (self.min[i] - r.origin()[i]) * inv_d;
            let mut t1 = (self.max[i] - r.origin()[i]) * inv_d;
            if inv_d < 0.0 {
                let temp = t1;
                t1 = t0;
                t0 = temp;
            }
            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::from(
            ffmin(box0.min.x(), box1.min.x()),
            ffmin(box0.min.y(), box1.min.y()),
            ffmin(box0.min.z(), box1.min.z()),
        );
        let big = Vec3::from(
            ffmax(box0.max.x(), box1.max.x()),
            ffmax(box0.max.y(), box1.max.y()),
            ffmax(box0.max.z(), box1.max.z()),
        );
        return AABB {
            min: small,
            max: big,
        };
    }
}
