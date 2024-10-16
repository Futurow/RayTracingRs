use std::rc::Rc;

use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}
impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}
impl Sphere {
    pub fn from(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            let mut rest = None;
            if temp < t_max && temp > t_min {
                rest = Some(temp);
            } else {
                temp = (-half_b + root) / a;
                if temp < t_max && temp > t_min {
                    rest = Some(temp);
                }
            }
            match rest {
                Some(t) => {
                    hit_record.t = t;
                    hit_record.p = r.at(t);
                    let outward_normal = (hit_record.p - self.center) / self.radius;
                    hit_record.set_face_normal(r, outward_normal);
                    hit_record.material = Some(Rc::clone(&self.material));
                    return true;
                }
                _ => {
                    return false;
                }
            }

            // let temp = (-half_b - root) / a;
            // if temp < t_max && temp > t_min {
            //     hit_record.t = temp;
            //     hit_record.p = r.at(temp);
            //     let outward_normal = (hit_record.p - self.center) / self.radius;
            //     hit_record.set_face_normal(r, outward_normal);
            //     return true;
            // }
            // let temp = (-half_b + root) / a;
            // if temp < t_max && temp > t_min {
            //     hit_record.t = temp;
            //     hit_record.p = r.at(temp);
            //     let outward_normal = (hit_record.p - self.center) / self.radius;
            //     hit_record.set_face_normal(r, outward_normal);
            //     return true;
            // } else {
            //     return false;
            // }
        } else {
            return false;
        }
    }
}
