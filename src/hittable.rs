use super::rtweekend::*;
use std::cmp::Ordering;

use rand::Rng;

use super::aabb::AABB;

use super::material::Material;
use super::ray::Ray;
use super::vec3::Vec3;
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool;
}
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
    pub u: f64,
    pub v: f64,
}
impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            material: None,
            u: 0.0,
            v: 0.0,
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
        } else {
            return false;
        }
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        let v_radius: Vec3 = Vec3::from(self.radius, self.radius, self.radius);
        *output_box = AABB::from(self.center - v_radius, self.center + v_radius);
        true
    }
}
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Rc<dyn Material>,
}
impl MovingSphere {
    pub fn from(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center(r.time());
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
                    let outward_normal = (hit_record.p - self.center(r.time())) / self.radius;
                    hit_record.set_face_normal(r, outward_normal);
                    hit_record.material = Some(Rc::clone(&self.material));
                    return true;
                }
                _ => {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        let v_radius: Vec3 = Vec3::from(self.radius, self.radius, self.radius);
        let box0 = AABB::from(self.center(t0) - v_radius, self.center(t0) + v_radius);
        let box1 = AABB::from(self.center(t1) - v_radius, self.center(t1) + v_radius);
        *output_box = AABB::surrounding_box(&box0, &box1);
        true
    }
}

pub struct BvhNode {
    pub left: Option<Rc<dyn Hittable>>,
    pub right: Option<Rc<dyn Hittable>>,
    aabb_box: AABB,
}
impl BvhNode {
    pub fn new() -> BvhNode {
        BvhNode {
            left: None,
            right: None,
            aabb_box: AABB::new(),
        }
    }
    pub fn from(
        objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut res = BvhNode::new();
        let axis: i32 = rand::thread_rng().gen_range(0..=2);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => panic!(),
        };
        let object_span: usize = end - start;
        if object_span == 1 {
            res.left = Some(objects[start].clone());
            res.right = Some(objects[start].clone());
        } else if object_span == 2 {
            if Ordering::Less == comparator(&objects[start], &objects[start + 1]) {
                res.left = Some(objects[start].clone());
                res.right = Some(objects[start + 1].clone());
            } else {
                res.left = Some(objects[start + 1].clone());
                res.right = Some(objects[start].clone());
            }
        } else {
            objects.sort_by(|a, b| comparator(a, b));
            let mid = start + object_span / 2;
            res.left = Some(Rc::new(BvhNode::from(objects, start, mid, time0, time1)));
            res.right = Some(Rc::new(BvhNode::from(objects, mid, end, time0, time1)));
        }
        let mut box_left = AABB::new();
        let mut box_right = AABB::new();
        if let Some(ref l) = res.left {
            if let Some(ref r) = res.right {
                l.bounding_box(time0, time1, &mut box_left);
                r.bounding_box(time0, time1, &mut box_right);
            } else {
                eprintln!("No bounding box in bvh_node constructor.");
                panic!()
            }
        } else {
            eprintln!("No bounding box in bvh_node constructor.");
            panic!()
        }

        res.aabb_box = AABB::surrounding_box(&box_left, &box_right);

        return res;
    }
    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
        let mut box_a: AABB = AABB::new();
        let mut box_b: AABB = AABB::new();
        if (!a.bounding_box(0.0, 0.0, &mut box_a)) || (!b.bounding_box(0.0, 0.0, &mut box_b)) {
            eprintln!("No bounding box in bvh_node constructor.");
        };
        match box_a.min.e[axis] < box_b.min.e[axis] {
            true => Ordering::Less,
            false => Ordering::Greater,
        }
    }
    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 2)
    }
}
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        if !self.aabb_box.hit(r, t_min, t_max) {
            return false;
        }
        let mut hit_left = false;
        let mut hit_right = false;
        if let Some(ref x) = self.left {
            hit_left = x.hit(r, t_min, t_max, hit_record);
        };
        if let Some(ref x) = self.right {
            hit_right = x.hit(
                r,
                t_min,
                if hit_left { hit_record.t } else { t_max },
                hit_record,
            )
        };
        return hit_left || hit_right;
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.aabb_box;
        true
    }
}
