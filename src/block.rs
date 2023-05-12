use nalgebra::Vector2;

use crate::composition::*;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Block {
    position: Vector2<usize>,
    chemical: Chemical,
    physical: Physical,
}