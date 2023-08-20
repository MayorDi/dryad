use nalgebra::Vector2;

use crate::traits::Behaviour;

use super::*;

/// `Block` represents the solid base of the grid, mainly acts as the soil.
/// It supplies plants with nutrients.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block {
    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,
}

impl Position for Block {
    fn get_position(&self) -> VectorWrapper<usize> {
        VectorWrapper(self.position)
    }
}

impl Behaviour for Block {
    fn update(&mut self, _world_read: &World, _world: &mut World) {}
}

pub(self) fn _is_needs_water(block: &Block, neighbor: &Block) -> bool {
    (block.chemical.water > neighbor.chemical.water) && (neighbor.chemical.water < 400.0)
}
