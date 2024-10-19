use crate::hittable_list::HittableList;
use crate::material::Isotropic;
use crate::texture::Texture;

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
    fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        let phi = (-p.z()).atan2(p.x()) + PI;
        let theta = (-p.y()).acos();

        *u = phi / (2.0 * PI);
        *v = theta / PI;
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
                    Self::get_sphere_uv(
                        &((hit_record.p - self.center) / self.radius),
                        &mut hit_record.u,
                        &mut hit_record.v,
                    );
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
pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Rc<dyn Material>,
}
pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<dyn Material>,
}
pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<dyn Material>,
}
impl XyRect {
    pub fn from(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Rc<dyn Material>) -> XyRect {
        XyRect {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}
impl XzRect {
    pub fn from(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> XzRect {
        XzRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}
impl YzRect {
    pub fn from(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> YzRect {
        YzRect {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}
impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (y - self.y0) / (self.y1 - self.y0);
        hit_record.t = t;
        let outward_normal = Vec3::from(0.0, 0.0, 1.0);
        hit_record.set_face_normal(r, outward_normal);
        hit_record.material = Some(self.material.clone());
        hit_record.p = r.at(t);
        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from(
            Vec3::from(self.x0, self.y0, self.k - 0.0001),
            Vec3::from(self.x1, self.y1, self.k + 0.0001),
        );
        return true;
    }
}
impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        let outward_normal = Vec3::from(0.0, 1.0, 0.0);
        hit_record.set_face_normal(r, outward_normal);
        hit_record.material = Some(self.material.clone());
        hit_record.p = r.at(t);
        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from(
            Vec3::from(self.x0, self.k - 0.0001, self.z0),
            Vec3::from(self.x1, self.k + 0.0001, self.z1),
        );
        return true;
    }
}
impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        hit_record.u = (y - self.y0) / (self.y1 - self.y0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;
        let outward_normal = Vec3::from(1.0, 0.0, 0.0);
        hit_record.set_face_normal(r, outward_normal);
        hit_record.material = Some(self.material.clone());
        hit_record.p = r.at(t);
        true
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from(
            Vec3::from(self.k - 0.0001, self.y0, self.z0),
            Vec3::from(self.k + 0.0001, self.y1, self.z1),
        );
        return true;
    }
}
pub struct RectBox {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}
impl RectBox {
    pub fn from(p0: Vec3, p1: Vec3, material: Rc<dyn Material>) -> RectBox {
        let mut sides: HittableList = HittableList::default();
        sides.add(Rc::new(XyRect::from(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        )));
        sides.add(Rc::new(XyRect::from(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        )));
        sides.add(Rc::new(XzRect::from(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        )));
        sides.add(Rc::new(XzRect::from(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        )));
        sides.add(Rc::new(YzRect::from(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material.clone(),
        )));
        sides.add(Rc::new(YzRect::from(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material,
        )));
        RectBox {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for RectBox {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        self.sides.hit(r, t_min, t_max, hit_record)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::from(self.box_min, self.box_max);
        true
    }
}
pub struct Translate {
    hittable: Rc<dyn Hittable>,
    offset: Vec3,
}
impl Translate {
    pub fn from(p: Rc<dyn Hittable>, displacement: Vec3) -> Translate {
        Translate {
            hittable: p,
            offset: displacement,
        }
    }
}
impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let moved_r = Ray::from(r.origin() - self.offset, r.direction(), r.time());
        if !self.hittable.hit(&moved_r, t_min, t_max, hit_record) {
            return false;
        }
        hit_record.p += self.offset;
        hit_record.set_face_normal(&moved_r, hit_record.normal);
        return true;
    }
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        if !self.hittable.bounding_box(t0, t1, output_box) {
            return false;
        }
        *output_box = AABB::from(output_box.min + self.offset, output_box.max + self.offset);

        return true;
    }
}
pub struct RotateY {
    hittable: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}
impl RotateY {
    pub fn from(p: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let mut bbox = AABB::new();
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);

        let mut min = Vec3::from(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::from(-INFINITY, -INFINITY, -INFINITY);
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x() + (1 - i) as f64 * bbox.min.x();
                    let y = j as f64 * bbox.max.y() + (1 - j) as f64 * bbox.min.y();
                    let z = k as f64 * bbox.max.z() + (1 - k) as f64 * bbox.min.z();
                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;
                    let tester = Vec3::from(newx, y, newz);
                    for c in 0..3 {
                        min[c] = ffmin(min[c], tester[c]);
                        max[c] = ffmax(max[c], tester[c]);
                    }
                }
            }
        }
        RotateY {
            hittable: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox: AABB::from(min, max),
        }
    }
}
impl Hittable for RotateY {
    fn bounding_box(&self, _t0: f64, _t1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        return self.hasbox;
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];
        let rotated_r = Ray::from(origin, direction, r.time());
        if !self.hittable.hit(&rotated_r, t_min, t_max, hit_record) {
            return false;
        }
        let mut p = hit_record.p;
        let mut normal = hit_record.normal;
        p[0] = self.cos_theta * hit_record.p[0] + self.sin_theta * hit_record.p[2];
        p[2] = -self.sin_theta * hit_record.p[0] + self.cos_theta * hit_record.p[2];

        normal[0] = self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
        normal[2] = -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

        hit_record.p = p;
        hit_record.set_face_normal(&rotated_r, normal);

        return true;
    }
}
pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}
impl ConstantMedium {
    pub fn from(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::from(a)),
        }
    }
}
impl Hittable for ConstantMedium {
    fn bounding_box(&self, t0: f64, t1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(t0, t1, output_box)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();
        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * (random_double().ln());
        if hit_distance > distance_inside_boundary {
            return false;
        }
        hit_record.t = rec1.t + hit_distance / ray_length;
        hit_record.p = r.at(hit_record.t);
        hit_record.normal = Vec3::from(1.0, 0.0, 0.0); // arbitrary
        hit_record.front_face = true; // also arbitrary
        hit_record.material = Some(self.phase_function.clone());
        return true;
    }
}
