mod block;
mod segment;
mod cell;
mod air;
mod genome;
mod composition;

use nalgebra::Vector2;
use rand::Rng;

use crate::constants::colors::*;
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
        let mut segments: Segments = vec![Segment::Air(Air::default()); COUNT_SEGMENTS].into_boxed_slice();

        for (i, segment) in segments.iter_mut().enumerate() {
            let (x, y) = get_pos(i, SIZE_WORLD[0]);

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

        let cell = Cell::new(Vector2::new(128, 50), TypeCell::Producer, rand::thread_rng().gen_range(0..1000000));

        segments[get_index(128, 50, SIZE_WORLD[0])] = Segment::Cell(cell);

        Self {
            segments,
            light: 1000.0
        }
    }
}

pub fn get_pos(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

pub fn get_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

pub trait Position {
    fn get_position(&self) -> Vector2<usize>;
}

// oh no...
pub fn get_idx_neighbors<T: Position>(segment: &T) -> (
    (usize, usize), (usize, usize), [usize; 4]
) {
    let (width, height) = (SIZE_WORLD[0], SIZE_WORLD[1]);
    let (x, y) = (segment.get_position().x, segment.get_position().y);

    // idx of neighbors
    let (left_idx, right_idx, top_idx, bottom_idx) = (
        get_index(limit(0.0, width as f32 - 1.0, x as f32 - 1.0) as usize, y, width),
        get_index(limit(0.0, width as f32 - 1.0, x as f32 + 1.0) as usize, y, width),
        get_index(x, limit(0.0, height as f32 - 1.0, y as f32 + 1.0) as usize, width),
        get_index(x, limit(0.0, height as f32 - 1.0, y as f32 - 1.0) as usize, width),
    );

    ((x, y), (width, height), [left_idx, right_idx, top_idx, bottom_idx])
}

pub fn limit(min: f32, max: f32, n: f32) -> f32 {
    if n < min {
        return min;
    } else if n > max {
        return max;
    }

    n
}