use super::{Block, Cell, Position, Air };

/// `Segment` is the basis for building the world by dividing segments into types.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Air(Air),
    Dirt(Block),
    Cell(Cell)
}

impl Segment {
    /// It is used when it is known exactly what the object is. <br>
    /// It is needed when there is a problem with a lot of `mut` links.
    pub fn to_block(&mut self) -> &mut Block {
        if let Segment::Dirt(block) = self {
            return block;
        }

        panic!("Error: unable to convert block.");
    }

    /// It is used when it is known exactly what the object is. <br>
    /// It is needed when there is a problem with a lot of `mut` links.
    pub fn to_cell(&mut self) -> &mut Cell {
        if let Segment::Cell(cell) = self {
            return cell;
        }

        panic!("Error: unable to convert cell.");
    }
}

impl Position for Segment {
    fn get_position(&self) -> nalgebra::Vector2<usize> {
        match self {
            Segment::Cell(cell) => cell.get_position(),
            Segment::Dirt(block) => block.get_position(),
            Segment::Air(air) => air.get_position(),
        }
    }
}