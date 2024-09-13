use std::sync::Arc;

use crate::material::Material;
use crate::ray::Ray;
use crate::scene::{HitRecord, Hittable};
use crate::vec3::Point3;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let ray_dir = r.direction();
        let oc = self.center - r.origin();

        let a = ray_dir.len_sq();
        let h = ray_dir.dot(&oc);
        let c = oc.len_sq() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();
        let mut root = (h - d_sqrt) / a;

        // TODO: maybe buggy?! tmin is included in the range
        let range = ray_tmin..=ray_tmax;
        if !range.contains(&root) {
            root = (h + d_sqrt) / a;
            if !range.contains(&root) {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat: self.mat.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };

        let out_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, out_normal);

        Some(rec)
    }
}
