use std::time::Duration;

use crate::{constants::colors::BACKGROUND, world::World};

pub mod render;
pub mod sdl;
pub mod update;

pub use render::*;
pub use sdl::*;
pub use update::*;

use sdl2::{event::Event, keyboard::Keycode};

pub struct App {
    pub world: World,
    pub settings: Settings,
}

impl App {
    pub fn init() -> Self {
        let mut world = World::new();
        let settings = Settings::default();

        world.generate();

        Self { world, settings }
    }

    pub fn run(&mut self) {
        let stop_frame = Duration::new(0, 1_000_000_000u32 / crate::constants::app::FPS);
        let mut sdl = SDL::init(&self.settings);

        'running: loop {
            sdl.canvas.set_draw_color(BACKGROUND);
            sdl.canvas.clear();

            if event_handler(&sdl) {
                break 'running;
            }

            let world_read = self.world.clone();
            self.update(&world_read);
            App::render(&world_read, &mut sdl);

            sdl.canvas.present();

            ::std::thread::sleep(stop_frame);
        }
    }
}

pub(self) fn event_handler(sdl: &SDL) -> bool {
    let mut event_pump = sdl.sdl_context.event_pump().unwrap();

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

pub struct Settings {
    pub title: String,
    pub size: (u32, u32),
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            title: String::from("Dryad"),
            size: (1200, 600),
        }
    }
}
