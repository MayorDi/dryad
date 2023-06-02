use crate::world::{World, Segment, Behaviour};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &mut World, idx: usize) {
        match world_read.segments[idx] {
            Segment::Cell(_) => {
                let mut cell = self.world.segments[idx].to_cell().clone();
                cell.update(world_read, &mut self.world, idx);

                self.world.segments[idx] = Segment::Cell(cell);
            }

            Segment::Dirt(_) => {
                let mut block = self.world.segments[idx].to_block().clone();
                block.update(world_read, &mut self.world, idx);

                self.world.segments[idx] = Segment::Dirt(block);
            }
            
            _ => {}
        }
    }
}