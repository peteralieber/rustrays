// Cameras
use super::vectors::*;
use super::rays::*;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0,0.0,focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}