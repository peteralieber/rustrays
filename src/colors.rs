// Color Type

use std::ops::*;
use super::vectors::*;

pub const DYN_RANGE: i32 = 256;
const MAX_VAL: f32 = DYN_RANGE as f32 - 0.001;

#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", (self.r.clamp(0.0, 0.999)*MAX_VAL) as u32, 
                                (self.g.clamp(0.0, 0.999)*MAX_VAL) as u32, 
                                (self.b.clamp(0.0, 0.999)*MAX_VAL) as u32)
    }
}

impl Color {
    pub fn new(r:f32,g:f32,b:f32) -> Self{
        Self {
            r,
            g,
            b,
        }
    }

    pub fn from_vector(v: Vector3) -> Self {
        Self {
            r:v.x,
            g:v.y,
            b:v.z,
        }
    }

    pub fn gamma_correct(&mut self) {
        self.r = self.r.sqrt();
        self.g = self.g.sqrt();
        self.b = self.b.sqrt();
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b
        }
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, other: f32) {
        self.r = self.r * other;
        self.g = self.g * other;
        self.b = self.b * other;
    }
}

impl Div for Color {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Color {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b
        }
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other
        }
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, other: f32) {
        self.r = self.r / other;
        self.g = self.g / other;
        self.b = self.b / other;
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, other: Self) {
        *self = Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        };
    }
}