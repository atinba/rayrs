use std::sync::Arc;
use std::time::Instant;

use rayrs::material::{Dielectric, Lambertian, Metal};
use rayrs::raytracer::{Raytracer, RenderConfig};
use rayrs::scene::Scene;
use rayrs::utils::Color;

const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let render_config = RenderConfig {
        resolution: (IMAGE_WIDTH, IMAGE_HEIGHT),
        aspect_ratio: ASPECT_RATIO,
        samples_per_pixel: 10,
        max_depth: 10,
    };

    let mut world = Scene::new();

    let mat_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add_sphere((0.0, -100.5, -1.0), 100.0, mat_ground);
    world.add_sphere((0.0, 0.0, -1.2), 0.5, mat_center);
    world.add_sphere((-1.0, 0.0, -1.0), 0.5, mat_left);
    world.add_sphere((-1.0, 0.0, -1.0), 0.4, mat_bubble);
    world.add_sphere((1.0, 0.0, -1.0), 0.5, mat_right);

    let raytracer = Raytracer::new(render_config, world);

    let start_time = Instant::now();

    raytracer.render_p();

    let elapsed_time = start_time.elapsed();
    eprintln!("\rDone. Time taken: {:.2?}", elapsed_time);
}
