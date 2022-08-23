extern crate sdl2;

use std::time::Duration;

use application::Application;

mod application;

mod vec3;
fn main() {
    let mut application = Application::new();

    while application.running() {
        application.input();
        application.update();
        application.render();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
