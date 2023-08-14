#![cfg(target_os = "windows")]

use dryad::app::App;

pub fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::builder().init();
    App::init().run();
}
