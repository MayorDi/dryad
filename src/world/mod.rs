mod block;
mod segment;
mod cell;
mod air;
mod genome;
mod composition;

use nalgebra::{Vector2, ArrayStorage, Matrix, U2, U1};
use rand::Rng;

use crate::{constants::colors::*, app::SDL};
pub use crate::constants::world::*;

pub use block::*;
pub use air::*;
pub use segment::*;
pub use cell::*;
pub use genome::*;
pub use composition::*;

type Segments = Box<[Segment]>;

#[derive(Debug, Clone)]
pub struct World {
    pub segments: Segments,
    pub light: f32
}

impl World {
    pub fn new() -> Self {
        let segments: Segments = vec![Segment::Air(Air::default()); COUNT_SEGMENTS].into_boxed_slice();

        Self {
            segments,
            light: 100.0
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

                if y > 18 { dirt.chemical.water = 500.0; }

                dirt.chemical.nitrates = 60.0;
                dirt.chemical.nitrites = 10.0;

                dirt.physical.color = COLOR_DIRT;

                *segment = Segment::Dirt(dirt);
            }
        }
        let cell = Cell::new(Vector2::new(128, 25), TypeCell::Producer, rand::thread_rng().gen_range(0..1000000));

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

pub trait Position {
    fn get_position(&self) -> VectorWrapper<usize>;
    fn get_index(&self) -> usize {
        let (x, y) = self.get_position().into();
        get_index(x, y, SIZE_WORLD[0])
    }
}

pub struct VectorWrapper<T>(pub Matrix<T, U2, U1, ArrayStorage<T, 2, 1>>);

impl Into<(usize, usize)> for VectorWrapper<usize> {
    fn into(self) -> (usize, usize) {
        (self.0.x, self.0.y)
    }
}

/// Trait for simple behavior.
pub trait Behaviour {
    fn update(&mut self, world_read: &World, world: &mut World);
}

pub trait Render {
    fn render(&self, sdl: &mut SDL);
}

// oh no...
pub fn get_idx_neighbors<T: Position>(segment: &T) -> Vec<usize> {
    let (width, height) = (SIZE_WORLD[0], SIZE_WORLD[1]);
    let (x, y) = segment.get_position().into();

    let idxes = [
        get_index(limit(0.0, width as f32 - 1.0, x as f32 - 1.0) as usize, y, width),
        get_index(limit(0.0, width as f32 - 1.0, x as f32 + 1.0) as usize, y, width),
        get_index(x, limit(0.0, height as f32 - 1.0, y as f32 - 1.0) as usize, width),
        get_index(x, limit(0.0, height as f32 - 1.0, y as f32 + 1.0) as usize, width),
    ];

    idxes.to_vec()
}

pub fn limit(min: f32, max: f32, n: f32) -> f32 {
    if n < min {
        return max;
    } else if n > max {
        return min;
    }

    n
}