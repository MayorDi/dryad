use nalgebra::Vector2;

use crate::{constants::colors::*, traits::Behaviour};

use super::*;

/// `Block` represents the solid base of the grid, mainly acts as the soil.
/// It supplies plants with nutrients.
#[derive(Debug, Default, Clone, Copy,PartialEq)]
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
    fn update(&mut self, world_read: &World, world: &mut World) {
        let neighbors = get_idx_neighbors(self);
        let wtr = 50.0;

        for i in 0..neighbors.len() {
            if let Segment::Dirt(neighbor) = &world_read.segments[neighbors[i]] {
                if is_needs_water(&self, neighbor){
                    let cof = self.chemical.water / 500.0;

                    if let Segment::Dirt(neighbor) = &mut world.segments[neighbors[i]] {
                        self.chemical.water -= wtr * cof;
                        neighbor.chemical.water += wtr * cof;
                    }
                }
            }
        }

        self.physical.color = COLOR_DIRT * (1.0 - self.chemical.water / 500.0);
    }
}

pub(self) fn is_needs_water(block: &Block, neighbor: &Block) -> bool {
    block.chemical.water > neighbor.chemical.water && 
    neighbor.chemical.water < 400.0
}