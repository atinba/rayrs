use rand::Rng;

pub use std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rand_f64() -> f64 {
    rand::thread_rng().gen()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}
