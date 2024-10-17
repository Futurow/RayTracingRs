use super::hittable::HitRecord;
use super::ray::Ray;
use super::rtweekend::{ffmin, random_double};
use super::vec3::Vec3;
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn from(a: Vec3) -> Lambertian {
        Lambertian { albedo: a }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray::from(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}
impl Metal {
    pub fn from(a: Vec3, fuzz: f64) -> Metal {
        Metal { albedo: a, fuzz }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction()), rec.normal);
        *scattered = Ray::from(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
pub struct Dielectric {
    ref_idx: f64,
}
impl Dielectric {
    pub fn from(ri: f64) -> Dielectric {
        Dielectric { ref_idx: ri }
    }
    pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * ((1.0 - cosine).powi(5));
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::from(1.0, 1.0, 1.0);
        let mut etai_over_etat: f64 = self.ref_idx;
        if rec.front_face {
            etai_over_etat = 1.0 / self.ref_idx;
        }
        let unit_direction = Vec3::unit_vector(r_in.direction());
        let cos_theta = ffmin(Vec3::dot(&(-unit_direction), &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::from(rec.p, reflected, r_in.time());
            return true;
        }
        let reflect_prob = Dielectric::schlick(cos_theta, etai_over_etat);
        if random_double() < reflect_prob {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray::from(rec.p, reflected, r_in.time());
            return true;
        }
        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray::from(rec.p, refracted, r_in.time());
        return true;
    }
}
