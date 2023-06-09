#![cfg(target_os = "windows")]

use dryad::app::App;

pub fn main() {
    App::init().run();
}

// let sdl_context = sdl2::init()?;
    // let video_subsystem = sdl_context.video()?;

    // let window = video_subsystem
    //     .window(app.settings.title.as_str(), 1300, 600)
    //     .position_centered()
    //     .resizable()
    //     .opengl()
    //     .build()
    //     .map_err(|e| e.to_string())?;

    // let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    // let mut event_pump = sdl_context.event_pump()?;
    // let mut world_buf = app.world.clone();

    // 'running: loop {
    //     canvas.set_draw_color(BACKGROUND.to_bytes());
    //     canvas.clear();

    //     world_buf = app.world.clone();
    //     for i in 0..COUNT_SEGMENTS {

    //         match app.world.segments[i] {
    //             Segment::Dirt(_) => {
    //                 let wtr = 10.0;

    //                 for j in 0..4 {
    //                     if let Segment::Dirt(neighbor) = &app.world.segments[neighbors[j]] {
    //                         if world_buf.segments[i].to_block().chemical.water
    //                             > neighbor.chemical.water
    //                             && 500.0 > neighbor.chemical.water
    //                         {
    //                             world_buf.segments[i].to_block().chemical.water -= wtr;
    //                             world_buf.segments[neighbors[j]].to_block().chemical.water += wtr;
    //                         }
    //                     }
    //                 }
    //             }

    //             Segment::Cell(_) => {
    //                 match app.world.segments[i].to_cell().type_cell {
    //                     TypeCell::Photosynthetic => {
    //                         world_buf.segments[i].to_cell().chemical.glucose -= 1.0;

    //                         let wtr = 10.0 * 0.2;
    //                         let gluc = 2.0;

    //                         for j in 0..4 {
    //                             if let Segment::Cell(neighbor) = &app.world.segments[neighbors[j]] {
    //                                 if neighbor.id != world_buf.segments[i].to_cell().id {
    //                                     continue;
    //                                 }

    //                                 if world_buf.segments[i].to_cell().chemical.water
    //                                     > neighbor.chemical.water
    //                                     && 200.0 > neighbor.chemical.water
    //                                 {
    //                                     world_buf.segments[i].to_cell().chemical.water -= wtr;
    //                                     world_buf.segments[neighbors[j]]
    //                                         .to_cell()
    //                                         .chemical
    //                                         .water += wtr;
    //                                 }

    //                                 if world_buf.segments[i].to_cell().chemical.glucose
    //                                     > neighbor.chemical.glucose
    //                                     && 200.0 > neighbor.chemical.glucose
    //                                 {
    //                                     world_buf.segments[i].to_cell().chemical.water -= gluc;
    //                                     world_buf.segments[neighbors[j]]
    //                                         .to_cell()
    //                                         .chemical
    //                                         .water += gluc;
    //                                 }
    //                             }
    //                         }

    //                         world_buf.segments[i]
    //                             .to_cell()
    //                             .synthesize_glucose(app.world.light);
    //                     }

    //                     TypeCell::Producer => {
    //                         world_buf.segments[i].to_cell().chemical.glucose -= 5.0;

    //                         if let Segment::Dirt(_) = world_buf.segments[neighbors[3]] {
    //                             let mut cell = world_buf.segments[i].to_cell();
    //                             let gene = cell.genome.0[cell.children[0]];

    //                             cell.id += 1;
    //                             cell.children = gene.children;
    //                             cell.type_cell = gene.type_cell;
    //                             cell.mutate();
    //                         } else if let Segment::Air(_) = world_buf.segments[neighbors[3]] {
    //                             let mut cell = world_buf.segments[i].to_cell().clone();
    //                             cell.position.y -= 1;

    //                             world_buf.segments[get_index(x, y - 1, width)] =
    //                                 Segment::Cell(cell);
    //                             world_buf.segments[i] = Segment::Air(Air::new(Vector2::new(x, y)));
    //                         }
    //                     }

    //                     TypeCell::Builder => {
    //                         let wtr = 10.0 * 0.2;
    //                         let gluc = 2.0;

    //                         for j in 0..4 {
    //                             if let Segment::Cell(neighbor) = &app.world.segments[neighbors[j]] {
    //                                 if neighbor.id != world_buf.segments[i].to_cell().id {
    //                                     continue;
    //                                 }

    //                                 if world_buf.segments[i].to_cell().chemical.water
    //                                     > neighbor.chemical.water
    //                                     && 200.0 > neighbor.chemical.water
    //                                 {
    //                                     world_buf.segments[i].to_cell().chemical.water -= wtr;
    //                                     world_buf.segments[neighbors[j]]
    //                                         .to_cell()
    //                                         .chemical
    //                                         .water += wtr;
    //                                 }
    //                             }
    //                         }

    //                         world_buf.segments[i].to_cell().chemical.glucose -= 10.0;
    //                         let parent = world_buf.segments[i].to_cell().clone();
    //                         if parent.chemical.glucose > 80.0 {
    //                             let neighbors = [i, neighbors[0], neighbors[1], neighbors[2]];
    //                             let mut children = [
    //                                 parent.clone(),
    //                                 parent.clone(),
    //                                 parent.clone(),
    //                                 parent.clone(),
    //                             ];

    //                             let gene = parent.genome.0[parent.children[0]];
    //                             children[0].type_cell = gene.type_cell;
    //                             children[0].children = gene.children;

    //                             world_buf.segments[i].to_cell().chemical.glucose = parent.chemical.glucose / gene.get_count_active_division() as f32;

    //                             let gene = parent.genome.0[parent.children[1]];
    //                             children[1].position.x = limit(
    //                                 0.0,
    //                                 width as f32 - 1.0,
    //                                 children[1].position.x as f32 - 1.0,
    //                             ) as usize;
    //                             children[1].type_cell = gene.type_cell;
    //                             children[1].children = gene.children;
    //                             children[1].lifetime = 0;

    //                             let gene = parent.genome.0[parent.children[2]];
    //                             children[2].position.x = limit(
    //                                 0.0,
    //                                 width as f32 - 1.0,
    //                                 children[2].position.x as f32 + 1.0,
    //                             ) as usize;
    //                             children[2].type_cell = gene.type_cell;
    //                             children[2].children = gene.children;
    //                             children[2].lifetime = 0;

    //                             let gene = parent.genome.0[parent.children[3]];
    //                             children[3].position.y += 1;
    //                             children[3].type_cell = gene.type_cell;
    //                             children[3].children = gene.children;
    //                             children[3].lifetime = 0;

    //                             for idx in 0..4 {
    //                                 if parent.children[idx] == 0 {
    //                                     continue;
    //                                 }

    //                                 if idx != 0 {
    //                                     if let Segment::Air(_) = world_buf.segments[neighbors[idx]]
    //                                     {
    //                                         children[idx].chemical.glucose = parent.chemical.glucose / gene.get_count_active_division() as f32;

    //                                         world_buf.segments[neighbors[idx]] =
    //                                             Segment::Cell(children[idx].clone());
    //                                     }
    //                                 } else {
    //                                     world_buf.segments[neighbors[idx]] =
    //                                         Segment::Cell(children[idx].clone());
    //                                 }
    //                             }
    //                         }
    //                     }

    //                     _ => {world_buf.segments[i].to_cell().chemical.glucose -= 5.0;}
    //                 }

    //                 if let Segment::Cell(cell) = &mut world_buf.segments[i] {
    //                     cell.lifetime += 1;

    //                     if cell.lifetime > MAX_LIFE_TIME || cell.chemical.glucose < 0.0 {
    //                         world_buf.segments[i] = Segment::Air(Air::new(cell.position));
    //                     }
    //                 }
    //             }
    //             _ => {}
    //         }

    //         let rect = Rect::new(
    //             x as i32 * SIZE_RECT,
    //             SIZE_RECT * (-(y as i32) + SIZE_WORLD[1] as i32 - 1),
    //             SIZE_RECT as u32,
    //             SIZE_RECT as u32,
    //         );

    //         match &mut app.world.segments[i] {
    //             Segment::Air(air) => {
    //                 canvas.set_draw_color(air.physical.color.to_bytes());
    //                 canvas.fill_rect(rect).unwrap();
    //             }

    //             Segment::Cell(cell) => {
    //                 match cell.type_cell {
    //                     TypeCell::Builder => {
    //                         canvas.set_draw_color((0x54, 0x92, 0x48, 255));
    //                     }
    //                     TypeCell::Conductor => {
    //                         canvas.set_draw_color((0x80, 0x5d, 0x55, 255));
    //                     }
    //                     TypeCell::Consumer => {
    //                         canvas.set_draw_color((0xd4, 0x80, 0x6b, 255));
    //                     }
    //                     TypeCell::Photosynthetic => {
    //                         canvas.set_draw_color((0x80, 0xb8, 0x63, 255));
    //                     }
    //                     TypeCell::Producer => {
    //                         canvas.set_draw_color((0xed, 0xf2, 0xa8, 255));
    //                     }
    //                 }

    //                 canvas.fill_rect(rect).unwrap();
    //             }
    //             Segment::Dirt(block) => {
    //                 block.physical.color = COLOR_DIRT * (1.0 - block.chemical.water / 500.0);

    //                 canvas.set_draw_color(block.physical.color.to_bytes());
    //                 canvas.fill_rect(rect).unwrap();
    //             }
    //         }
    //     }

    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }

    //     app.world = world_buf;
    //     canvas.present();

    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }