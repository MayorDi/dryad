use nalgebra::Vector2;

use crate::traits::{Behaviour, ToBlock};

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
    fn update(world_read: &World, world: &mut World, idx_segment: usize) {
        let neighbors = get_idx_neighbors(world.segments[idx_segment].to_block().unwrap());
        let wtr = 6.0;

        for i in 0..neighbors.len() {
            if let Segment::Dirt(neighbor) = &world_read.segments[neighbors[i]] {
                if neighbors[i] == idx_segment { continue; }

                let this = world.segments[idx_segment].to_block().unwrap();
                if is_needs_water(&this, neighbor) {
                    if let Segment::Dirt(_) = &world_read.segments[neighbors[i]] {
                        this.chemical.water -= wtr;
                        world.segments[neighbors[i]]
                            .to_block()
                            .unwrap()
                            .chemical
                            .water += wtr;
                    }
                }
            }
        }

        let this = world.segments[idx_segment].to_block().unwrap();
        this.physical.color = COLOR_DIRT * (1.0 - this.chemical.water / 500.0);
    }
}

pub(self) fn is_needs_water(block: &Block, neighbor: &Block) -> bool {
    (block.chemical.water > neighbor.chemical.water) && (neighbor.chemical.water < 400.0)
}
