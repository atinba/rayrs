use std::io;

use crate::color::{self, Color};
use crate::ray::Ray;
use crate::scene::{HitRecord, Hittable, Scene};
use crate::utils::rand_f64;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    center: Point3,
    pixel00_loc: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(config: &RenderConfig) -> Self {
        let (image_width, image_height) = config.resolution;
        let samples_per_pixel = config.samples_per_pixel;

        let focal_length = 1.0;
        let viewport_height = 2.0;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_du = viewport_u / image_width as f64;
        let pixel_dv = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_du + pixel_dv) / 2.0;

        Camera {
            center,
            pixel00_loc,
            pixel_du,
            pixel_dv,
            pixel_samples_scale,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let [x, y, _] = Camera::sample_square().xyz();

        let x = x + i as f64;
        let y = y + j as f64;
        let pixel_sample = self.pixel00_loc + (x * self.pixel_du) + (y * self.pixel_dv);
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    // Anti-Aliasing
    fn sample_square() -> Vec3 {
        Vec3::new(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
    }
}

pub struct RenderConfig {
    pub resolution: (u32, u32),
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32
}

pub struct Raytracer {
    scene: Scene,
    camera: Camera,
    config: RenderConfig,
}

impl Raytracer {
    pub fn new(config: RenderConfig, scene: Scene) -> Self {
        let camera = Camera::new(&config);

        Self {
            scene,
            camera,
            config,
        }
    }

    pub fn render(&self) {
        let (image_width, image_height) = self.config.resolution;

        println!("P3\n{image_width} {image_height}\n255\n");

        for j in 0..image_height {
            eprint!("\rScanlines remaining: {} ", image_height - j);

            for i in 0..image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.config.samples_per_pixel {
                    pixel_color += self.ray_color(&self.camera.get_ray(i, j));
                }

                color::write_color(
                    &mut io::stdout(),
                    self.camera.pixel_samples_scale * pixel_color,
                );
            }
        }

        eprint!("\rDone.                 \n");
    }
    fn ray_color(&self, r: &Ray) -> Color {
        let mut rec = HitRecord::new();
        if self.scene.hit(r, 0.0, f64::INFINITY, &mut rec) {
            // Map [-1,1] to [0,1]
            return (rec.normal + Color::new(1.0, 1.0, 1.0)) / 2.0;
        }

        let unit_direction = r.direction().unit();
        let a: f64 = (unit_direction.y() + 1.0) / 2.0;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
