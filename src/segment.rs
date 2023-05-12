use crate::{block::Block, cell::Cell, composition::Physical};

pub enum Segment {
    Air(Physical),
    Dirt(Block),
    Cell(Cell)
}