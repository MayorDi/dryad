use nalgebra::Vector2;
use super::{Chemical, Physical};

/// `Block` представляет твёрдую основу сетки, в основном выступают в роли почвы.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Block {
    position: Vector2<usize>,
    chemical: Chemical,
    physical: Physical,
}