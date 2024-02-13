use crate::alias::Position;
use macros_helper::GetPosition;

use crate::traits::{Behaviour, ToBlock};

use super::*;

/// `Block` represents the solid base of the grid, mainly acts as the soil.
/// It supplies plants with nutrients.
#[derive(Debug, Default, Clone, PartialEq, GetPosition)]
pub struct Block {
    pub position: Position,
    pub chemical: Chemical,
    pub physical: Physical,
}

impl Behaviour for Block {
    fn update(world_read: &World, world: &mut World, idx_segment: usize) {
        let neighbors = get_idx_neighbors(world.segments[idx_segment].to_block().unwrap());
        let wtr = 6.0;

        for i in 0..neighbors.len() {
            if let Segment::Dirt(neighbor) = &world_read.segments[neighbors[i]] {
                if neighbors[i] == idx_segment {
                    continue;
                }

                let block = world.segments[idx_segment].to_block().unwrap();
                if is_needs_water(&block, neighbor) {
                    if let Segment::Dirt(_) = &world_read.segments[neighbors[i]] {
                        block.chemical.water -= wtr;
                        world.segments[neighbors[i]]
                            .to_block()
                            .unwrap()
                            .chemical
                            .water += wtr;
                    }
                }
            }
        }

        let block = world.segments[idx_segment].to_block().unwrap();
        block.physical.color = COLOR_DIRT * (1.0 - block.chemical.water / 500.0);
    }
}

pub(self) fn is_needs_water(block: &Block, neighbor: &Block) -> bool {
    (block.chemical.water > neighbor.chemical.water) && (neighbor.chemical.water < 400.0)
}
