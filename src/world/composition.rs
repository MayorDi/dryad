//! `composition` - модуль хранит основное описания составных особенностей сегментов мира.

use crate::color::Color;

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

impl Default for Physical {
    fn default() -> Self {
        Self {
            light: 1.0,
            color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
        }
    }
}