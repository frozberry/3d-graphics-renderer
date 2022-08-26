use std::f32::consts::PI;
use std::time::Duration;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use sdl2::{event::Event, rect::Point};

use crate::display;
use crate::light::Light;
use crate::math::mat4::Mat4;
use crate::math::vec3::Vec3;
use crate::mesh::Mesh;
use crate::render_mode::RenderMode;
use crate::{camera::Camera, face::Face, sdl::init_sdl};

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
    render_mode: RenderMode,
    cull: bool,
    projection_matrix: Mat4,
    light: Light,
}

impl Application {
    pub fn new() -> Self {
        let (sdl, canvas) = init_sdl();

        let fov = PI / 3.0;
        let aspect = HEIGHT as f32 / WIDTH as f32;
        let znear = 0.1;
        let zfar = 100.;
        let projection_matrix = Mat4::perspective(fov, aspect, znear, zfar);

        // let mesh = Mesh::new_cube(projection_matrix);
        let mesh = Mesh::from_obj("./assets/f22.obj", projection_matrix);
        let camera = Camera::new(640., Vec3::new(0., 0., 0.));
        let light = Light::new();

        Application {
            sdl,
            canvas,
            camera,
            mesh,
            running: true,
            paused: false,
            cube: false,
            render_mode: RenderMode::FillTriangle,
            cull: true,
            projection_matrix,
            light,
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
                    Keycode::C => self.cull = !self.cull,
                    Keycode::Q => self.render_mode = RenderMode::FillTriangle,
                    Keycode::W => self.render_mode = RenderMode::FillTriangleWire,
                    Keycode::E => self.render_mode = RenderMode::Wire,
                    Keycode::R => self.render_mode = RenderMode::WireVertex,
                    Keycode::T => {
                        if self.cube {
                            self.mesh = Mesh::from_obj("./assets/f22.obj", self.projection_matrix);
                            self.cube = false;
                        } else {
                            self.mesh = Mesh::new_cube(self.projection_matrix);
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
        self.mesh.update(self.camera, self.light, self.cull);
    }
    /* -------------------------------------------------------------------------- */
    /*                                   Render                                   */
    /* -------------------------------------------------------------------------- */
    pub fn render(&mut self) {
        // self.canvas.set_draw_color(Color::RGB(0, 64, 255));
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        display::render_mesh(&self.mesh, self.render_mode, &mut self.canvas);

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
