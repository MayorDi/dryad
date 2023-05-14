//! `composition` - модуль хранит основное описания составных особенностей сегментов мира.

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

/// `Chemical` определяет основной химический состав сегмента мира. <br>
/// В полях нет определённых единиц исчисления.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Chemical {
    pub water: f32,
    pub metals: f32,
    
// ======== organic ========

    pub glucose: f32,
    pub nitrates: f32,
    pub nitrites: f32,
}

/// `Physical` определяет физические особенности сегмента мира.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Physical {
    pub light: f32,
    pub color: Color
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
}

impl Default for Physical {
    fn default() -> Self {
        Self {
            light: 1.0,
            color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
        }
    }
}