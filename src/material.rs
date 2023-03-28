use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Vec3, hit_record: &HitRecord, attenuation: &Vec3, scattered: &Ray);
}
