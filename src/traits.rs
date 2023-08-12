use crate::{world::*, app::SDL};

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