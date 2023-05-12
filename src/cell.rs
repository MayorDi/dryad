use nalgebra::Vector2;

use crate::{composition::*, genome::Genome};

/// `Cell` основная рабочая единица в которой и происходят все процессы.
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

/// `TypeCell` отвечает за определение типа для `Cell`.<br>
/// * `Photosynthetic` - отвечает за фотосинтез, а следовательно и за создание энергии.
/// * `Conductor` - является хорошим транспортёром питательных веществ.
/// * `Builder` - тип, что производит создание новых клеток. 
/// * `Producer` - тип, что создаёт новую особь.
/// * `Consumer` - тип, что выступает в качестве корней, 
/// единственный тип, обеспечивающий обмен между почвой и остальным организмом.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum TypeCell {
    Photosynthetic,
    Conductor,
    #[default]
    Builder,
    Producer,
    Consumer,
}

impl Default for Cell {
    fn default() -> Self {
        todo!()
    }
}