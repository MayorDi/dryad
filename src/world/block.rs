use nalgebra::Vector2;

use super::{Chemical, Physical, Position, Behaviour};

/// `Block` представляет твёрдую основу сетки, в основном выступают в роли почвы.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Block {
    pub position: Vector2<usize>,
    pub chemical: Chemical,
    pub physical: Physical,
}

impl Position for Block {
    fn get_position(&self) -> Vector2<usize> {
        self.position
    }
}

impl Behaviour for Block {
    fn update(&mut self, world: &mut super::World) {
        
    }
}