#![feature(iter_array_chunks)]

mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod render;
mod sphere;
mod vec3;

use crate::vec3::Vec3;
use camera::Camera;
use hittable_list::HittableList;
use material::Lambertian;
use material::Metal;
use render::render_image;
use render::RenderSettings;
use sphere::Sphere;

fn main() {
    let available_threads = std::thread::available_parallelism().unwrap().into();
    let settings = RenderSettings::new(800, 16.0 / 9.0, 100, 50, available_threads);

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Vec3::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let camera = Camera::new();

    render_image(&settings, &world, &camera)
        .save("output.png")
        .unwrap();
}
