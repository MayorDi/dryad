use nalgebra::Vector2;

use super::{Physical, Position};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Air {
    pub position: Vector2<usize>,
    pub physical: Physical,
}

impl Position for Air {
    fn get_position(&self) -> Vector2<usize> {
        self.position
    }
}