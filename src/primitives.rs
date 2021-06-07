// Primitives
use super::vectors::*;
use super::rays::*;

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub hittables : Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn clear(&mut self) {
        self.hittables.clear();
    }

    pub fn add(&mut self, p: Box<dyn Hittable>) { // Had to use Box<> because trait is dynamic and therefore size isn't known at compile time
        self.hittables.push(p);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        //Was able to remove "hit_anything" because that logic is encapsulated in the use of Option<>, yay Rust!
        let mut closest_so_far = t_max;

        for hittable in &self.hittables {
            match hittable.hit(r, t_min, closest_so_far) {
                None => (),
                Some(hit_record) => {
                    closest_so_far = hit_record.t;
                    temp_rec = Some(hit_record);
                }
            }
        }
        temp_rec
    }
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
        let outward_normal = (p - self.center)/self.radius;
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        Some(HitRecord {t:root, p:p, normal:normal, front_face: false})
    }
}