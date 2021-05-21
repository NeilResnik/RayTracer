
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: Point3, look_at: Point3, view_up: Vec3, vertical_fov_degrees: f64, aspect_ratio: f64) -> Camera {
        let theta = vertical_fov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        Camera{ 
            origin: look_from,
            lower_left_corner: look_from - (horizontal / 2.0) - (vertical / 2.0) - w,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + (s * self.horizontal) + (t * self.vertical) - self.origin)
    }
}

