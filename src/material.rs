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
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
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
        *scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_unit_vector(),
        );
        *attenuation = self.albedo;
        true
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);

        let mut outward_normal = Vec3::default();
        let mut reflected = ray.direction.reflect(&hit_record.normal);
        let mut index_ratio = 0.0;
        let mut refracted = Vec3::default();

        if Vec3::dot(&ray.direction, &hit_record.normal) > 0.0 {
            outward_normal = hit_record.normal * -1.0;
            index_ratio = self.refractive_index;
        } else {
            outward_normal = hit_record.normal;
            index_ratio = 1.0 / self.refractive_index;
        }
        true
    }
    fn refract(direction: &Vec3, normal: &Vec3, index_ratio: f64, refracted: &mut Vec3) -> {
        let unit_direction = direction.normalized();
        
    }
}
