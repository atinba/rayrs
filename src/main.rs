use rayrs::raytracer::{Raytracer, RenderConfig};
use rayrs::scene::Scene;
use rayrs::sphere::Sphere;
use rayrs::vec3::Point3;

const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let render_config = RenderConfig {
        resolution: (IMAGE_WIDTH, IMAGE_HEIGHT),
        aspect_ratio: ASPECT_RATIO,
    };

    let mut world = Scene::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let raytracer = Raytracer::new(render_config, world);

    raytracer.render();
}
