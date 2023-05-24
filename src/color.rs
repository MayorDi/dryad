pub use crate::constants::colors::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl std::ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut color = self;

        color.r *= rhs;
        color.g *= rhs;
        color.b *= rhs;

        color
    }
}

impl From<[f32; 4]> for Color {
    fn from(value: [f32; 4]) -> Self {
        Self {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3]
        }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }

    pub fn from_byte(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            r: red as f32 / 255.0,
            g: green as f32 / 255.0,
            b: blue as f32 / 255.0,
            a: alpha as f32 / 255.0,
        }
    }

    pub fn to_bytes(&self) -> (u8, u8, u8, u8) {
        ((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8, (self.a * 255.0) as u8)
    }
}