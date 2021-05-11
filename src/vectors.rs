// Vector Type
use std::ops::*;

pub trait Length {
    fn length(&self) -> f32;
}

pub struct Point3(f32, f32, f32);

impl Length for Point3 {
    fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

impl Add for Point3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}