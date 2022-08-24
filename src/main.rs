extern crate sdl2;

use std::time::Duration;

use application::Application;

mod application;
mod camera;
mod sdl;
mod vec3;

fn main() {
    let mut application = Application::new();

    while application.running() {
        application.input();
        application.update();
        application.render();
        application.cap_fps();
    }
}
