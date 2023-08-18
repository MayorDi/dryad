#![cfg(test)]

use crate::{app::App, world::*};

#[test]
fn generate_world() {
    let mut world: World = World::new();
    world.generate();
}

#[test]
fn init_app() {
    App::init();
}

#[test]
fn init_and_run_app() {
    App::init().run();
}
