use nalgebra::Vector2;

use crate::constants::colors::COLOR_SKYBLUE;

use super::{Physical, Position};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Air {
    pub position: Vector2<usize>,
    pub physical: Physical,
}

impl Air {
    pub fn new(position: Vector2<usize>) -> Self {
        Self {
            position,
            physical: Physical { light: 1.0, color: COLOR_SKYBLUE }
        }
    }
}

impl Position for Air {
    fn get_position(&self) -> Vector2<usize> {
        self.position
    }
}