use crate::world::{World, Segment::*, Behaviour};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..self.world.segments.len() {
            if let Cell(cell) = &self.world.segments[i] {
                cell.clone().update(world_read, &mut self.world);
                
            } else if let Dirt(block) = &self.world.segments[i] {
                let mut block = block.clone();
                block.update(world_read, &mut self.world);
    
                self.world.segments[i] = Dirt(block);
            }
        };
    }
}