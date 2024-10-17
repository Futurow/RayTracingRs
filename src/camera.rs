use crate::rtweekend::random_double_range;

use super::ray::Ray;
use super::rtweekend::degrees_to_radians;
use super::vec3::Vec3;
pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    time0: f64,
    time1: f64,
}
impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Camera {
        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let theta: f64 = degrees_to_radians(vfov);
        let half_height: f64 = (theta / 2.0).tan();
        let half_width: f64 = aspect * half_height;
        Camera {
            lower_left_corner: lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u,
            v,
            _w: w,
            time0: t0,
            time1: t1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::from(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_double_range(self.time0, self.time1),
        )
    }
}
