//! `composition` - the module stores the general features of the segments.

use crate::color::Color;

/// `Chemical` defines the basic chemical composition of a segment of the world. <br>
/// There are no specific units of calculus in the fields.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Chemical {
    pub water: f32,
    pub metals: f32,
    
// ======== organic ========

    pub glucose: f32,
    pub nitrates: f32,
    pub nitrites: f32,
}

/// `Physical` defines the physical features of a segment of the world.
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