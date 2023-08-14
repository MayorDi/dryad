pub mod app {
    pub const FPS: u32 = 60;
}

pub mod colors {
    use crate::color::Color;

    pub const BACKGROUND: Color = Color {
        r: 0x0e as f32 / 255.0,
        g: 0x12 as f32 / 255.0,
        b: 0x15 as f32 / 255.0,
        a: 1.0,
    };
    pub const COLOR_SKYBLUE: Color = Color {
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
    pub const COLOR_BUILDER: Color = Color {
        r: 0x54 as f32 / 255.0,
        g: 0x92 as f32 / 255.0,
        b: 0x48 as f32 / 255.0,
        a: 1.0,
    };
    pub const COLOR_CONDUCTOR: Color = Color {
        r: 0x80 as f32 / 255.0,
        g: 0x5d as f32 / 255.0,
        b: 0x55 as f32 / 255.0,
        a: 1.0,
    };
    pub const COLOR_CONSUMER: Color = Color {
        r: 0xd4 as f32 / 255.0,
        g: 0x80 as f32 / 255.0,
        b: 0x6b as f32 / 255.0,
        a: 1.0,
    };
    pub const COLOR_PHOTOSYNTHETIC: Color = Color {
        r: 0x80 as f32 / 255.0,
        g: 0xb8 as f32 / 255.0,
        b: 0x63 as f32 / 255.0,
        a: 1.0,
    };
    pub const COLOR_PRODUCER: Color = Color {
        r: 0xed as f32 / 255.0,
        g: 0xf2 as f32 / 255.0,
        b: 0xa8 as f32 / 255.0,
        a: 1.0,
    };
}

pub mod world {
    pub const SIZE_WORLD: [usize; 2] = [256, 128];
    pub const COUNT_SEGMENTS: usize = SIZE_WORLD[0] * SIZE_WORLD[1];
    pub const WATER_EXCHANGE_COEFFICIENT: f32 = 0.2;
    pub const NITROGEN_EXCHANGE_COEFFICIENT: f32 = 0.1;
}

pub mod cell {
    pub const MAX_LIFE_TIME: usize = 40;
}

pub mod genome {
    pub const COUNT_GENES: usize = 20;
}
