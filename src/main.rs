mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use std::io;
use vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let ray_dir = r.direction();
    let temp = *center - r.origin();

    let a = ray_dir.len_sq();
    let b = -2.0 * ray_dir.dot(&temp);
    let c = temp.len_sq() - radius * radius;

    b * b >= 4.0 * a * c
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit();
    let a: f64 = (unit_direction.y() + 1.0) / 2.0;
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
}
