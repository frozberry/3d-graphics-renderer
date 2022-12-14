extern crate sdl2;

use application::Application;

mod application;
mod camera;
mod display;
mod face;
mod light;
mod math;
mod mesh;
mod parser;
mod render_mode;
mod sdl;

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
