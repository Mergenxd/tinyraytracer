use crate::{vec3::Vector3, Scalar};

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self: &Self, t: Scalar) -> Vector3 {
        self.origin + self.direction * t
    }
}
