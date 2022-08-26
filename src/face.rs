use sdl2::pixels::Color;

use crate::math::vec2::Vec2;

pub struct Face {
    pub verticies_idx: [usize; 3],
    pub color: Color,
}

impl Face {
    pub fn new(verticies_idx: [usize; 3], color: Color) -> Self {
        Face {
            verticies_idx,
            color,
        }
    }
}

#[derive(Clone)]
pub struct ProjectedFace {
    pub verticies: [Vec2; 3],
    pub color: Color,
    pub depth: f32,
}

impl ProjectedFace {
    pub fn new(verticies: [Vec2; 3], color: Color, depth: f32) -> Self {
        ProjectedFace {
            verticies,
            color,
            depth,
        }
    }
}
