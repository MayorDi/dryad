mod air;
mod block;
mod cell;
mod composition;
mod genome;
mod segment;

use std::fmt::Debug;

use nalgebra::{ArrayStorage, Matrix, Vector2, U1, U2};
use rand::Rng;

pub use crate::constants::world::*;
use crate::{constants::colors::*, traits::Position};

pub use air::*;
pub use block::*;
pub use cell::*;
pub use composition::*;
pub use genome::*;
pub use segment::*;

type Segments = Box<[Segment]>;

#[derive(Debug, Clone)]
pub struct World {
    pub segments: Segments,
    pub light: f32,
}

impl World {
    pub fn new() -> Self {
        let segments: Segments =
            vec![Segment::Air(Air::default()); COUNT_SEGMENTS].into_boxed_slice();

        Self {
            segments,
            light: 100.0,
        }
    }

    /// To create a world.
    pub fn generate(&mut self) {
        log::info!(target: "world_generate", "The process of generating the world has begun.");

        // Create sements for grid.
        for (i, segment) in self.segments.iter_mut().enumerate() {
            let (x, y) = get_pos(i, SIZE_WORLD[0]).into();

            *segment = Segment::Air(Air::new(Vector2::new(x, y)));

            // Create dirt.
            if y < 20 {
                let mut dirt = Block::default();
                dirt.position = Vector2::new(x, y);

                dirt.chemical.metals = 200.0;
                dirt.chemical.water = rand::thread_rng().gen_range(150.0..350.0);

                if y > 18 {
                    dirt.chemical.water = 500.0;
                }

                dirt.chemical.nitrates = 60.0;
                dirt.chemical.nitrites = 10.0;

                dirt.physical.color = COLOR_DIRT;

                *segment = Segment::Dirt(dirt);
            }
        }

        // Add cell.
        let cell = Cell::new(
            Vector2::new(128, 25),
            TypeCell::Producer,
            rand::thread_rng().gen_range(0..1000000),
        );

        self.segments[get_index(128, 25, SIZE_WORLD[0])] = Segment::Cell(cell);

        log::info!(target: "world_generate", "The world has been successfully generated.");
    }
}

/// Getting the segment position by its index.
pub const fn get_pos(index: usize, width: usize) -> VectorWrapper<usize> {
    VectorWrapper(Vector2::new(index % width, index / width))
}

/// Getting the segment index by its position.
pub const fn get_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Vector's wrapper for more convenient conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VectorWrapper<T>(pub Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>)
where
    T: Clone + Copy + Debug + 'static;

impl From<(usize, usize)> for VectorWrapper<usize> {
    fn from(value: (usize, usize)) -> Self {
        VectorWrapper(Vector2::new(value.0, value.1))
    }
}

impl From<(i32, i32)> for VectorWrapper<usize> {
    fn from(value: (i32, i32)) -> Self {
        VectorWrapper(Vector2::new(value.0 as usize, value.1 as usize))
    }
}

impl Into<(usize, usize)> for VectorWrapper<usize> {
    fn into(self) -> (usize, usize) {
        (self.0.x, self.0.y)
    }
}

impl Into<(i32, i32)> for VectorWrapper<usize> {
    fn into(self) -> (i32, i32) {
        (self.0.x as i32, self.0.y as i32)
    }
}

/// Get adjoining cells near segment of grid.
pub fn get_idx_neighbors<T: Position>(segment: &T) -> [usize; 4] {
    let (width, height) = (SIZE_WORLD[0], SIZE_WORLD[1]);
    let (x, y): (i32, i32) = segment.get_position().into();
    let (w_max, h_max) = (width as i32 - 1, height as i32 - 1);

    let idxes = [
        get_index(
            (((x - 1 % w_max) + w_max) % w_max) as usize,
            y as usize,
            width,
        ),
        get_index(
            (((x + 1 % w_max) + w_max) % w_max) as usize,
            y as usize,
            width,
        ),
        get_index(
            x as usize,
            (((y - 1 % h_max) + h_max) % h_max) as usize,
            width,
        ),
        get_index(
            x as usize,
            (((y + 1 % h_max) + h_max) % h_max) as usize,
            width,
        ),
    ];

    idxes
}
