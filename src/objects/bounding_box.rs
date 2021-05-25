use crate::ray::Ray;

pub trait BoundingBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool;
}
