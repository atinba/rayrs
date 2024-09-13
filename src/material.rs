use std::sync::Arc;

use crate::ray::Ray;
use crate::scene::HitRecord;
use crate::sphere::Sphere;
use crate::utils;
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction().reflect(&hit_rec.normal);
        let scattered = Ray::new(
            hit_rec.p,
            reflected.unit() + self.fuzz * Vec3::rand_unit_vec(),
        );

        if scattered.direction().dot(&hit_rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cos: f64, ri: f64) -> f64 {
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 *= r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cos, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        let ri = if hit_rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_dir = r_in.direction().unit();

        let cos_t = f64::min(-unit_dir.dot(&hit_rec.normal), 1.0);
        let sin_t = f64::sqrt(1.0 - cos_t * cos_t);

        let cant_reflect = ri * sin_t > 1.0;

        let dir = if cant_reflect || Self::reflectance(cos_t, ri) > utils::rand_f64() {
            unit_dir.reflect(&hit_rec.normal)
        } else {
            unit_dir.refract(&hit_rec.normal, ri)
        };

        Some(ScatterRecord {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hit_rec.p, dir),
        })
    }
}
