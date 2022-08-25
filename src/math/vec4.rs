use super::vec3::Vec3;
use std::fmt::{Debug, Display};

pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }
    pub fn init() -> Self {
        Vec4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
    }
}

impl From<Vec3> for Vec4 {
    fn from(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z, 1.)
    }
}

impl Debug for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, y: {}, z: {}, w: {}",
            self.x, self.y, self.z, self.w
        )
    }
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x: {}, y: {}, z: {}, w:{}",
            self.x, self.y, self.z, self.w
        )
    }
}
