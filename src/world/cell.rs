use nalgebra::Vector2;
use rand::Rng;

use super::{Genome, Physical, Chemical, Gene, COUNT_GENES, Color};

/// `Cell` основная рабочая единица в которой и происходят все процессы.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub id: usize,

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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TypeCell {
    Photosynthetic,
    Conductor,
    #[default]
    Builder,
    Producer,
    Consumer,
}

impl Cell {
    pub fn new(type_cell: TypeCell) -> Self {
        let mut physical = Physical::default();
        match type_cell {
            TypeCell::Builder => { 
                physical.color = Color::from_byte(0x80, 0xb8, 0x63, 255);
                physical.light = 0.5;
            },
            TypeCell::Conductor => { 
                physical.color = Color::from_byte(0x80, 0x5d, 0x55, 255);
                physical.light = 0.2;
            },
            TypeCell::Consumer => {
                physical.color = Color::from_byte(0xd4, 0x80, 0x6b, 255);
                physical.light = 0.3;
            },
            TypeCell::Photosynthetic => {
                physical.color = Color::from_byte(0x80, 0xb8, 0x63, 255);
                physical.light = 0.8;
            },
            TypeCell::Producer => {
                physical.color = Color::from_byte(0xed, 0xf2, 0xa8, 255);
                physical.light = 0.1;
            }
        }

        todo!()
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: rand::thread_rng().gen_range(0..1000000),

            position: Vector2::new(0, 0),
            chemical: Chemical::default(),
            physical: Physical::default(),
            type_cell: TypeCell::default(),
            children: [0; 4],
            genome: Genome([Gene::default(); COUNT_GENES])
        }
    }
}