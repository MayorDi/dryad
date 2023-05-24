use crate::world::World;

pub struct App {
    pub world: World,
    pub settings: Settings,
}

pub struct Settings {
    pub title: String,
}