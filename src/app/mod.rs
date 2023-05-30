use std::time::Duration;

use crate::{world::World, constants::colors::BACKGROUND};

pub mod sdl;

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
        world.generate();

        let settings = Settings::default();
        let sdl = SDL::init(&settings);

        Self {
            world,
            settings,
            sdl,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl.sdl_context.event_pump().unwrap();

        'running: loop {
            self.sdl.canvas.set_draw_color(BACKGROUND.to_bytes());
            self.sdl.canvas.clear();

            self.update();
            self.render();
    
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.sdl.canvas.present();
    
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn update(&mut self) {

    }

    fn render(&self) {

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