use crate::traits::*;

use super::{Air, Block, Cell, VectorWrapper};

/// `Segment` is the basis for building the world by dividing segments into types.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Air(Air),
    Dirt(Block),
    Cell(Cell),
}

impl ToAir for Segment {
    fn to_air(self) -> Result<Air, ()> {
        if let Segment::Air(air) = self {
            return Ok(air);
        }

        Err(())
    }
}

impl ToBlock for Segment {
    fn to_block(self) -> Result<Block, ()> {
        if let Segment::Dirt(block) = self {
            return Ok(block);
        }

        Err(())
    }
}

impl ToCell for Segment {
    fn to_cell(self) -> Result<Cell, ()> {
        if let Segment::Cell(cell) = self {
            return Ok(cell);
        }

        Err(())
    }
}

impl Position for Segment {
    fn get_position(&self) -> VectorWrapper<usize> {
        match self {
            Segment::Cell(cell) => cell.get_position(),
            Segment::Dirt(block) => block.get_position(),
            Segment::Air(air) => air.get_position(),
        }
    }
}
