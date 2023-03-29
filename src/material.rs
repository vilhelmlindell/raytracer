use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Vec3, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray);
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Vec3, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray) {}
}
