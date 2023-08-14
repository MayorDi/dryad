use crate::{app::SDL, world::*};

pub trait ToBlock {
    fn to_block(self) -> Result<Block, ()>;
}

pub trait ToCell {
    fn to_cell(self) -> Result<Cell, ()>;
}

pub trait ToAir {
    fn to_air(self) -> Result<Air, ()>;
}

/// Trait for simple behavior.
pub trait Behaviour {
    fn update(&mut self, world_read: &World, world: &mut World);
}

pub trait Render {
    fn render(&self, sdl: &mut SDL);
}

pub trait Position {
    fn get_position(&self) -> VectorWrapper<usize>;
    fn get_index(&self) -> usize {
        let (x, y) = self.get_position().into();
        get_index(x, y, SIZE_WORLD[0])
    }
}

pub trait Mutation {
    fn mutate(&mut self);
}

pub trait Glucose {
    fn synthesize_glucose(&mut self, light: f32);
    fn get_glucose(&self) -> f32;
    fn set_glucose(&mut self, value: f32);
    fn glucose_to_energy(&mut self) {}
}