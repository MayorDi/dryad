use crate::alias::Position;
use crate::traits::*;

use super::{Air, Block, Cell};

/// `Segment` is the basis for building the world by dividing segments into types.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Air(Air),
    Dirt(Block),
    Cell(Cell),
}

impl ToAir for Segment {
    fn to_air(&mut self) -> Result<&mut Air, ()> {
        if let Segment::Air(air) = self {
            return Ok(air);
        }

        Err(())
    }
}

impl ToBlock for Segment {
    fn to_block(&mut self) -> Result<&mut Block, ()> {
        if let Segment::Dirt(block) = self {
            return Ok(block);
        }

        Err(())
    }
}

impl ToCell for Segment {
    fn to_cell(&mut self) -> Result<&mut Cell, ()> {
        if let Segment::Cell(cell) = self {
            return Ok(cell);
        }

        Err(())
    }
}

impl GetPosition for Segment {
    fn get_position(&self) -> Position {
        match self {
            Segment::Cell(cell) => cell.get_position(),
            Segment::Dirt(block) => block.get_position(),
            Segment::Air(air) => air.get_position(),
        }
    }

    fn get_index(&self) -> usize {
        match self {
            Segment::Cell(cell) => cell.get_index(),
            Segment::Dirt(block) => block.get_index(),
            Segment::Air(air) => air.get_index(),
        }
    }
}
