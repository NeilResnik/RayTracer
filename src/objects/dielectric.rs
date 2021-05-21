use std::ops::Neg;

use crate::color::Color;
use crate::ray::Ray;

use crate::objects::hittable::HitRecord;
use crate::objects::material::Material;

use rand::random;

pub struct Dielectric {
    index_of_refraction: f64
}

impl Dielectric {
    #[inline(always)]
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric{ index_of_refraction }
    }

    #[inline(always)]
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.is_front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = ray_in.get_direction().unit_vector();
        let cos_theta = unit_direction.neg().dot(&record.get_normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let direction = if refraction_ratio * sin_theta > 1.0 
                            || self.reflectance(cos_theta, refraction_ratio) > random::<f64>() {
            unit_direction.reflect(&record.get_normal())
        } else {
            unit_direction.refract(&record.get_normal(), refraction_ratio)
        };
        Some((Color::new(255, 255, 255), Ray::new(record.get_point(), direction)))
    }
}
