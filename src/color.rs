use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let [r, g, b] = pixel_color
        .xyz()
        .map(|c| (256.0 * c.clamp(0.000, 0.999)) as i32);
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
