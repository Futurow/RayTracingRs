use crate::rtweekend::PI;

use super::rtweekend::*;
use std::io::Write;
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}
//初始化与获取值
impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0; 3] }
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn write_color(&self, samples_per_pixel: i32, out: &mut dyn Write) -> std::io::Result<()> {
        let scale: f64 = 1.0 / samples_per_pixel as f64;
        let r: f64 = (scale * self.e[0]).sqrt();
        let g: f64 = (scale * self.e[1]).sqrt();
        let b: f64 = (scale * self.e[2]).sqrt();
        write!(
            out,
            "{} {} {}\n",
            (256.0 * clamp(r, 0.0, 0.999)) as i32,
            (256.0 * clamp(g, 0.0, 0.999)) as i32,
            (256.0 * clamp(b, 0.0, 0.999)) as i32
        )
    }
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }
    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let len = v.length();
        v / len
    }
    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_double(), random_double(), random_double()],
        }
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_double_range(min, max),
                random_double_range(min, max),
                random_double_range(min, max),
            ],
        }
    }
    //lambertian漫散射模型近似
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p: Vec3 = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    //lambertian漫散射模型近似
    pub fn random_unit_vector() -> Vec3 {
        let a = random_double_range(0.0, 2.0 * PI);
        let z = random_double_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        return Vec3::from(r * a.cos(), r * a.sin(), z);
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(&v, &n) * n
    }
    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&(-uv), &n);
        let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
        let r_out_perp = -((1.0 - r_out_parallel.length_squared()).sqrt()) * n;
        return r_out_parallel + r_out_perp;
    }
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::from(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.e[idx]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        (1 as f64 / t) * self
    }
}
impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
