// Cameras
use super::vectors::*;
use super::rays::*;
use super::util::*;
use Vector3 as Point3;

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vector3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
        
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
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
        Self::new(Point3::new(0.0,0.0,0.0), Point3::new(0.0,0.0,-1.0), Point3::new(0.0,1.0,0.0), 90.0, 16.0/9.0)
    }
}