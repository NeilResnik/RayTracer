use std::convert::{From, Into, TryFrom, TryInto};
use std::sync::Arc;

use image::{ColorType, ImageFormat, save_buffer_with_format};
use rand::prelude::*;
use rayon::prelude::*;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::ray::Ray;
use raytracer::vec3::{Point3, Vec3};

use raytracer::objects::dielectric::Dielectric;
use raytracer::objects::hittable::Hittable;
use raytracer::objects::lambertian::Lambertian;
use raytracer::objects::material::Material;
use raytracer::objects::metal::Metal;
use raytracer::objects::sphere::Sphere;

fn random_scene<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec<Box<dyn Hittable + Sync + Send>> {
    let ground_material = Arc::new(Lambertian::new(Color::new(127, 127, 127)));
    let mut world: Vec<Box<dyn Hittable + Sync + Send>> = vec![Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))];

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + (0.9 * rng.gen::<f64>()),
                0.2,
                b as f64 + (0.9 * rng.gen::<f64>()),
            );
            let mat_roll: f64 = rng.gen();
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Arc<dyn Material + Sync + Send> = if mat_roll < 0.8 {
                    // diffuse
                    let albedo = Color::try_from(
                        Vec3::random_with_gen(rng, 0.0, 1.0) * Vec3::random_with_gen(rng, 0.0, 1.0),
                    )
                    .unwrap();
                    Arc::new(Lambertian::new(albedo))
                } else if mat_roll < 0.95 {
                    // metal
                    let albedo = Color::try_from(Vec3::random_with_gen(rng, 0.5, 1.0)).unwrap();
                    let fuzz = rng.gen_range(0.0..0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(102, 51, 25)));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(178, 153, 127), 0.0));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn ray_color_vec(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        Vec3::new(0.0, 0.0, 0.0)
    } else if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.get_material().scatter(r, &rec) {
            Vec3::from(attenuation) * ray_color_vec(&scattered, world, depth - 1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_dir = r.get_direction().unit_vector();
        let t = 0.5 * (unit_dir.get_y() + 1.0);
        ((1.0 - t) * Vec3::from(Color::new(255, 255, 255)))
            + (t * Vec3::from(Color::new(128, 179, 255)))
    }
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // RNG
    let mut rng = rand::thread_rng();

    // World
    let world = random_scene(&mut rng);

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    // Generate
    let raw_img_buf: Vec<u8> = vec![vec![[0_u8; 3]; image_width]; image_height]
        .into_par_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_par_iter()
               .enumerate()
               .flat_map(|(x, _s)|{
                    let mut pixel_color_vec = Vec3::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel {
                        let u = (x as f64 + random::<f64>()) / (image_width - 1) as f64;
                        let v =
                            ((image_height - 1 - y) as f64 + random::<f64>()) / (image_height - 1) as f64;
                        let r = cam.get_ray(u, v);
                        pixel_color_vec += ray_color_vec(&r, &world, max_depth);
                    }
                    pixel_color_vec.scale_in_range(1.0 / samples_per_pixel as f64, 0.0, 0.999);
                    let rgb_slice: [u8; 3] = Color::try_from(pixel_color_vec).unwrap().into();
                    rgb_slice
               }).collect::<Vec<u8>>()
           }).collect::<Vec<u8>>();
    // let raw_img_buf : Array1::<(usize, u8)> = Array3::<u8>::zeros((10, 10, 3))
    //                                             .into_iter()
    //                                             .enumerate()
    //                                             .map(|v| { v })
    //                                             .collect();
    // let raw_img_buf = vec![vec![[0_u8; 3]; image_height]; image_width].par_iter()
    //                                                                   .map();
    // let img = ImageBuffer::from_fn(image_width, image_height, |x, y| {
    //     let mut pixel_color_vec = Vec3::new(0.0, 0.0, 0.0);
    //     for _ in 0..samples_per_pixel {
    //         let u = (x as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
    //         let v =
    //             ((image_height - 1 - y) as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
    //         let r = cam.get_ray(u, v);
    //         pixel_color_vec += ray_color_vec(&r, &world, max_depth);
    //     }
    //     pixel_color_vec.scale_in_range(1.0 / samples_per_pixel as f64, 0.0, 0.999);
    //     Rgb::from(Color::try_from(pixel_color_vec).unwrap())
    // });

    // Render
    save_buffer_with_format("test.png",
                            &raw_img_buf,
                            image_width.try_into().unwrap(),
                            image_height.try_into().unwrap(),
                            ColorType::Rgb8,
                            ImageFormat::Png).unwrap();
}
