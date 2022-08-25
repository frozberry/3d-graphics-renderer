use std::{fmt::Display, ops::Sub};

use crate::application::{HEIGHT, WIDTH};

use super::vec2::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn init() -> Self {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn rotate(&self, rotation: Vec3) -> Vec3 {
        let mut rotated = *self;
        rotated = rotated.rot_x(rotation.x);
        rotated = rotated.rot_y(rotation.y);
        rotated = rotated.rot_z(rotation.z);
        rotated
    }

    pub fn rot_x(&self, angle: f32) -> Vec3 {
        let x = self.x;
        let y = self.y * angle.cos() - self.z * angle.sin();
        let z = self.y * angle.sin() + self.z * angle.cos();
        Vec3::new(x, y, z)
    }

    pub fn rot_y(&self, angle: f32) -> Vec3 {
        let x = self.x * angle.cos() - self.z * angle.sin();
        let y = self.y;
        let z = self.x * angle.sin() + self.z * angle.cos();
        Vec3::new(x, y, z)
    }

    pub fn rot_z(&self, angle: f32) -> Vec3 {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        let z = self.z;
        Vec3::new(x, y, z)
    }

    pub fn project(&self, fov: f32) -> Vec2 {
        let x = self.x * fov / self.z;
        let y = self.y * fov / self.z;
        Vec2::new(x, y)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
