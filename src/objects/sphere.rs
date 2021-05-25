use std::sync::Arc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use crate::objects::aabb::AABB;
use crate::objects::bounding_box::BoundingBox;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::objects::material::Material;

#[derive(Clone)]
pub struct Sphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    #[inline(always)]
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material + Sync + Send>,
    ) -> Sphere {
        Sphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    #[inline(always)]
    pub fn new_immobile(
        center: Point3,
        radius: f64,
        material: Arc<dyn Material + Sync + Send>,
    ) -> Sphere {
        Sphere {
            center0: center,
            center1: center,
            time0: 0.0,
            time1: std::f64::INFINITY,
            radius,
            material,
        }
    }

    #[inline(always)]
    pub fn get_center(&self, time: f64) -> Point3 {
        if self.center0 == self.center1 {
            self.center0
        } else {
            self.center0
                + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
        }
    }

    #[inline(always)]
    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.get_origin() - self.get_center(r.get_time());
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
        let outward_normal = (p - self.get_center(r.get_time())) / self.radius;
        Some(HitRecord::from_outward_normal(
            p,
            root,
            r,
            outward_normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Box<dyn BoundingBox>> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        if self.center0 == self.center1 {
            Some(Box::new(AABB::new(
                self.center0 - radius_vec,
                self.center0 + radius_vec,
            )))
        } else {
            let b0 = Box::new(AABB::new(
                self.get_center(self.time0) - radius_vec,
                self.get_center(self.time0) + radius_vec,
            ));
            let b1 = Box::new(AABB::new(
                self.get_center(self.time1) - radius_vec,
                self.get_center(self.time1) + radius_vec,
            ));
            Some(Box::new(AABB::surrounding_box(*b0, *b1)))
        }
    }
}
