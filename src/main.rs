use std::convert::TryFrom;
use std::rc::Rc;
use rand::prelude::*;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::ppm::create_ppm;
use raytracer::ray::Ray;
use raytracer::vec3::Vec3;

use raytracer::objects::hittable::Hittable;
use raytracer::objects::lambertian::Lambertian;
use raytracer::objects::metal::Metal;
use raytracer::objects::sphere::Sphere;

fn ray_color_vec(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3
{
    if depth <= 0 {
        Vec3::new(0.0, 0.0, 0.0)
    }else if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.get_material().scatter(r, &rec) {
            Vec3::from(attenuation) * ray_color_vec(&scattered, world, depth-1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_dir = r.get_direction().unit_vector();
        let t = 0.5 * (unit_dir.get_y() + 1.0);
        ((1.0 - t) * Vec3::from(Color::new(255, 255, 255))) + (t * Vec3::from(Color::new(128, 179, 255)))
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let ground_material = Rc::new(Lambertian::new(Color::new(204, 204, 0)));
    let center_material = Rc::new(Lambertian::new(Color::new(179, 76, 76)));
    let left_material = Rc::new(Metal::new(Color::new(204, 204, 204)));
    let right_material = Rc::new(Metal::new(Color::new(204, 153, 51)));

    let world = vec!(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material),
                     Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, center_material),
                     Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, left_material),
                     Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, right_material));

    // Camera
    let cam = Camera::new();

    // RNG
    let mut rng = rand::thread_rng();

    let mut image = Vec::new();
    for j in 0..image_height {
        let mut row = Vec::new();
        for i in 0..image_width {
            let mut pixel_color_vec = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = ((image_height - 1 - j) as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color_vec += ray_color_vec(&r, &world, max_depth);
            }
            pixel_color_vec.scale_in_range(1.0 / samples_per_pixel as f64, 0.0, 0.999);
            row.push(Color::try_from(pixel_color_vec).unwrap());
        }
        image.push(row);
    }

    // Render
    create_ppm(std::io::stdout(), image).unwrap();
}
