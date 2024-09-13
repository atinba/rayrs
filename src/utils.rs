use rand::Rng;
use std::f64::consts::PI;
use std::io::Write;

use crate::vec3::Vec3;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn rand_f64() -> f64 {
    rand::thread_rng().gen()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}

// Color

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let [r, g, b] = pixel_color
        .xyz()
        .map(|c| (256.0 * linear_to_gamma(c).clamp(0.000, 0.999)) as i32);
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
