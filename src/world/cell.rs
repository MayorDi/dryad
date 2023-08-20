use nalgebra::Vector2;
use rand::Rng;

use crate::traits::{Behaviour, Glucose, Mutation};

use super::*;

/// `Cell` is the main working unit in which most of all processes take place.
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub id: usize,
    pub lifetime: usize,

    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,

    pub type_cell: TypeCell,
    pub children: [usize; 5],

    pub step: usize,
    pub genome: Genome,
}

/// `Type Cell` is responsible for determining the type for `Cell`: <br>
/// * `Photosynthetic` is responsible for photosynthesis, and therefore for the creation of energy.
/// * `Conductor` is a good nutrient transporter.
/// * `Builder` is the type that creates new cells.
/// * `Producer` is the type that creates a new cell.
/// * `Consumer` is the type that acts as roots,
/// the only type that provides an exchange between the soil and the rest of the organism.
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
            water: 100000,
            glucose: 300000,
            metals: 1000,
            nitrates: 10000,
            nitrites: 1000,
        };

        let mut physical = Physical::default();
        match type_cell {
            TypeCell::Builder => physical.light = 0.5,
            TypeCell::Conductor => physical.light = 0.2,
            TypeCell::Consumer => physical.light = 0.3,
            TypeCell::Photosynthetic => physical.light = 0.8,
            TypeCell::Producer => physical.light = 0.1,
        }

        Self {
            id,
            lifetime: 0,
            position,
            chemical,
            physical,
            type_cell,
            children: [1, 0, 0, 0, 0],
            step: 1,
            genome: Genome::default(),
        }
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
            children: [0; 5],
            step: 0,
            genome: Genome([Gene::default(); COUNT_GENES]),
        }
    }
}

impl Glucose for Cell {
    fn synthesize_glucose(&mut self, _light: f32) {}

    fn get_glucose(&self) -> u32 {
        self.chemical.glucose
    }

    fn set_glucose(&mut self, value: u32) {
        self.chemical.glucose = value;
    }
}

impl Mutation for Cell {
    fn mutate(&mut self) {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.05 {
            for gene in self.genome.0.iter_mut() {
                match rand::thread_rng().gen_range(0..10) {
                    0 => gene.type_cell = TypeCell::Consumer,
                    1 => gene.type_cell = TypeCell::Builder,
                    2 => gene.type_cell = TypeCell::Producer,
                    3 => gene.type_cell = TypeCell::Photosynthetic,
                    4 => gene.type_cell = TypeCell::Conductor,
                    _ => {}
                }

                gene.children = [
                    rand::thread_rng().gen_range(0..COUNT_GENES),
                    rand::thread_rng().gen_range(0..COUNT_GENES),
                    rand::thread_rng().gen_range(0..COUNT_GENES),
                    rand::thread_rng().gen_range(0..COUNT_GENES),
                    rand::thread_rng().gen_range(0..COUNT_GENES),
                ];
            }
        }
    }
}

impl Position for Cell {
    fn get_position(&self) -> VectorWrapper<usize> {
        VectorWrapper(self.position)
    }
}

impl Behaviour for Cell {
    fn update(&mut self, _world_read: &World, _world: &mut World) {}
}
