mod air;
mod block;
mod cell;
mod composition;
mod genome;
mod segment;

use std::fmt::Debug;

use nalgebra::Vector2;
use rand::Rng;

pub use crate::constants::world::*;
use crate::{constants::colors::*, traits::GetPosition};

use crate::alias::Position;
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
            let pos = get_pos(i, SIZE_WORLD[0]);
            let (x, y) = (pos.x, pos.y);

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
            Vector2::new(128, 30),
            TypeCell::Producer,
            rand::thread_rng().gen_range(0..1_000_000),
        );

        self.segments[get_index(128, 25, SIZE_WORLD[0])] = Segment::Cell(cell);

        log::info!(target: "world_generate", "The world has been successfully generated.");
    }

    pub fn get_segments_at(&self, idxs: Vec<usize>) -> Vec<Segment> {
        idxs.iter()
            .filter(|i| **i < COUNT_SEGMENTS)
            .map(|i| self.segments[*i].clone())
            .collect()
    }
}

/// Getting the segment position by its index.
pub const fn get_pos(index: usize, width: usize) -> Position {
    Position::new(index % width, index / width)
}

/// Getting the segment index by its position.
pub const fn get_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Get adjoining cells near segment of grid.
pub fn get_idx_neighbors<T: GetPosition>(segment: &T) -> [usize; 4] {
    let (width, height) = (SIZE_WORLD[0], SIZE_WORLD[1]);
    let (x, y): (i32, i32) = (
        segment.get_position().x as i32,
        segment.get_position().y as i32,
    );
    let (w_max, h_max) = (width as i32, height as i32);

    let idxes = [
        get_index(limit(0, w_max, x - 1), y as usize, width),
        get_index(limit(0, w_max, x + 1), y as usize, width),
        get_index(x as usize, limit(0, h_max, y + 1), width),
        get_index(x as usize, limit(0, h_max, y - 1), width),
    ];

    idxes
}

fn limit(min: i32, max: i32, n: i32) -> usize {
    if n < min {
        return min as usize;
    } else if n > max {
        return max as usize;
    }

    n as usize
}
