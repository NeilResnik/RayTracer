use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::Point3;

use crate::objects::hittable::{HitRecord, Hittable};
use crate::objects::material::Material;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    #[inline(always)]
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    #[inline(always)]
    pub fn get_center(&self) -> Point3 {
        self.center
    }

    #[inline(always)]
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.get_origin() - self.center;
        let a = r.get_direction().length_squared();
        let half_b = oc.dot(&r.get_direction());
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - (a * c);
        if discriminant < 0.0 {
            return None;
        }

        let d_sqrt = discriminant.sqrt();
        let mut root = (-half_b - d_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + d_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::from_outward_normal(
            p,
            root,
            r,
            outward_normal,
            self.material.clone(),
        ))
    }
}
