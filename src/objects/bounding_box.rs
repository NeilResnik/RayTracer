use std::cmp::{Ordering, PartialEq};

use crate::ray::Ray;
use crate::vec3::Point3;

pub trait BoundingBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool;
    fn get_lower(&self) -> Point3;
    fn get_upper(&self) -> Point3;
    fn cmp_axis(&self, other: &Box<dyn BoundingBox>, axis: usize) -> Ordering;
}

impl PartialEq for dyn BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.get_lower() == other.get_lower() && self.get_upper() == self.get_upper()
    }
}
