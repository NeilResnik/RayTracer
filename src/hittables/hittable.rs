use std::option::Option;
//use std::iter::Iterator;
use std::vec::Vec;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        HitRecord{ point, normal, t, front_face }
    }

    pub fn from_outward_normal(point: Point3, t: f64, r: &Ray, outward_normal: Vec3) -> HitRecord {
        let front_face = r.get_direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord::new(point, normal, t, front_face)
    }

    pub fn get_point(&self) -> Point3 {
        self.point
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn is_front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

// TODO: Implement for all iterators, not just vecs
// The fucking iterator trait seems to be a headache here since 
// it requires mutable self but hit doesn't
// Maybe ask on a forum
// We come back to this
// We aren't done here
// Fuck you language
//impl Hittable for dyn Iterator<Item = &Box<dyn Hittable>> {
impl Hittable for Vec<Box<dyn Hittable>> {
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
