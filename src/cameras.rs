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
    u: Vector3,
    v: Vector3,
    w: Vector3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vector3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);
        
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (horizontal/2.0) - (vertical/2.0) - (focus_dist*w);

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = (self.u * rd.x) + (self.v * rd.y);

        Ray{
            origin: self.origin + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Point3::new(0.0,0.0,0.0), Point3::new(0.0,0.0,-1.0), Point3::new(0.0,1.0,0.0), 90.0, 16.0/9.0, 1.0, 1.0)
    }
}

fn random_in_unit_disk() -> Point3 {
    let mut p = Vector3::new(rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0);
    while p.length_squared() >= 1.0 {
        p = Vector3::new(rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0);
    };
    p
}