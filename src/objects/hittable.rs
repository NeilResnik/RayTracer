use std::boxed::Box;
use std::option::Option;
//use std::iter::Iterator;
use std::sync::Arc;
use std::vec::Vec;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use crate::objects::bounding_box::BoundingBox;
use crate::objects::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    #[inline(always)]
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }

    pub fn from_outward_normal(
        point: Point3,
        t: f64,
        r: &Ray,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
    ) -> HitRecord {
        let front_face = r.get_direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord::new(point, normal, material, t, front_face)
    }

    #[inline(always)]
    pub fn get_point(&self) -> Point3 {
        self.point
    }

    #[inline(always)]
    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    #[inline(always)]
    pub fn get_material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }

    #[inline(always)]
    pub fn get_t(&self) -> f64 {
        self.t
    }

    #[inline(always)]
    pub fn is_front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Box<dyn BoundingBox>>;
}

impl Hittable for Vec<Box<dyn Hittable + Sync + Send>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest = t_max;
        for obj in self.iter() {
            if let Some(hr) = obj.hit(r, t_min, closest) {
                closest = hr.get_t();
                rec = Some(hr);
            }
        }
        rec
    }
}

impl<T: Hittable + Sync + Send> Hittable for Vec<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest = t_max;
        for obj in self.iter() {
            if let Some(hr) = obj.hit(r, t_min, closest) {
                closest = hr.get_t();
                rec = Some(hr);
            }
        }
        rec
    }
}
