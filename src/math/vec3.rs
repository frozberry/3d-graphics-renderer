use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

use crate::application::{HEIGHT, WIDTH};

use super::{vec2::Vec2, vec4::Vec4};

#[derive(Clone, Copy)]
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

    pub fn cross(&self, b: Vec3) -> Vec3 {
        let x = self.y * b.z - self.z * b.y;
        let y = self.z * b.x - self.x * b.z;
        let z = self.x * b.y - self.y * b.x;
        Vec3::new(x, y, z)
    }

    pub fn dot(&self, b: Vec3) -> f32 {
        (self.x * b.x) + (self.y * b.y) + (self.z * b.z)
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

    fn magnitude(&self) -> f32 {
        (f32::powf(self.x, 2.) + f32::powf(self.y, 2.) + f32::powf(self.z, 2.)).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        let mut result = Vec3::init();
        let length = self.magnitude();
        if length != 0. {
            result.x = self.x / length;
            result.y = self.y / length;
            result.z = self.z / length;
        }
        result
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl From<Vec4> for Vec3 {
    fn from(v: Vec4) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}
