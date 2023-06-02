use sdl2::rect::Rect;

use super::{App, SDL};
use crate::{world::*, constants::{world::SIZE_WORLD, colors::*}};

const SIZE_RECT: i32 = 5;

impl App {
    pub fn render(world_read: &World, sdl: &mut SDL, idx: usize) {
        let (x, y) = get_pos(idx, SIZE_WORLD[0]);
        let canvas = &mut sdl.canvas;

        let rect = Rect::new(
            x as i32 * SIZE_RECT,
            SIZE_RECT * (-(y as i32) + SIZE_WORLD[1] as i32 - 1),
            SIZE_RECT as u32,
            SIZE_RECT as u32,
        );

        match &world_read.segments[idx] {
            Segment::Air(air) => {
                canvas.set_draw_color(air.physical.color.to_bytes());
                canvas.fill_rect(rect).unwrap();
            }

            Segment::Cell(cell) => {
                match cell.type_cell {
                    TypeCell::Builder => canvas.set_draw_color(COLOR_BUILDER.to_bytes()),
                    TypeCell::Conductor => canvas.set_draw_color(COLOR_CONDUCTOR.to_bytes()),
                    TypeCell::Consumer => canvas.set_draw_color(COLOR_CONSUMER.to_bytes()),
                    TypeCell::Photosynthetic => canvas.set_draw_color(COLOR_PHOTOSYNTHETIC.to_bytes()),
                    TypeCell::Producer => canvas.set_draw_color(COLOR_PRODUCER.to_bytes()),
                }

                canvas.fill_rect(rect).unwrap();
            }
            Segment::Dirt(block) => {
                canvas.set_draw_color(block.physical.color.to_bytes());
                canvas.fill_rect(rect).unwrap();
            }
        }
    }
}