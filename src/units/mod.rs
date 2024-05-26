use crate::chemical_composition::Chemistry;

use self::cells::Cell;

pub mod cells;

#[derive(Debug, Clone)]
pub enum Unit {
    Air,
    Cell(Cell),
    Dirt,
    Water,
}

#[derive(Debug, Clone)]
pub struct Water {
    chemical_composition: Chemistry
}

impl Water {
    pub fn new() -> Self {
        Self {
            chemical_composition: Chemistry {
                water: 1000.0
            }
        }
    }
}
