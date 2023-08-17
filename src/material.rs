use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
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
    fn scatter(
        &self,
        _ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray.direction.normalized().reflect(&hit_record.normal);
        *scattered = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;
        true
    }
}
