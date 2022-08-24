use sdl2::{rect::Rect, render::Canvas, video::Window};

use crate::vec3::Vec2;

pub fn draw_line(a: Vec2, b: Vec2, canvas: &mut Canvas<Window>) {
    let ab = b - a;

    let len = if ab.x.abs() > ab.y.abs() { ab.x } else { ab.y };

    let x_increment = ab.x / len;
    let y_increment = ab.y / len;

    let mut cur_x = a.x;
    let mut cur_y = a.y;

    for _ in 0..len as i32 {
        let rect = Rect::new(cur_x as i32, cur_y as i32, 1, 1);
        canvas.fill_rect(rect).unwrap();

        cur_x += x_increment;
        cur_y += y_increment
    }
}
