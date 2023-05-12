#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chemical {
    pub water: f32,
    pub nitrates: f32,
    pub nitrites: f32,
    pub metals: f32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Physical {
    pub light: u8,
    pub color: Color
}

impl Default for Chemical {
    fn default() -> Self {
        Self {
            water: 250.0,      
            nitrates: 60.0,    
            nitrites: 10.0,    
            metals: 200.0       
        }
    }
}

impl Default for Physical {
    fn default() -> Self {
        Self {
            light: 0,
            color: Color { r: 146, g: 105, b: 95 }
        }
    }
}