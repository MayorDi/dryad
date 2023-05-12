use nalgebra::Vector2;

use crate::composition::*;

pub struct Cell {
    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,

    pub type_cell: TypeCell,
}

pub enum TypeCell {
    PhotosyntheticTissue,
    ConductingTissue
}