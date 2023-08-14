use crate::{
    traits::Behaviour,
    world::{Segment::*, World},
};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..self.world.segments.len() {
            if let Cell(mut cell) = self.world.segments[i] {
                cell.update(world_read, &mut self.world);

                self.world.segments[i] = Cell(cell);
            } else if let Dirt(mut block) = self.world.segments[i] {
                block.update(world_read, &mut self.world);

                self.world.segments[i] = Dirt(block);
            }
        }
    }
}
