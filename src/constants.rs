pub mod colors {
    pub const BACKGROUND: [f32; 4] = [0x0e as f32 / 255.0, 0x12 as f32 / 255.0, 0x15 as f32 / 255.0, 1.0];
    pub const SKYBLUE: (f32, f32, f32) = (0x88 as f32 / 255.0, 0xa4 as f32 / 255.0, 0xff as f32 / 255.0);
}

pub mod world {
    pub const SIZE_WORLD: [usize; 2] = [256, 32];
    pub const COUNT_SEGMENTS: usize = SIZE_WORLD[0] * SIZE_WORLD[1];
    pub const WATER_EXCHANGE_COEFFICIENT: f32 = 0.2;
    pub const NITROGEN_EXCHANGE_COEFFICIENT: f32 = 0.1;
}

pub mod genome {
    pub const COUNT_GENES: usize = 40;
}