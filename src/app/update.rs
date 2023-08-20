use crate::{
    traits::{Behaviour, Position, ToCell},
    world::{Segment::{*, self}, World, TypeCell, get_index}, constants::world::SIZE_WORLD,
};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..self.world.segments.len() {
            match &mut self.world.segments[i] {
                Cell(_) => {
                    
                }

                Dirt(mut block) => {
                    block.update(world_read, &mut self.world);

                    self.world.segments[i] = Dirt(block);
                }

                _ => {}
            }
        }
    }
}
