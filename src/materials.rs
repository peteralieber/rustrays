// Materials

use super::primitives::*;
use super::rays::*;
use super::colors::*;

pub enum Material {
    Diffuse {
        
    },
    Metal {

    },
}

impl Material {
    pub fn new() -> Self {
        Material::Diffuse {

        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material::Diffuse {

        }
    }
}