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
use crate::{
    camera::Camera,
    cube::Cube,
    face::Face,
    sdl::init_sdl,
    vec3::{Vec2, Vec3},
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub struct Application {
    running: bool,
    sdl: Sdl,
    canvas: Canvas<Window>,

    paused: bool,

    mesh: Mesh,
    camera: Camera,
}

impl Application {
    pub fn new() -> Self {
        let (sdl, canvas) = init_sdl();

        let mesh = Mesh::new_cube();
        let camera = Camera::new(640., Vec3::new(0., 0., -5.));

        Application {
            sdl,
            canvas,
            camera,
            mesh,
            running: true,
            paused: false,
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
        self.canvas.set_draw_color(Color::RGB(0, 64, 255));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));

        for projected_face in &self.mesh.projected_faces {
            for i in 0..3 {
                let a = projected_face[i];
                let b = projected_face[(i + 1) % 3];

                display::draw_line(a, b, &mut self.canvas);

                // Could use ruilt in SDL function
                // let start = Point::new(a.x as i32, a.y as i32);
                // let end = Point::new(b.x as i32, b.y as i32);
                // self.canvas.draw_line(start, end).unwrap();
            }
        }

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
