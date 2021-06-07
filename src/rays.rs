// rustrays
use super::vectors::*;
use Vector3 as Point3;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t*self.direction
    }
}