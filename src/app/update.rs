use crate::world::{World, Segment::*, Behaviour};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for idx in 0..self.world.segments.len() {
            if let Cell(cell) = &self.world.segments[idx] {
                let mut cell = cell.clone();
                cell.update(world_read, &mut self.world);
    
                self.world.segments[idx] = Cell(cell);
                
            } else if let Dirt(block) = &self.world.segments[idx] {
                let mut block = block.clone();
                block.update(world_read, &mut self.world);
    
                self.world.segments[idx] = Dirt(block);
            }
        }
    }
}