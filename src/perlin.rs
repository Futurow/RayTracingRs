use rand::Rng;

use crate::vec3::Vec3;
const POINT_COUNT: usize = 256;
pub struct Pelin {
    ranfloat: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}
impl Pelin {
    pub fn new() -> Pelin {
        let mut ranfloat = vec![Vec3::new(); POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranfloat[i] = Vec3::unit_vector(Vec3::random_range(-1.0, 1.0));
        }
        Pelin {
            ranfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let v0 = Vec3::new();
        let mut c = [[[v0, v0], [v0, v0]], [[v0, v0], [v0, v0]]];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }
        return trilinear_interp(&c, u, v, w);
    }
    pub fn turb(&self, p: &Vec3, depth: Option<usize>) -> f64 {
        let mut accum = 0.0;
        let depth = depth.unwrap_or(7);
        let mut temp_p = Vec3::from(p.x(), p.y(), p.z());
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = vec![0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }
    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (1..n).rev() {
            let mut rng = rand::thread_rng();
            let target = rng.gen_range(0..=i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::from(u - i as f64, v - j as f64, w - k as f64);
                accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                    * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                    * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                    * Vec3::dot(&c[i][j][k], &weight_v);
            }
        }
    }
    accum
}
