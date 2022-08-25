use std::ops::{self};

use crate::application::{HEIGHT, WIDTH};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn init() -> Self {
        Vec2 { x: 0., y: 0. }
    }

    pub fn centered(&self) -> Vec2 {
        Vec2::new(self.x + WIDTH as f32 / 2., self.y + HEIGHT as f32 / 2.)
    }

    pub fn add(&mut self, vec: Vec2) {
        self.x += vec.x;
        self.y += vec.y
    }

    pub fn subtract(&mut self, vec: Vec2) {
        self.x -= vec.x;
        self.y -= vec.y
    }

    pub fn scale(&mut self, n: f32) {
        self.x *= n;
        self.y *= n;
    }

    pub fn rotate(&self, angle: f32) -> Vec2 {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        Vec2::new(x, y)
    }

    pub fn magnitude(&self) -> f32 {
        (f32::powf(self.x, 2.) + f32::powf(self.y, 2.)).sqrt()
    }

    pub fn magnitude_squared(&self) -> f32 {
        f32::powf(self.x, 2.) + f32::powf(self.y, 2.)
    }

    pub fn unit_vector(&self) -> Vec2 {
        let mut result = Vec2::new(0., 0.);
        let length = self.magnitude();
        if length != 0. {
            result.x = self.x / length;
            result.y = self.y / length;
        }
        result
    }

    pub fn normal(&self) -> Vec2 {
        let mut perpendicular = Vec2::new(self.y, -self.x);
        perpendicular.unit_vector()
    }

    pub fn dot(&self, v: Vec2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    pub fn cross(&self, v: Vec2) -> f32 {
        self.x * v.y - self.y * v.x
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

// Scale
impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

// Dot product
impl ops::Mul<Vec2> for Vec2 {
    type Output = f32;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2::new(self.x * -1., self.y * -1.)
    }
}
