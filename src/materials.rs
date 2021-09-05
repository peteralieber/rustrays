// Materials

use super::primitives::*;
use super::rays::*;
use super::colors::*;
use super::util::*;

pub enum Material {
    Diffuse {
        albedo: Color,
    },
    Metal {
        albedo: Color,
        fuzz: f32,
    },
    Dialectric {
        albedo: Color,
        index_of_refraction: f32,
    }
}

impl Material {
    pub fn new(albedo: Color) -> Self {
        Material::Diffuse {
            albedo,
        }
    }

    pub fn scatter(&self, r: &Ray, rec: &HitRecord) -> (Color, Ray) {
        match self {
            Self::Diffuse { albedo } => {
                let mut scatter_dir = rand_lamb_vector(rec);
                if scatter_dir.near_zero() {
                    scatter_dir = rec.normal;
                };
                let scattered = Ray { origin: rec.p, direction: scatter_dir};
                (*albedo, scattered)
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = r.direction.unit_vector().reflect(rec.normal);
                let scattered = Ray{origin: rec.p, direction: reflected + *fuzz*rand_in_unit_sphere()};
                (*albedo, scattered)
            }
            Self::Dialectric { albedo, index_of_refraction } => {
                let refraction_ratio = if rec.front_face {1.0/index_of_refraction} else {*index_of_refraction};
                let unit_direction = r.direction.unit_vector();
                let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
                let cannot_refract = ( refraction_ratio * sin_theta ) > 1.0;
                if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > rand() {
                    let reflected = unit_direction.reflect(rec.normal);
                    let scattered = Ray{origin: rec.p, direction: reflected};
                    (*albedo, scattered)
                }
                else {
                    let refracted = unit_direction.refract(rec.normal, refraction_ratio);
                    let scattered = Ray{origin: rec.p, direction: refracted};
                    (*albedo, scattered)
                }
            }
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0r0 = r0*r0;
        r0r0 + (1.0 - r0r0)*((1.0 - cosine).powi(5))
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Diffuse {
            albedo: Color::new(0.5, 0.5, 0.5)
        }
    }
}