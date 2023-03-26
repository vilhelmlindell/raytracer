use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn is_hit(&self, ray: &Ray, t_min: &f64, t_max: &f64) -> Option<HitRecord> {
        let mut hit_record = Option::<HitRecord>::None;
        let mut closest_hit = *t_max;

        for object in self.objects.iter() {
            if let Some(new_hit_record) = object.is_hit(ray, t_min, &closest_hit) {
                closest_hit = new_hit_record.t;
                hit_record = Some(new_hit_record);
            }
        }

        hit_record
    }
}
