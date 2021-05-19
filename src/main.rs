use std::convert::TryFrom;

use raytracer::color::Color;
use raytracer::ppm::create_ppm;
use raytracer::ray::Ray;
use raytracer::vec3::{Point3, Vec3};

use raytracer::hittables::hittable::Hittable;
use raytracer::hittables::sphere::Sphere;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    Color::try_from(
        if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
            0.5 * (rec.get_normal() + Vec3::new(1.0, 1.0, 1.0))
        } else {
            let unit_dir = r.get_direction().unit_vector();
            let t = 0.5 * (unit_dir.get_y() + 1.0);
            ((1.0 - t) * Vec3::from(Color::new(255, 255, 255))) + (t * Vec3::from(Color::new(128, 179, 255)))
        }
    ).unwrap()
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let world = vec!(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
                     Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    let mut image = Vec::new();
    for j in 0..image_height {
        let mut row = Vec::new();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = (image_height - 1 - j) as f64 / (image_height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + (u * horizontal) + (v * vertical) - origin);
            row.push(ray_color(&r, &world));
        }
        image.push(row);
    }

    // Render
    create_ppm(std::io::stdout(), image).unwrap();
}
