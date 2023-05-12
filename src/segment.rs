use crate::{block::Block, life::Cell};

pub enum Segment {
    Air,
    Dirt(Block),
    Cell(Cell)
}