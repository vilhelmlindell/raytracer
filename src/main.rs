mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;
use camera::Camera;
use hittable::Hittable;
use hittable_list::HittableList;
use image::RgbImage;
use rand::{thread_rng, Rng};
use sphere::Sphere;

fn ray_color(ray: &Ray, world: &dyn Hittable, max_depth: u32) -> Vec3 {
    if max_depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.is_hit(ray, 0.001, f64::INFINITY) {
        let target = hit_record.point + Vec3::random_in_hemisphere(&hit_record.normal);
        return 0.5
            * ray_color(
                &Ray::new(hit_record.point, target - hit_record.point),
                world,
                max_depth - 1,
            );
    }
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
fn sample_color(pixel_color: &mut Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;
    *pixel_color = *pixel_color * scale;
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    pixel_color.x = f64::sqrt(pixel_color.x.clamp(0.0, 0.999));
    pixel_color.y = f64::sqrt(pixel_color.y.clamp(0.0, 0.999));
    pixel_color.z = f64::sqrt(pixel_color.z.clamp(0.0, 0.999));
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 20;
    let max_depth = 50;

    let mut image = RgbImage::new(image_width, image_height);

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut thread_rng = thread_rng();

    for x in 0..image_width {
        for y in 0..image_height {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for sample in 0..samples_per_pixel {
                let u = (x as f64 + thread_rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                let v = (y as f64 + thread_rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;
                let ray = camera.get_ray(&u, &v);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
            }
            sample_color(&mut pixel_color, samples_per_pixel);
            image.put_pixel(x, image_height - y - 1, pixel_color.into());
        }
    }

    image.save("output.png").unwrap();
}
