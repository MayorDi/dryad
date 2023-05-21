mod block;
mod segment;
mod cell;
mod genome;
mod composition;

use nalgebra::Vector2;
use rand::Rng;

pub use crate::constants::world::*;

pub use block::*;
pub use segment::*;
pub use cell::*;
pub use genome::*;
pub use composition::*;

type Segments = Vec<Segment>;

#[derive(Debug, Clone)]
pub struct World {
    pub segments: Segments,
    pub light: f32
}

impl World {
    pub fn new() -> Self {
        let mut segments: Segments = vec![Segment::Air(Physical::default()); COUNT_SEGMENTS];

        for (i, segment) in segments.iter_mut().enumerate() {
            let (x, y) = get_pos(i, SIZE_WORLD[0]);

            if y < 10 {
                let mut dirt = Block::default();
                dirt.position = Vector2::new(x, y);

                dirt.chemical.metals = 200.0;
                dirt.chemical.water = rand::thread_rng().gen_range(150.0..350.0);
                dirt.chemical.nitrates = 60.0;
                dirt.chemical.nitrites = 10.0;

                dirt.physical.color.r = 0x8d as f32 / 255.0;
                dirt.physical.color.g = 0x64 as f32 / 255.0;
                dirt.physical.color.b = 0x5a as f32 / 255.0;
                dirt.physical.color.a = 1.0;

                *segment = Segment::Dirt(dirt);
            }
        }
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

pub trait Behaviour {
    fn update(&mut self, world: &mut World);
}

// oh no...
fn get_idx_neighbors<T: Position>(segment: &T) -> (
    (usize, usize), (usize, usize), usize, [Option<usize>; 4]
) {
    let (width, height) = (SIZE_WORLD[0], SIZE_WORLD[1]);
    let (x, y) = (segment.get_position().x, segment.get_position().y);
    let idx = get_index(x, y, width);

    // idx of neighbors
    let (left_idx, right_idx, top_idx, bottom_idx) = (
        check_limit_pos(x, y, width, height, (-1, 0)),
        check_limit_pos(x, y, width, height, ( 1, 0)),
        check_limit_pos(x, y, width, height, ( 1, 1)),
        check_limit_pos(x, y, width, height, (-1, 1)),
    );

    ((x, y), (width, height), idx, [left_idx, right_idx, top_idx, bottom_idx])
}

// oh no...
fn check_limit_pos(x: usize, y: usize, width: usize, height: usize, n: (i32, usize)) -> Option<usize> {
    let (x, y, width, height) = (x as i32, y as i32, width as i32, height as i32);

    if n.1 == 0 {
        if  x + n.0 < 0 || x + n.0 > width {
            return None;
        }

        return Some(get_index((x + n.0) as usize, y as usize, width as usize));
    } 

    if y + n.0 < 0 || y + n.0 > height {
        return None;
    }

    return Some(get_index(x as usize, (y + n.0) as usize, width as usize));
}