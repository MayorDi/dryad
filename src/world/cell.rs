use nalgebra::Vector2;
use rand::Rng;

use super::{Genome, Physical, Chemical, Gene, COUNT_GENES, Position};

/// `Cell` основная рабочая единица в которой и происходят все процессы.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub id: usize,
    pub lifetime: usize,

    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,

    pub type_cell: TypeCell,
    pub children: [usize; 4],

    pub step: usize,
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
    pub fn new(position: Vector2<usize>, type_cell: TypeCell, id: usize) -> Self {
        let chemical = Chemical {
            water: 100.0,
            glucose: 100.0,
            metals: 1.0,
            nitrates: 10.0,
            nitrites: 1.0
        };

        let mut physical = Physical::default();
        match type_cell {
            TypeCell::Builder => { 
                physical.light = 0.5;
            },
            TypeCell::Conductor => { 
                physical.light = 0.2;
            },
            TypeCell::Consumer => {
                physical.light = 0.3;
            },
            TypeCell::Photosynthetic => {
                physical.light = 0.8;
            },
            TypeCell::Producer => {
                physical.light = 0.1;
            }
        }

        Self {
            id,
            lifetime: 0,
            position,
            chemical,
            physical,
            type_cell,
            children: [1, 0, 0, 0],
            step: 1,
            genome: Genome::default()
        }
    }
}

impl Position for Cell {
    fn get_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            id: 0,
            lifetime: 0,
            position: Vector2::new(0, 0),
            chemical: Chemical::default(),
            physical: Physical::default(),
            type_cell: TypeCell::default(),
            children: [0; 4],
            step: 0,
            genome: Genome([Gene::default(); COUNT_GENES])
        }
    }
}