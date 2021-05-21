
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::objects::hittable::HitRecord;
use crate::objects::material::Material;

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    #[inline(always)]
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal{ 
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.get_direction().unit_vector().reflect(&record.get_normal());
        let scattered = Ray::new(record.get_point(), reflected + (self.fuzz * Vec3::random_in_unit_sphere()));
        if scattered.get_direction().dot(&record.get_normal()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
