#![cfg(target_os="windows")]

use std::time::Duration;

use dryad::{app::{App, Settings}, world::{World, SIZE_WORLD, Segment, get_idx_neighbors}, color::*};
use sdl2::{self, event::Event, keyboard::Keycode, rect::Rect};

const SIZE_RECT: i32 = 5;

pub fn main() -> Result<(), String> {
    let mut app = App {
        world: World::new(),
        settings: Settings { title: String::from("Dryad") }
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Dryad", 1300, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        canvas.set_draw_color(BACKGROUND.to_bytes());
        canvas.clear();

        let mut world_buf = app.world.clone();
        for i in 0..app.world.segments.len() {
            let (
                (x, y), 
                (_width, _height), 
                neighbors
            ) = get_idx_neighbors(&app.world.segments[i]);

            if let Segment::Dirt(_) = world_buf.segments[i] {
                let wtr = 10.0;

                for j in 0..4 {
                    if let Segment::Dirt(neighbor) = &app.world.segments[neighbors[j]] {
                        if world_buf.segments[i].to_block().chemical.water > neighbor.chemical.water &&
                           500.0 > neighbor.chemical.water{
                            world_buf.segments[i].to_block().chemical.water -= wtr;
                            world_buf.segments[neighbors[j]].to_block().chemical.water += wtr;
                        }
                    }
                }
            }

            let rect = Rect::new(
                x as i32 * SIZE_RECT,
                SIZE_RECT*(-(y as i32) + SIZE_WORLD[1] as i32 - 1),
                SIZE_RECT as u32,
                SIZE_RECT as u32);
    
            match &mut app.world.segments[i] {
                Segment::Air(air) => {
                    canvas.set_draw_color(air.physical.color.to_bytes());
                    canvas.fill_rect(rect).unwrap();
                },
                Segment::Cell(cell) => {
                    canvas.set_draw_color(cell.physical.color.to_bytes());
                    canvas.fill_rect(rect).unwrap();
                },
                Segment::Dirt(block) => {
                    block.physical.color = COLOR_DIRT * (1.0 - block.chemical.water / 500.0);
                    
                    canvas.set_draw_color(block.physical.color.to_bytes());
                    canvas.fill_rect(rect).unwrap();
                }
            }
        }

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

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        app.world = world_buf;
    }

    Ok(())
}