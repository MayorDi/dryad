use nalgebra::Vector2;

use crate::{composition::*, genome::Genome};

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub hash: md5::Digest,

    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,

    pub type_cell: TypeCell,
    pub children: [usize; 4],

    pub genome: Genome,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum TypeCell {
    Photosynthetic,
    Conductor,
    #[default]
    Builder,
    Producer,
}

impl Default for Cell {
    fn default() -> Self {
        todo!()
    }
}