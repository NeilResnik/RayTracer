use crate::color::Color;
use crate::ray::Ray;

use crate::objects::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}
