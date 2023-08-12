use crate::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Physical {
    pub light: f32,
    pub color: Color
}

impl Default for Physical {
    fn default() -> Self {
        Self {
            light: 1.0,
            color: Color::default()
        }
    }
}