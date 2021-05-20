
use crate::color::Color;
use crate::ray::Ray;
use crate::objects::hittable::HitRecord;
use crate::objects::material::Material;

pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal{ albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.get_direction().unit_vector().reflect(&record.get_normal());
        let scattered = Ray::new(record.get_point(), reflected);
        if scattered.get_direction().dot(&record.get_normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
