use nannou::prelude::*;
use nannou_egui::{self, egui};
use super::Model;

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Test")
        .show(&ctx, |ui|{
            ui.label("Test text");
        });    
}