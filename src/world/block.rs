use nalgebra::Vector2;

use crate::constants::colors::*;

use super::{Behaviour, Chemical, Physical, Position, World, Segment, get_idx_neighbors};

/// `Block` represents the solid base of the grid, mainly acts as the soil.
/// It supplies plants with nutrients.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block {
    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,
}

impl Position for Block {
    fn get_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl Behaviour for Block {
    fn update(&mut self, world_read: &World, world: &mut World, idx: usize) {
        let neighbors = get_idx_neighbors(&world_read.segments[idx]);
        let wtr = 50.0;

        for j in 0..4 {
            if let Segment::Dirt(neighbor) = &world_read.segments[neighbors[j]] {
                if self.chemical.water > neighbor.chemical.water
                    && 400.0 > world.segments[neighbors[j]].to_block().chemical.water
                {
                    let cof = self.chemical.water / 500.0;

                    self.chemical.water -= wtr * cof;
                    world.segments[neighbors[j]].to_block().chemical.water += wtr * cof;
                }
            }
        }

        self.physical.color = COLOR_DIRT * (1.0 - self.chemical.water / 500.0);
    }
}
