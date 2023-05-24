use nalgebra::Vector2;
use crate::constants::colors::COLOR_DIRT;

use super::{Chemical, Physical, Behaviour, Position, get_idx_neighbors, World};

/// `Block` представляет твёрдую основу сетки, в основном выступают в роли почвы.
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
    fn update(&mut self, world: &mut World) {
        use super::WATER_EXCHANGE_COEFFICIENT as WE_COEF;
        use crate::world::Segment;

        let (
            (_x, _y), 
            (_width, _height), 
            _idx, neighbors
        ) = get_idx_neighbors(self);

        for neighbor in neighbors.iter() {
            if let Some(neighbor) = neighbor {
                match &mut world.segments[*neighbor] {
                    Segment::Dirt(block) => {
                        let water = &mut self.chemical.water;
                        let water_n = &mut block.chemical.water;

                        if *water > *water_n {
                            let wtr = *water * WE_COEF;

                            *water -= wtr;
                            *water_n += wtr;
                        }
                    }
                    _ => {}
                }
            }
        }

        self.physical.color = COLOR_DIRT * (1.0 - self.chemical.water / 500.0);

    }
}

