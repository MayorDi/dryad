use sdl2::rect::Rect;

use super::{App, SDL};
use crate::{
    constants::{colors::*, world::SIZE_WORLD},
    traits::{Position, Render},
    world::*,
};

const SIZE_RECT: i32 = 5;

impl App {
    pub fn render(world_read: &World, sdl: &mut SDL) {
        for segment in world_read.segments.iter() {
            match &segment {
                Segment::Air(air) => air.render(sdl),
                Segment::Cell(cell) => cell.render(sdl),
                Segment::Dirt(block) => block.render(sdl),
            }
        }
    }
}

impl Render for Air {
    fn render(&self, sdl: &mut SDL) {
        let (x, y): (i32, i32) = self.get_position().into();
        let canvas = &mut sdl.canvas;
        let rect = Rect::new(
            x * SIZE_RECT,
            SIZE_RECT * (-y + SIZE_WORLD[1] as i32 - 1),
            SIZE_RECT as u32,
            SIZE_RECT as u32,
        );

        canvas.set_draw_color(self.physical.color);
        canvas.fill_rect(rect).unwrap();
    }
}

impl Render for Cell {
    fn render(&self, sdl: &mut crate::app::SDL) {
        let (x, y): (i32, i32) = self.get_position().into();
        let canvas = &mut sdl.canvas;
        let rect = Rect::new(
            x * SIZE_RECT,
            SIZE_RECT * (-y + SIZE_WORLD[1] as i32 - 1),
            SIZE_RECT as u32,
            SIZE_RECT as u32,
        );

        match self.type_cell {
            TypeCell::Builder => canvas.set_draw_color(COLOR_BUILDER),
            TypeCell::Conductor => canvas.set_draw_color(COLOR_CONDUCTOR),
            TypeCell::Consumer => canvas.set_draw_color(COLOR_CONSUMER),
            TypeCell::Photosynthetic => canvas.set_draw_color(COLOR_PHOTOSYNTHETIC),
            TypeCell::Producer => canvas.set_draw_color(COLOR_PRODUCER),
        }

        canvas.fill_rect(rect).unwrap();
    }
}

impl Render for Block {
    fn render(&self, sdl: &mut SDL) {
        let (x, y): (i32, i32) = self.get_position().into();
        let canvas = &mut sdl.canvas;
        let rect = Rect::new(
            x * SIZE_RECT,
            SIZE_RECT * (-y + SIZE_WORLD[1] as i32 - 1),
            SIZE_RECT as u32,
            SIZE_RECT as u32,
        );

        canvas.set_draw_color(self.physical.color);
        canvas.fill_rect(rect).unwrap();
    }
}
