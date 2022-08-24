use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::{
    camera::Camera,
    sdl::init_sdl,
    vec3::{Vec2, Vec3},
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

pub struct Application {
    sdl: Sdl,
    canvas: Canvas<Window>,
    r: u8,
    g: u8,
    b: u8,

    x: i32,
    y: i32,

    clockwise: bool,
    running: bool,

    cube_rotation: Vec3,
    cube_points: Vec<Vec3>,
    projected_points: Vec<Vec2>,
    camera: Camera,
}

impl Application {
    pub fn new() -> Self {
        let (sdl, canvas) = init_sdl();

        let mut cube_points = vec![];
        for i in 0..8 {
            for j in 0..8 {
                for k in 0..8 {
                    let x = -1. + i as f32 * 0.25;
                    let y = -1. + j as f32 * 0.25;
                    let z = -1. + k as f32 * 0.25;
                    let point = Vec3::new(x, y, z);
                    cube_points.push(point)
                }
            }
        }

        let cube_rotation = Vec3::init();
        let projected_points = vec![];

        let camera = Camera::new(640., Vec3::new(0., 0., -5.));

        Application {
            sdl,
            canvas,
            camera,
            running: true,
            cube_points,
            cube_rotation,
            projected_points,
            clockwise: true,
            r: 0,
            g: 64,
            b: 255,
            x: 0,
            y: 0,
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
                Event::MouseButtonDown { x, y, .. } => {
                    self.x = x;
                    self.y = y;
                }
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Escape => {
                        self.running = false;
                    }
                    Keycode::A => self.clockwise = !self.clockwise,
                    Keycode::Left => {
                        self.r = (self.r + 10) % u8::MAX;
                        self.b = (self.r - 10) % u8::MAX;
                    }
                    Keycode::Right => {
                        self.r = (self.r - 10) % u8::MAX;
                        self.b = (self.r + 10) % u8::MAX;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn project(point: Vec3) -> Vec2 {
        let x = point.x * 640. / point.z;
        let y = point.y * 640. / point.z;
        Vec2::new(x, y)
    }

    /* -------------------------------------------------------------------------- */
    /*                                   Update                                   */
    /* -------------------------------------------------------------------------- */
    pub fn update(&mut self) {
        self.cube_rotation.x = 10.;
        self.cube_rotation.y += if self.clockwise { 0.01 } else { -0.01 };
        // self.cube_rotation.z += 0.01;

        for i in 0..self.cube_points.len() {
            let mut point = self.cube_points[i];
            point = point.rot_x(self.cube_rotation.x);
            point = point.rot_y(self.cube_rotation.y);
            point = point.rot_z(self.cube_rotation.z);

            point.z -= self.camera.pos.z;

            let projected_point = Self::project(point);
            self.projected_points.push(projected_point);
        }

        self.x += 2;
        self.y += 1;

        if self.x > WIDTH as i32 {
            self.x = 0;
        }

        if self.y > HEIGHT as i32 {
            self.y = 0
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                   Render                                   */
    /* -------------------------------------------------------------------------- */
    pub fn render(&mut self) {
        self.canvas
            .set_draw_color(Color::RGB(self.r, self.g, self.b));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        let rect = Rect::new(self.x, self.y, 10, 10);
        self.canvas.fill_rect(rect).unwrap();

        for point in self.projected_points.clone() {
            let w = WIDTH as i32 / 2;
            let h = HEIGHT as i32 / 2;
            let rect = Rect::new(point.x as i32 + w, point.y as i32 + h, 4, 4);
            self.canvas.draw_rect(rect).unwrap();
        }
        self.projected_points = vec![];

        self.canvas.present();
    }

    pub fn cap_fps(&self) {
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    pub fn running(&self) -> bool {
        self.running
    }
}
