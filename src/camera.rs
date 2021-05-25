use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use rand::prelude::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time_start: f64,
    time_end: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vertical_fov_degrees: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        time_start: f64,
        time_end: f64,
    ) -> Camera {
        let theta = vertical_fov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;

        Camera {
            origin: look_from,
            lower_left_corner: look_from
                - (horizontal / 2.0)
                - (vertical / 2.0)
                - (focus_distance * w),
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
            time_start,
            time_end,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = (self.u * rd.get_x()) + (self.v * rd.get_y());
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
            thread_rng().gen_range(self.time_start..self.time_end),
        )
    }
}
