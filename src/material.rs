use std::sync::Arc;

use crate::ray::Ray;
use crate::scene::HitRecord;
use crate::sphere::Sphere;
use crate::utils::Color;
use crate::vec3::Vec3;

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_dir = hit_rec.normal + Vec3::rand_unit_vec();

        if scatter_dir.is_near_zero() {
            scatter_dir = hit_rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit_rec.p, scatter_dir),
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction().reflect(&hit_rec.normal);

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit_rec.p, reflected),
        })
    }
}
