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
    },
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
            Self::Metal { albedo } => {
                let reflected = r.direction.unit_vector().reflect(rec.normal);
                let scattered = Ray{origin: rec.p, direction: reflected};
                (*albedo, scattered)
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Diffuse {
            albedo: Color::new(0.5, 0.5, 0.5)
        }
    }
}