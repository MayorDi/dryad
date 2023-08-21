use crate::{
    constants::world::COUNT_SEGMENTS,
    traits::Behaviour,
    world::{Block, Segment, World, Cell},
};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..COUNT_SEGMENTS {
            match &world_read.segments[i] {
                Segment::Dirt(_) => {
                    Block::update(world_read, &mut self.world, i);
                }

                Segment::Cell(_) => {
                    Cell::update(world_read, &mut self.world, i);
                }

                _ => {}
            }
        }
    }
}
