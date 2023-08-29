use nalgebra::Vector2;
use rand::Rng;

use crate::traits::{Behaviour, Glucose, Mutation, ToCell};

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
            water: 100.0,
            glucose: 300.0,
            metals: 1.0,
            nitrates: 10.0,
            nitrites: 1.0,
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

    pub fn share(&self) -> [Option<Cell>; 5] {
        let genes: Vec<Option<Gene>> = self
            .children
            .iter()
            .map(|ch| {
                if *ch > 0 {
                    return Some(self.genome.0[*ch]);
                }

                None
            })
            .collect();

        let mut children: [Option<Cell>; 5] = [None, None, None, None, None];
        for (idx, gene) in genes.iter().enumerate() {
            match gene {
                Some(gene) => {
                    children[idx] = Some(Cell {
                        children: gene.children,
                        type_cell: gene.type_cell,
                        ..self.clone()
                    });
                }
                None => children[idx] = None,
            }
        }

        children
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

    fn get_glucose(&self) -> f32 {
        self.chemical.glucose
    }

    fn set_glucose(&mut self, value: f32) {
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
    fn update(world_read: &World, world: &mut World, idx_segment: usize) {
        let cell = world.segments[idx_segment].to_cell().unwrap();
        let (x, y): (usize, usize) = cell.get_position().into();
        let idx = idx_segment;

        cell.lifetime += 1;
        match cell.type_cell {
            TypeCell::Conductor => {
                if cell.lifetime > 200 {
                    world.segments[idx_segment] = Segment::Air(Air::new(Vector2::new(x, y)));
                    return;
                }
            }

            TypeCell::Consumer => {
                if cell.lifetime > 50 {
                    world.segments[idx_segment] = Segment::Air(Air::new(Vector2::new(x, y)));
                    return;
                }
            }

            TypeCell::Producer => {
                if cell.lifetime > 70 {
                    world.segments[idx_segment] = Segment::Air(Air::new(Vector2::new(x, y)));
                    return;
                }
            }

            TypeCell::Builder => {
                if cell.lifetime > 40 {
                    world.segments[idx_segment] = Segment::Air(Air::new(Vector2::new(x, y)));
                    return;
                }
            }

            TypeCell::Photosynthetic => {
                if cell.lifetime > 30 {
                    world.segments[idx_segment] = Segment::Air(Air::new(Vector2::new(x, y)));
                    return;
                }
            }
        }

        match cell.type_cell {
            TypeCell::Producer => {
                let idx_bottom = get_index(x, y - 1, SIZE_WORLD[0]);
                if let Segment::Air(air) = &world_read.segments[idx_bottom] {
                    let mut cell = cell.clone();
                    let mut air = air.clone();

                    cell.position.y -= 1;
                    air.position.y += 1;

                    world.segments[idx_bottom] = Segment::Cell(cell);
                    world.segments[idx] = Segment::Air(Air::from(air));
                } else if let Segment::Dirt(_) = &world_read.segments[idx_bottom] {
                    let mut new_cell = cell.clone();
                    new_cell.lifetime = 0;
                    new_cell.mutate();

                    let gene = cell.genome.0[cell.children[0]];

                    new_cell.children = gene.children;
                    new_cell.type_cell = gene.type_cell;

                    world.segments[idx] = Segment::Cell(new_cell);
                }
            }

            TypeCell::Builder => {
                let mut children = cell.share();
                if let Some(child) = &mut children[0] {
                    child.position = cell.position;
                    world.segments[idx_segment] = Segment::Cell(child.clone());
                }

                let n = get_idx_neighbors(world.segments[idx_segment].to_cell().unwrap());

                for (i, segment) in world_read.get_segments_at(n.to_vec()).iter().enumerate() {
                    if let Segment::Air(_) = segment {
                        if let Some(child) = &mut children[i + 1] {
                            child.position = world.segments[n[i]].get_position().unwrap();
                            world.segments[n[i]] = Segment::Cell(child.clone());
                        }
                    }
                }
            }

            _ => {}
        }
    }
}
