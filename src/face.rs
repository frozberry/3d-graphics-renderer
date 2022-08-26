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

// pub type Face = ([usize; 3], Color);
pub type ProjectedFace = ([Vec2; 3], Color);
