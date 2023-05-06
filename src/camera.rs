use crate::{ray::Ray, vec3::Vector3, Scalar};

pub const ASPECT_RATIO: Scalar = 16.0 / 9.0;

#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Vector3,
    pub lower_left_corner: Vector3,
    pub horizontal: Vector3,
    pub vertical: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        let viewport_height: Scalar = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;
        let focal_length: Scalar = 1.0;

        let origin = Vector3::new(0.0);
        let horizontal = Vector3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vector3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - (horizontal / 2.0)
            - (vertical / 2.0)
            - Vector3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(self: &Self, u: Scalar, v: Scalar) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (self.horizontal * u) + (self.vertical * v)
                - self.origin,
        }
    }
}
