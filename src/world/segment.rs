use super::{Block, Cell, Position, Air };

/// `Segment` является основой построения мира, осуществляя разделение сегментов на типы.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Air(Air),
    Dirt(Block),
    Cell(Cell)
}

impl Segment {
    /// Используется когда точно известно, что за объект. <br>
    /// Нужен, когда есть проблема с множеством `mut` ссылок.
    pub fn to_block(&mut self) -> &mut Block {
        if let Segment::Dirt(block) = self {
            return block;
        }

        panic!("Error: unable to convert block.");
    }

    /// Используется когда точно известно, что за объект. <br>
    /// Нужен, когда есть проблема с множеством `mut` ссылок.
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