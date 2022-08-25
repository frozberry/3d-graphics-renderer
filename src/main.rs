extern crate sdl2;

use application::Application;

mod application;
mod camera;
mod cube;
mod display;
mod face;
mod mesh;
mod parser;
mod sdl;
mod vec;
mod render_mode;

fn main() {
    let mut application = Application::new();

    while application.running() {
        application.input();

        if !application.paused() {
            application.update();
            application.render();
            application.cap_fps();
        }
    }
}
