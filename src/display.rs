use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{mesh::Mesh, vec3::Vec2};

pub fn render_mesh(mesh: &Mesh, canvas: &mut Canvas<Window>) {
    for projected_face in &mesh.projected_faces {
        for i in 0..3 {
            let a = projected_face[i];
            let b = projected_face[(i + 1) % 3];

            draw_line(a, b, canvas);

            canvas
                .filled_circle(a.x as i16, a.y as i16, 2, Color::GREEN)
                .unwrap();
        }
    }
}

pub fn draw_line(a: Vec2, b: Vec2, canvas: &mut Canvas<Window>) {
    let ab = b - a;

    let len = if ab.x.abs() > ab.y.abs() { ab.x } else { ab.y };

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
