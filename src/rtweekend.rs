use rand::Rng;
pub const INFINITY: f64 = std::f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
pub use std::rc::Rc;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    // min + (max - min) * random_double()
    rand::thread_rng().gen_range(min..max)
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
pub fn ffmin(a: f64, b: f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}
pub fn ffmax(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}
