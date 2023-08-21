use crate::{
    traits::Behaviour,
    world::{Block, Cell, Segment, World},
};

use super::App;

impl App {
    pub fn update(&mut self, world_read: &World) {
        for i in 0..world_read.segments.len() {
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
