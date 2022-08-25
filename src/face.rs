use sdl2::pixels::Color;

use crate::math::vec2::Vec2;

pub type Face = ([usize; 3], Color);
pub type ProjectedFace = ([Vec2; 3], Color);
