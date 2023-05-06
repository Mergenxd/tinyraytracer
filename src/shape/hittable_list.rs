use crate::Scalar;

use super::hittable::{HitRecord, Hittable};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(
        self: &Self,
        ray: &crate::ray::Ray,
        t_min: Scalar,
        t_max: Scalar,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(&ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(self: &mut Self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(self: &mut Self) {
        self.objects.clear();
    }
}
