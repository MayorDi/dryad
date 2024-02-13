use macros_helper::GetPosition;
use nalgebra::Vector2;

use crate::constants::colors::COLOR_SKYBLUE;

use super::{get_index, GetPosition, Physical};
use crate::{alias::Position, constants::world::SIZE_WORLD};

#[derive(Debug, Default, Clone, PartialEq, GetPosition)]
pub struct Air {
    pub position: Vector2<usize>,
    pub physical: Physical,
}

impl Air {
    pub fn new(position: Vector2<usize>) -> Self {
        Self {
            position,
            physical: Physical {
                light: 1.0,
                color: COLOR_SKYBLUE,
            },
        }
    }
}
