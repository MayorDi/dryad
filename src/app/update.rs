use crate::{
    traits::Behaviour,
    world::{Segment::*, World},
};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..self.world.segments.len() {
            match self.world.segments[i] {
                Cell(mut cell) => {
                    cell.update(world_read, &mut self.world);

                    self.world.segments[i] = Cell(cell);
                }

                Dirt(mut block) => {
                    block.update(world_read, &mut self.world);

                    self.world.segments[i] = Dirt(block);
                }

                Air(_) => {}
            }
        }
    }
}
