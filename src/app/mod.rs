use std::time::Duration;

use crate::{world::World, constants::colors::BACKGROUND};

pub mod sdl;
pub mod render;
pub mod update;

pub use render::*;
pub use update::*;
pub use sdl::*;

use sdl2::{event::Event, keyboard::Keycode};

pub struct App {
    pub world: World,
    pub settings: Settings,
    pub sdl: SDL
}

impl App {
    pub fn init() -> Self {
        let mut world = World::new();
        let settings = Settings::default();
        let sdl = SDL::init(&settings);
        
        world.generate();
        
        Self {
            world,
            settings,
            sdl,
        }
    }

    pub fn run(&mut self) {
        'running: loop {
            self.sdl.canvas.set_draw_color(BACKGROUND.to_bytes());
            self.sdl.canvas.clear();

            let world_read = self.world.clone();
            for idx in 0..world_read.segments.iter().len() {
                self.update(idx);
                App::render(&world_read, &mut self.sdl, idx);
            }
    
            if self.event_handler() { break 'running; }

            self.sdl.canvas.present();
    
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / crate::constants::app::FPS));
        }
    }

    fn event_handler(&self) -> bool {
        let mut event_pump = self.sdl.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                _ => {}
            }
        }

        false
    }
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