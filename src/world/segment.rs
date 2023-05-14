use super::{ Physical, Block, Cell };

/// `Segment` является основой построения мира, осуществляя разделение сегментов на типы.
#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Air(Physical),
    Dirt(Block),
    Cell(Cell)
}