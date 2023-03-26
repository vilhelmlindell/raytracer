mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::ray::Ray;
use crate::vec3::Vec3;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{Rgb, RgbImage};
use sphere::Sphere;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Rgb<u8> {
    if let Some(hit_record) = world.is_hit(ray, &0.0, &f64::INFINITY) {
        return Rgb::from(0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0)));
    }
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y + 1.0);
    Rgb::from((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    let mut image = RgbImage::new(image_width, image_height);

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for x in 0..image_width {
        for y in 0..image_height {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray, &world);
            image.put_pixel(x, image_height - y - 1, pixel_color);
        }
    }

    image.save("output.png").unwrap();
}
