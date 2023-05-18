use jarylo::paint::Painter;

use crate::{app::*, world::*, colors::*};

pub const SIZE_RECT: f32 = 10.0;

pub fn render(app: *mut App, painter: &mut Painter) {
    let a;
    unsafe { a = &mut *app; }
    let app = a;

    for i in 0..app.world.segments.len() {
        match app.world.segments[i] {
            Segment::Air(_) => {
            },

            Segment::Cell(cell) => {
            },

            Segment::Dirt(block) => {
            }
        }
    }
}