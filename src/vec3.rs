use crate::*;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl Vector3 {
    pub fn new(scalar: Scalar) -> Vector3 {
        Vector3 {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    pub fn length(self: &Self) -> Scalar {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self: &Self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(self: Self, v: &Self) -> Scalar {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self: Self, v: &Self) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn unit_vector(self: &Self) -> Vector3 {
        *self / self.length()
    }

    pub fn random(min: Scalar, max: Scalar) -> Vector3 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Vector3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        let mut point = Self::random(-1.0, 1.0);
        while point.length_squared() >= 1.0 {
            point = Self::random(-1.0, 1.0);
        }

        point
    }

    pub fn random_unit_vector() -> Vector3 {
        Vector3::unit_vector(&Vector3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = Vector3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Scalar> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Scalar) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<Scalar> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Scalar) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<Scalar> for Vector3 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<Scalar> for Vector3 {
    fn div_assign(&mut self, rhs: Scalar) {
        *self *= 1.0 / rhs;
    }
}
