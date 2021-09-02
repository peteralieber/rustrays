// rustrays
use super::vectors::*;
use Vector3 as Point3;

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 { // had to use &self to make it an unmutable reference to self, so this could be called by borrowers of *self
        self.origin + t*self.direction
    }
}