use nalgebra::Vector2;

use crate::composition::*;

/// `Block` представляет твёрдую основу сетки, в основном выступают в роли почвы.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Block {
    position: Vector2<usize>,
    chemical: Chemical,
    physical: Physical,
}