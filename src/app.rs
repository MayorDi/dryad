use crate::world::World;

#[derive(Clone, Debug)]
pub struct App {
    pub world: World,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }
}