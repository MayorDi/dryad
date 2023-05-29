use crate::world::World;

pub mod sdl;

pub use sdl::*;


pub struct App {
    pub world: World,
    pub settings: Settings,
    pub render: fn(&'static Self),
    pub update: fn(&'static mut Self),
}

impl App {
    pub fn init(render: fn(&'static Self), update: fn(&'static mut Self)) -> Self {
        let mut world = World::new();
        world.generate();

        let settings = Settings::default();

        Self {
            world,
            settings,
            render,
            update,
        }
    }

    // pub fn run(mut self) {
    //     let update = self.update;
    //     let render = self.render;

    //     std::thread::Builder::new()
    //         .name("update".to_string())
    //         .spawn(|| {
    //             update(&mut self);
    //         });

    //     std::thread::Builder::new()
    //         .name("render".to_string())
    //         .spawn(|| {
    //             render(&self);
    //         });
    // }
}

pub struct Settings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: String::from("Dryad"),
            size: (1200, 600)
        }
    }
}