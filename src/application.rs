use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::{event::Event, rect::Point};

use crate::display;
use crate::mesh::Mesh;
use crate::vec::vec3::Vec3;
use crate::{camera::Camera, cube::Cube, face::Face, sdl::init_sdl};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub struct Application {
    running: bool,
    sdl: Sdl,
    canvas: Canvas<Window>,
    paused: bool,
    mesh: Mesh,
    camera: Camera,
    cube: bool,
}

impl Application {
    pub fn new() -> Self {
        let (sdl, canvas) = init_sdl();

        let mesh = Mesh::new("./assets/cube.obj");
        let camera = Camera::new(640., Vec3::new(0., 0., 0.));

        Application {
            sdl,
            canvas,
            camera,
            mesh,
            running: true,
            paused: false,
            cube: true,
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                    Input                                   */
    /* -------------------------------------------------------------------------- */
    pub fn input(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.running = false;
                }
                Event::MouseButtonDown { x, y, .. } => {}
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Escape => {
                        self.running = false;
                    }
                    Keycode::P => {
                        self.paused = !self.paused;
                    }
                    Keycode::T => {
                        if self.cube {
                            self.mesh = Mesh::new("./assets/f22.obj");
                            self.cube = false;
                        } else {
                            self.mesh = Mesh::new("./assets/cube.obj");
                            self.cube = true;
                        }
                    }
                    Keycode::Left => {}
                    Keycode::Right => {}
                    _ => {}
                },
                _ => {}
            }
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                   Update                                   */
    /* -------------------------------------------------------------------------- */
    pub fn update(&mut self) {
        self.mesh.update(self.camera);
    }
    /* -------------------------------------------------------------------------- */
    /*                                   Render                                   */
    /* -------------------------------------------------------------------------- */
    pub fn render(&mut self) {
        // self.canvas.set_draw_color(Color::RGB(0, 64, 255));
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        display::render_mesh(&self.mesh, &mut self.canvas);

        self.canvas.present();
    }

    pub fn cap_fps(&self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn paused(&self) -> bool {
        self.paused
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
