use nannou::{prelude::*};
use crate::world::{get_pos, SIZE_WORLD, Segment, Color};

use super::Model;

pub const SKYBLUE: (f32, f32, f32) = (0x88 as f32 / 255.0, 0xa4 as f32 / 255.0, 0xff as f32 / 255.0);
pub const SIZE_RECT: f32 = 10.0;

pub fn render(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().x_y(model.settings.offset.0, model.settings.offset.1);

    draw.background().color(rgb::Srgb::new(
        0x0e as f32 / 0xFF as f32, 
        0x12 as f32 / 0xFF as f32, 
        0x15 as f32 / 0xFF as f32));

    for i in 0..model.world.segments.len() {
        let (x, y) = get_pos(i, SIZE_WORLD[0]);
        let rect = draw.rect()
            .x_y(x as f32 * SIZE_RECT, y as f32 * SIZE_RECT)
            .w_h(SIZE_RECT, SIZE_RECT);

        match model.world.segments[i] {
            Segment::Air(_) => {
                rect.color(rgb::Srgba::new(SKYBLUE.0, SKYBLUE.1, SKYBLUE.2, 1.0));
            },
            Segment::Cell(cell) => {
                let Color { r, g, b, a } = cell.physical.color;
                rect.color(rgb::Srgba::new(r, g, b, a));
            },
            Segment::Dirt(block) => {
                let Color { r, g, b, a } = block.physical.color;
                rect.color(rgb::Srgba::new(r, g, b, a));
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}