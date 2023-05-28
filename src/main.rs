#![cfg(target_os = "windows")]

use std::time::Duration;

use dryad::{app::*, color::*, world::*, constants::cell::MAX_LIFE_TIME};
use nalgebra::Vector2;
use sdl2::{self, event::Event, keyboard::Keycode, rect::Rect};

const SIZE_RECT: i32 = 5;

pub fn main() -> Result<(), String> {
    let mut app = App {
        world: World::new(),
        settings: Settings {
            title: String::from("Dryad"),
        },
    };

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(app.settings.title.as_str(), 1300, 600)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut world_buf = app.world.clone();

    'running: loop {
        canvas.set_draw_color(BACKGROUND.to_bytes());
        canvas.clear();

        world_buf = app.world.clone();
        for i in 0..COUNT_SEGMENTS {
            let ((x, y), (width, _height), neighbors) = get_idx_neighbors(&app.world.segments[i]);

            match app.world.segments[i] {
                Segment::Dirt(_) => {
                    let wtr = 10.0;

                    for j in 0..4 {
                        if let Segment::Dirt(neighbor) = &app.world.segments[neighbors[j]] {
                            if world_buf.segments[i].to_block().chemical.water
                                > neighbor.chemical.water
                                && 500.0 > neighbor.chemical.water
                            {
                                world_buf.segments[i].to_block().chemical.water -= wtr;
                                world_buf.segments[neighbors[j]].to_block().chemical.water += wtr;
                            }
                        }
                    }
                }

                Segment::Cell(_) => {
                    match app.world.segments[i].to_cell().type_cell {
                        TypeCell::Producer => {
                            if let Segment::Dirt(_) = world_buf.segments[neighbors[3]] {
                                let mut cell = world_buf.segments[i].to_cell();
                                let gene = cell.genome.0[cell.children[0]];

                                cell.id += 1;
                                cell.children = gene.children;
                                cell.type_cell = gene.type_cell;
                                cell.mutate();
                            } else if let Segment::Air(_) = world_buf.segments[neighbors[3]] {
                                let mut cell = world_buf.segments[i].to_cell().clone();
                                cell.position.y -= 1;

                                world_buf.segments[get_index(x, y - 1, width)] =
                                    Segment::Cell(cell);
                                world_buf.segments[i] = Segment::Air(Air::new(Vector2::new(x, y)));
                            }
                        }

                        TypeCell::Builder => {
                            let parent = world_buf.segments[i].to_cell().clone();
                            let neighbors = [i, neighbors[0], neighbors[1], neighbors[2]];
                            let mut children = [
                                parent.clone(),
                                parent.clone(),
                                parent.clone(),
                                parent.clone(),
                            ];

                            let gene = parent.genome.0[parent.children[0]];
                            children[0].type_cell = gene.type_cell;
                            children[0].children = gene.children;

                            let gene = parent.genome.0[parent.children[1]];
                            children[1].position.x =
                                limit(0.0, width as f32 - 1.0, children[1].position.x as f32 - 1.0)
                                    as usize;
                            children[1].type_cell = gene.type_cell;
                            children[1].children = gene.children;
                            children[1].lifetime = 0;

                            let gene = parent.genome.0[parent.children[2]];
                            children[2].position.x =
                                limit(0.0, width as f32 - 1.0, children[2].position.x as f32 + 1.0)
                                    as usize;
                            children[2].type_cell = gene.type_cell;
                            children[2].children = gene.children;
                            children[2].lifetime = 0;

                            let gene = parent.genome.0[parent.children[3]];
                            children[3].position.y += 1;
                            children[3].type_cell = gene.type_cell;
                            children[3].children = gene.children;
                            children[3].lifetime = 0;

                            for idx in 0..4 {
                                if parent.children[idx] == 0 {
                                    continue;
                                }

                                if idx != 0 {
                                    if let Segment::Air(_) = world_buf.segments[neighbors[idx]] {
                                        world_buf.segments[neighbors[idx]] =
                                            Segment::Cell(children[idx].clone());
                                    }
                                } else {
                                    world_buf.segments[neighbors[idx]] =
                                        Segment::Cell(children[idx].clone());
                                }
                            }
                        }

                        _ => {}
                    }

                    if let Segment::Cell(cell) = &mut world_buf.segments[i] {
                        cell.lifetime += 1;

                        if cell.lifetime > MAX_LIFE_TIME {
                            world_buf.segments[i] = Segment::Air(Air::new(cell.position));
                        }
                    }
                }
                _ => {}
            }

            let rect = Rect::new(
                x as i32 * SIZE_RECT,
                SIZE_RECT * (-(y as i32) + SIZE_WORLD[1] as i32 - 1),
                SIZE_RECT as u32,
                SIZE_RECT as u32,
            );

            match &mut app.world.segments[i] {
                Segment::Air(air) => {
                    canvas.set_draw_color(air.physical.color.to_bytes());
                    canvas.fill_rect(rect).unwrap();
                }

                Segment::Cell(cell) => {
                    match cell.type_cell {
                        TypeCell::Builder => {
                            canvas.set_draw_color((0x80, 0xb8, 0x63, 255));
                        }
                        TypeCell::Conductor => {
                            canvas.set_draw_color((0x80, 0x5d, 0x55, 255));
                        }
                        TypeCell::Consumer => {
                            canvas.set_draw_color((0xd4, 0x80, 0x6b, 255));
                        }
                        TypeCell::Photosynthetic => {
                            canvas.set_draw_color((0x80, 0xb8, 0x63, 255));
                        }
                        TypeCell::Producer => {
                            canvas.set_draw_color((0xed, 0xf2, 0xa8, 255));
                        }
                    }

                    canvas.fill_rect(rect).unwrap();
                }
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

        app.world = world_buf;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
