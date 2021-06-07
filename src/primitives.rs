// Primitives
use super::vectors::*;
use super::rays::*;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f32
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f32
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let nroot = (-half_b + sqrtd) / a;
            if nroot < t_min || t_max < nroot {
                return None;
            }
        }

        let p = r.at(root);
        Some(HitRecord {t:root, p:p, normal: (p - self.center)/self.radius})
    }
}