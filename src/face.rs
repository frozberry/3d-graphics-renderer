// #[derive(Clone, Copy)]
// pub struct Face {
//     pub a: usize,
//     pub b: usize,
//     pub c: usize,
// }

// impl Face {
//     pub fn new(a: usize, b: usize, c: usize) -> Self {
//         Face { a, b, c }
//     }
// }

use std::fmt::Display;

use crate::vec3::Vec2;

pub type Face = [usize; 3];
pub type ProjectedFace = [Vec2; 3];
