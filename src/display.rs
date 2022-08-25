use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{mesh::Mesh, vec::vec2::Vec2};

pub fn render_mesh(mesh: &Mesh, canvas: &mut Canvas<Window>) {
    for projected_face in &mesh.projected_faces {
        for i in 0..3 {
            let a = projected_face[i];
            let b = projected_face[(i + 1) % 3];

            // draw_line(a, b, canvas);
            // canvas.filled_trigon(x1, y1, x2, y2, x3, y3, color)

            // canvas
            //     .filled_circle(a.x as i16, a.y as i16, 2, Color::GREEN)
            //     .unwrap();
        }
        let a = projected_face[0];
        let b = projected_face[1];
        let c = projected_face[2];
        canvas
            .filled_trigon(
                a.x as i16,
                a.y as i16,
                b.x as i16,
                b.y as i16,
                c.x as i16,
                c.y as i16,
                Color::GREEN,
            )
            .unwrap();
    }
}

pub fn draw_line(a: Vec2, b: Vec2, canvas: &mut Canvas<Window>) {
    let ab = b - a;

    let len = if ab.x.abs() > ab.y.abs() {
        ab.x.abs()
    } else {
        ab.y.abs()
    };

    let x_increment = ab.x / len;
    let y_increment = ab.y / len;

    let mut cur_x = a.x;
    let mut cur_y = a.y;

    for _ in 0..len as i32 {
        let rect = Rect::new(cur_x as i32, cur_y as i32, 1, 1);
        canvas.set_draw_color(Color::RGB(0, 100, 0));
        canvas.fill_rect(rect).unwrap();

        cur_x += x_increment;
        cur_y += y_increment
    }
}
