use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    face::ProjectedFace,
    mesh::Mesh,
    render_mode::{self, RenderMode},
    math::vec2::Vec2,
};

pub fn render_mesh(mesh: &Mesh, render_mode: RenderMode, canvas: &mut Canvas<Window>) {
    for projected_face in &mesh.projected_faces {
        match render_mode {
            RenderMode::Wire => {
                draw_face_wire(projected_face, canvas);
            }
            RenderMode::WireVertex => {
                draw_face_wire(projected_face, canvas);
                draw_verticies(projected_face, canvas);
            }
            RenderMode::FillTriangle => {
                fill_face(projected_face, canvas);
            }
            RenderMode::FillTriangleWire => {
                fill_face(projected_face, canvas);
                draw_face_wire(projected_face, canvas);
            }
        }
    }
}

fn draw_face_wire(projected_face: &ProjectedFace, canvas: &mut Canvas<Window>) {
    for i in 0..3 {
        let a = projected_face.0[i];
        let b = projected_face.0[(i + 1) % 3];
        canvas
            .line(
                a.x as i16,
                a.y as i16,
                b.x as i16,
                b.y as i16,
                projected_face.1,
            )
            .unwrap();
    }
}

fn draw_verticies(projected_face: &ProjectedFace, canvas: &mut Canvas<Window>) {
    for i in 0..3 {
        let a = projected_face.0[i];
        canvas
            .filled_circle(a.x as i16, a.y as i16, 2, projected_face.1)
            .unwrap();
    }
}

fn fill_face(projected_face: &ProjectedFace, canvas: &mut Canvas<Window>) {
    let a = projected_face.0[0];
    let b = projected_face.0[1];
    let c = projected_face.0[2];
    canvas
        .filled_trigon(
            a.x as i16,
            a.y as i16,
            b.x as i16,
            b.y as i16,
            c.x as i16,
            c.y as i16,
            projected_face.1,
        )
        .unwrap();
}

// pub fn draw_line(a: Vec2, b: Vec2, canvas: &mut Canvas<Window>) {
//     let ab = b - a;

//     let len = if ab.x.abs() > ab.y.abs() {
//         ab.x.abs()
//     } else {
//         ab.y.abs()
//     };

//     let x_increment = ab.x / len;
//     let y_increment = ab.y / len;

//     let mut cur_x = a.x;
//     let mut cur_y = a.y;

//     for _ in 0..len as i32 {
//         let rect = Rect::new(cur_x as i32, cur_y as i32, 1, 1);
//         canvas.set_draw_color(Color::RGB(0, 100, 0));
//         canvas.fill_rect(rect).unwrap();

//         cur_x += x_increment;
//         cur_y += y_increment
//     }
// }
