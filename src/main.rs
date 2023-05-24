#![cfg(target_os="windows")]

use std::time::Duration;

use dryad::{app::{App, Settings}, world::{World, get_pos, SIZE_WORLD, Segment, Behaviour}, color::*};
use sdl2::{self, event::Event, keyboard::Keycode, rect::Rect};

const SIZE_RECT: i32 = 10;

pub fn main() -> Result<(), String> {
    let mut app = App {
        world: World::new(),
        settings: Settings { title: String::from("Dryad") }
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Dryad", 800, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut frame: usize = 0;

    'running: loop {
        canvas.set_draw_color(BACKGROUND.to_bytes());
        canvas.clear();

        for i in 0..app.world.segments.len() {
            let (x, y) = get_pos(i, SIZE_WORLD[0]);

            if frame % 1 == 0 {
                let segment = &mut app.world.segments[i] as *mut Segment;
                
                unsafe {
                    if let Segment::Dirt(block) = &mut (*segment) {
                        block.update(&mut app.world);
                    }
                }
            }

            let rect = Rect::new(
                x as i32 * SIZE_RECT,
                SIZE_RECT*(-(y as i32) + SIZE_WORLD[1] as i32 - 1),
                SIZE_RECT as u32,
                SIZE_RECT as u32);
    
            match &app.world.segments[i] {
                Segment::Air(_) => {
                    canvas.set_draw_color(SKYBLUE.to_bytes());
                    canvas.fill_rect(rect).unwrap();
                },
                Segment::Cell(cell) => {
                    canvas.set_draw_color(cell.physical.color.to_bytes());
                    canvas.fill_rect(rect).unwrap();
                },
                Segment::Dirt(block) => {
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

        frame += 1;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}