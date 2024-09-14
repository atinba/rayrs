use rayon::prelude::*;
use std::io;

//use crate::color::{self, Color};
use crate::ray::Ray;
use crate::scene::{Hittable, Scene};
use crate::utils::{self, rand_f64, Color};
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    center: Point3,
    pixel00_loc: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    defocus_angle: f64,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(config: &RenderConfig) -> Self {
        let (image_width, image_height) = config.resolution;
        let samples_per_pixel = config.samples_per_pixel;

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let vfov = 20.0;
        let defocus_angle = 0.6;
        let focus_dist = 10.0;

        let lookfrom = Point3::new(13.0, 2.0, 3.0);
        let lookat = Point3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let theta = utils::deg_to_rad(vfov);
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let center = lookfrom;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u).unit();

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_du = viewport_u / image_width as f64;
        let pixel_dv = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * w) - (viewport_u + viewport_v) / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_du + pixel_dv) / 2.0;

        let defocus_radius = focus_dist * f64::tan(utils::deg_to_rad(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            center,
            pixel00_loc,
            pixel_du,
            pixel_dv,
            pixel_samples_scale,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let [x, y, _] = Camera::sample_square().xyz();

        let x = x + i as f64;
        let y = y + j as f64;
        let pixel_sample = self.pixel00_loc + (x * self.pixel_du) + (y * self.pixel_dv);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::rand_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    // Anti-Aliasing
    fn sample_square() -> Vec3 {
        Vec3::new(rand_f64() - 0.5, rand_f64() - 0.5, 0.0)
    }
}

pub struct RenderConfig {
    pub resolution: (u32, u32),
    pub aspect_ratio: f64,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
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

    pub fn render_p(&self) {
        let (image_width, image_height) = self.config.resolution;
        let max_depth = self.config.max_depth;

        println!("P3\n{image_width} {image_height}\n255\n");

        for j in 0..image_height {
            eprint!("\rScanlines remaining: {} ", image_height - j);

            let pixel_colors: Vec<_> = (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..self.config.samples_per_pixel {
                        pixel_color += self.ray_color(&self.camera.get_ray(i, j), max_depth);
                    }

                    pixel_color
                })
                .collect();

            for pixel_color in pixel_colors {
                utils::write_color(
                    &mut io::stdout(),
                    self.camera.pixel_samples_scale * pixel_color,
                );
            }
        }
    }

    pub fn render(&self) {
        let (image_width, image_height) = self.config.resolution;
        let max_depth = self.config.max_depth;

        println!("P3\n{image_width} {image_height}\n255\n");

        for j in 0..image_height {
            eprint!("\rScanlines remaining: {} ", image_height - j);

            for i in 0..image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.config.samples_per_pixel {
                    pixel_color += self.ray_color(&self.camera.get_ray(i, j), max_depth);
                }

                utils::write_color(
                    &mut io::stdout(),
                    self.camera.pixel_samples_scale * pixel_color,
                );
            }
        }
    }

    fn ray_color(&self, r: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = self.scene.hit(r, 0.001, f64::INFINITY) {
            if let Some(scatter) = rec.mat.scatter(r, &rec) {
                return scatter.attenuation * self.ray_color(&scatter.scattered, depth - 1);
            }
            //let ray = Ray::new(rec.p, rec.normal + Vec3::rand_unit_vec());
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.direction().unit();
        let a: f64 = (unit_direction.y() + 1.0) / 2.0;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
