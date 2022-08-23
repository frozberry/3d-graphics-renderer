use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub struct Application {
    sdl: Sdl,
    canvas: Canvas<Window>,
    r: u8,
    g: u8,
    b: u8,
    x: i32,
    y: i32,
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Graphics", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        Application {
            sdl: sdl_context,
            canvas,
            running: true,
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
                Event::KeyDown { keycode, .. } => match keycode.unwrap() {
                    Keycode::Escape => {
                        self.running = false;
                    }
                    Keycode::A => println!("hi"),
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

    /* -------------------------------------------------------------------------- */
    /*                                   Update                                   */
    /* -------------------------------------------------------------------------- */
    pub fn update(&mut self) {
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

        self.canvas.present();
    }

    pub fn running(&self) -> bool {
        self.running
    }
}
