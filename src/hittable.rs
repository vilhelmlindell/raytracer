use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable {
    fn is_hit(&self, ray: &Ray, t_min: &f64, t_max: &f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.0
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            point: Vec3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        }
    }
}
