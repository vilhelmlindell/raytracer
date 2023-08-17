use crate::ray::Ray;
use crate::Vec3;
use crate::{camera::Camera, hittable::Hittable, hittable_list::HittableList};
use image::RgbImage;
use rand::Rng;

pub fn render_image(settings: &RenderSettings, world: &HittableList, camera: &Camera) -> RgbImage {
    let mut image = RgbImage::new(settings.image_width, settings.image_height);

    let chunk_size = image.len() / settings.num_threads;
    let mut pixel_index = 0;

    crossbeam::thread::scope(|s| {
        for chunk in image.chunks_mut(chunk_size) {
            let chunk_length = chunk.len();
            s.spawn(move |_| {
                render_image_chunk(chunk, pixel_index, settings, world, camera);
            });
            pixel_index += chunk_length / 3;
        }
    })
    .unwrap();

    image
}

fn render_image_chunk(
    chunk: &mut [u8],
    pixel_start_index: usize,
    settings: &RenderSettings,
    world: &HittableList,
    camera: &Camera,
) {
    let mut thread_rng = rand::thread_rng();

    for (index, pixel) in chunk.chunks_mut(3).enumerate() {
        let pixel_index = pixel_start_index + index;
        let x = pixel_index as u32 % settings.image_width;
        let y = settings.image_height - pixel_index as u32 / settings.image_width;
        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
        for _sample in 0..settings.samples_per_pixel {
            let horizontal_delta =
                (x as f64 + thread_rng.gen_range(0.0..1.0)) / (settings.image_width - 1) as f64;
            let vertical_delta =
                (y as f64 + thread_rng.gen_range(0.0..1.0)) / (settings.image_height - 1) as f64;
            let ray = camera.get_viewport_ray(horizontal_delta, vertical_delta);
            pixel_color = pixel_color + ray_color(&ray, world, settings.max_depth);
        }
        sample_color(&mut pixel_color, settings.samples_per_pixel);
        pixel[0] = (pixel_color.x * 255.0).round() as u8;
        pixel[1] = (pixel_color.y * 255.0).round() as u8;
        pixel[2] = (pixel_color.z * 255.0).round() as u8;
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = world.is_hit(ray, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3::default();
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

pub struct RenderSettings {
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub num_threads: usize,
}

impl RenderSettings {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        samples_per_pixel: u32,
        max_depth: u32,
        num_threads: usize,
    ) -> Self {
        RenderSettings {
            image_width,
            image_height: (image_width as f64 / aspect_ratio) as u32,
            samples_per_pixel,
            max_depth,
            num_threads,
        }
    }
}
