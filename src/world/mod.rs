mod air;
mod block;
mod cell;
mod composition;
mod genome;
mod segment;

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

    pub fn generate(&mut self) {
        for (i, segment) in self.segments.iter_mut().enumerate() {
            let (x, y) = get_pos(i, SIZE_WORLD[0]).into();

            *segment = Segment::Air(Air::new(Vector2::new(x, y)));

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
        let cell = Cell::new(
            Vector2::new(128, 25),
            TypeCell::Producer,
            rand::thread_rng().gen_range(0..1000000),
        );

        self.segments[get_index(128, 25, SIZE_WORLD[0])] = Segment::Cell(cell);
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

pub struct VectorWrapper<T>(pub Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>);

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
