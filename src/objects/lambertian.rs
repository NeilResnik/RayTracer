
use crate::color::Color;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::objects::material::Material;

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{ albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = record.get_normal() + Vec3::random_in_unit_sphere().unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.get_normal();
        }
        Some((self.albedo, Ray::new(record.get_point(), scatter_direction)))
    }
}