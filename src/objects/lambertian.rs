use crate::color::Color;
use crate::objects::hittable::HitRecord;
use crate::objects::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    #[inline(always)]
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction =
            record.get_normal() + Vec3::random_in_unit_sphere().unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.get_normal();
        }
        Some((
            self.albedo,
            Ray::new(record.get_point(), scatter_direction, ray_in.get_time()),
        ))
    }
}
