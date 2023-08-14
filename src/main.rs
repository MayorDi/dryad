#![cfg(target_os = "windows")]

use dryad::app::App;

pub fn main() {
    App::init().run();
}
