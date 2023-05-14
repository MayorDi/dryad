use nannou::prelude::*;
use nannou_egui::{self, Egui};

mod update;
mod render;

pub struct Model {
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
    model.egui.handle_raw_event(event);
}