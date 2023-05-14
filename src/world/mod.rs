mod block;
mod segment;
mod cell;
mod genome;
mod composition;

pub use block::*;
use nalgebra::Vector2;
pub use segment::*;
pub use cell::*;
pub use genome::*;
pub use composition::*;

pub const SIZE_WORLD: [usize; 2] = [255, 32];
pub const COUNT_SEGMENTS: usize = SIZE_WORLD[0] * SIZE_WORLD[1];
pub type Segments = [Segment; COUNT_SEGMENTS];

#[derive(Debug, Clone)]
pub struct World {
    pub segments: Segments,
    pub light: f32
}

impl World {
    pub fn new() -> Self {
        let mut segments: Segments = [Segment::Air(Physical::default()); COUNT_SEGMENTS];

        for (i, segment) in segments.iter_mut().enumerate() {
            let (x, y) = get_pos(i, SIZE_WORLD[0]);

            if y > 27 {
                let mut dirt = Block::default();
                dirt.position = Vector2::new(x, y);
                dirt.chemical.metals = 200.0;
                dirt.chemical.water = 250.0;
                dirt.chemical.nitrates = 60.0;
                dirt.chemical.nitrites = 10.0;

                *segment = Segment::Dirt(Block::default());
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
    y*width+x
}