use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{
    face::ProjectedFace,
    math::vec2::Vec2,
    mesh::Mesh,
    render_mode::{self, RenderMode},
};

pub fn render_mesh(mesh: &Mesh, render_mode: RenderMode, canvas: &mut Canvas<Window>) {
    for projected_face in &mesh.projected_faces {
        match render_mode {
            RenderMode::Wire => {
                draw_face_wire(projected_face, Color::GREEN, canvas);
            }
            RenderMode::WireVertex => {
                draw_face_wire(projected_face, Color::RGB(0, 200, 0), canvas);
                draw_verticies(projected_face, canvas);
            }
            RenderMode::FillTriangle => {
                fill_face(projected_face, canvas);
            }
            RenderMode::FillTriangleWire => {
                fill_face(projected_face, canvas);
                draw_face_wire(projected_face, Color::BLACK, canvas);
            }
        }
    }
}

fn draw_face_wire(projected_face: &ProjectedFace, color: Color, canvas: &mut Canvas<Window>) {
    for i in 0..3 {
        let a = projected_face.verticies[i];
        let b = projected_face.verticies[(i + 1) % 3];
        canvas
            .line(a.x as i16, a.y as i16, b.x as i16, b.y as i16, color)
            .unwrap();
    }
}

fn draw_verticies(projected_face: &ProjectedFace, canvas: &mut Canvas<Window>) {
    for i in 0..3 {
        let a = projected_face.verticies[i];
        canvas
            .filled_circle(a.x as i16, a.y as i16, 2, Color::GREEN)
            .unwrap();
    }
}

fn fill_face(projected_face: &ProjectedFace, canvas: &mut Canvas<Window>) {
    let a = projected_face.verticies[0];
    let b = projected_face.verticies[1];
    let c = projected_face.verticies[2];

    let color = calc_light_intensity(projected_face.color, projected_face.intensity);
    canvas
        .filled_trigon(
            a.x as i16, a.y as i16, b.x as i16, b.y as i16, c.x as i16, c.y as i16, color,
        )
        .unwrap();
}

fn calc_light_intensity(color: Color, intensity: f32) -> Color {
    let r = color.r as f32 * intensity;
    let g = color.g as f32 * intensity;
    let b = color.b as f32 * intensity;
    Color::RGB(r as u8, g as u8, b as u8)
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
