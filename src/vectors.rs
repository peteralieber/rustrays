// Vector Type
use std::ops::*;

pub trait Length {
    fn length(&self) -> f32;
    fn length_squared(&self) -> f32;
}

#[derive(Copy, Clone)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl std::fmt::Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Length for Point3 {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Point3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Mul for Point3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Point3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Point3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl Mul<Point3> for f32 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Div for Point3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Point3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Point3 {
    pub fn dot(self, other: Self) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Point3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}