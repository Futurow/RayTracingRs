use std::rc::Rc;

use super::aabb::AABB;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}
impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box: AABB = AABB::new();
        let first_box: bool = true;
        for object in self.objects.iter() {
            if !object.bounding_box(t0, t1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                AABB::surrounding_box(&output_box, &temp_box)
            };
        }
        true
    }
}
