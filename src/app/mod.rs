use nannou::{prelude::*, event::ElementState};
use nannou_egui::{self, Egui};

use crate::world::World;

mod update;
mod render;

pub struct Settings {
    offset: (f32, f32)
}

pub struct Model {
    settings: Settings,
    world: World,
    egui: Egui,
}

pub fn init(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .view(render)
        .raw_event(raw_event)
        .fullscreen()
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        settings: Settings { offset: (0.0, 0.0) },
        world: World::new(),
        egui
    }
}

pub fn update(app: &App, model: &mut Model, update: Update) {
    self::update::update(app, model, update);  
}

pub fn render(app: &App, model: &Model, frame: Frame) {
    self::render::render(app, model, frame);
}

pub fn raw_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    match event {
        nannou::winit::event::WindowEvent::KeyboardInput { input, .. } => {
            if let ElementState::Pressed = input.state {
                if let Some(key) = input.virtual_keycode {
                    match key {
                        Key::A => model.settings.offset.0 += 10.0,
                        Key::D => model.settings.offset.0 -= 10.0,
                        Key::W => model.settings.offset.1 -= 10.0,
                        Key::S => model.settings.offset.1 += 10.0,
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }

    model.egui.handle_raw_event(event);
}