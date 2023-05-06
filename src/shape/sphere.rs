use crate::{vec3::Vector3, Scalar};

use super::hittable::Hittable;

pub struct Sphere {
    pub center: Vector3,
    pub radius: Scalar,
}

impl Hittable for Sphere {
    fn hit(
        self: &Self,
        ray: &crate::ray::Ray,
        t_min: Scalar,
        t_max: Scalar,
        rec: &mut super::hittable::HitRecord,
    ) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        };

        let sqrtd = discriminant.sqrt();

        /* Find the nearest root that lies in the acceptable range */
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(&ray, &outward_normal);

        true
    }
}

impl Sphere {
    pub fn new(center: Vector3, radius: Scalar) -> Self {
        Self { center, radius }
    }
}
