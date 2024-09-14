use std::sync::Arc;
use std::time::Instant;

use rayrs::material::{Dielectric, Lambertian, Metal};
use rayrs::raytracer::{Raytracer, RenderConfig};
use rayrs::scene::Scene;
use rayrs::vec3::Point3;
use rayrs::utils::{self, Color};

const IMAGE_WIDTH: u32 = 1200;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let render_config = RenderConfig {
        resolution: (IMAGE_WIDTH, IMAGE_HEIGHT),
        aspect_ratio: ASPECT_RATIO,
        samples_per_pixel: 500,
        max_depth: 50,
    };

    let mut world = Scene::new();

    let mat_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add_sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::rand_f64();

            let center = Point3::new(
                a as f64 + 0.9 * utils::rand_f64(),
                0.2,
                b as f64 + 0.9 * utils::rand_f64(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::rand() * Color::rand();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add_sphere(center, 0.2, sphere_material);
                } else if choose_mat < 0.95 {
                    let albedo = Color::rand_range(0.5, 1.0);
                    let fuzz = utils::rand_range_f64(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add_sphere(center, 0.2, sphere_material);
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add_sphere(center, 0.2, sphere_material);
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    world.add_sphere(Point3::new(0.0, 1.0, 0.0), 1.0, material1);
    world.add_sphere(Point3::new(-4.0, 1.0, 0.0), 1.0, material2);
    world.add_sphere(Point3::new(4.0, 1.0, 0.0), 1.0, material3);

    let raytracer = Raytracer::new(render_config, world);

    //let start_time = Instant::now();

    raytracer.render_p();

    //let elapsed_time = start_time.elapsed();
    //eprintln!("\rDone. Time taken: {:.2?}", elapsed_time);
}
