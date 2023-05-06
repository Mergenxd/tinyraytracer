use crate::{ray::Ray, vec3::Vector3, Scalar};

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: Scalar,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(self: &Self, ray: &Ray, t_min: Scalar, t_max: Scalar, rec: &mut HitRecord) -> bool;
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Vector3::new(0.0),
            normal: Vector3::new(0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(self: &mut Self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -(*outward_normal)
        };
    }
}
