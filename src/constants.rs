pub mod colors {
    use crate::color::Color;

    pub const BACKGROUND: Color = Color {
        r: 0x0e as f32 / 255.0, 
        g: 0x12 as f32 / 255.0, 
        b: 0x15 as f32 / 255.0, 
        a: 1.0,
    };
    pub const SKYBLUE: Color = Color {
        r: 0x88 as f32 / 255.0, 
        g: 0xa4 as f32 / 255.0, 
        b: 0xff as f32 / 255.0,
        a: 1.0,
    };
    pub const COLOR_DIRT: Color = Color {
        r: 0x8d as f32 / 255.0, 
        g: 0x64 as f32 / 255.0, 
        b: 0x5a as f32 / 255.0, 
        a: 1.0,
    };
}

pub mod world {
    pub const SIZE_WORLD: [usize; 2] = [256, 64];
    pub const COUNT_SEGMENTS: usize = SIZE_WORLD[0] * SIZE_WORLD[1];
    pub const WATER_EXCHANGE_COEFFICIENT: f32 = 0.2;
    pub const NITROGEN_EXCHANGE_COEFFICIENT: f32 = 0.1;
}

pub mod genome {
    pub const COUNT_GENES: usize = 40;
}