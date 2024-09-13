use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::Color;
use crate::vec3::{Point3, Vec3};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn add_sphere(&mut self, center: (f64, f64, f64), radius: f64, mat: Arc<dyn Material>) {
        let (x, y, z) = center;
        self.add(Box::new(Sphere::new(Point3::new(x, y, z), radius, mat)));
    }
}

impl Hittable for Scene {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, ray_tmin, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec)
            }
        }

        rec
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, out_normal: Vec3) {
        self.front_face = r.direction().dot(&out_normal) < 0.0;

        self.normal = if self.front_face {
            out_normal
        } else {
            -out_normal
        };
    }
}
