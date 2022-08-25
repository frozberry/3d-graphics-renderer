use std::ops::Mul;

use super::vec4::Vec4;

struct Mat4 {
    m: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Mat4 {
        // | 1 0 0 0 |
        // | 0 1 0 0 |
        // | 0 0 1 0 |
        // | 0 0 0 1 |
        Mat4 {
            m: [
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.],
            ],
        }
    }
    pub fn scale(x: f32, y: f32, z: f32) -> Mat4 {
        // | sx  0  0  0 |
        // |  0 sy  0  0 |
        // |  0  0 sz  0 |
        // |  0  0  0  1 |
        let mut m = Self::identity();
        m.m[0][0] = x;
        m.m[1][2] = y;
        m.m[2][2] = z;
        m
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Mat4 {
        let mut m = Self::identity();
        m.m[0][3] = x;
        m.m[1][3] = y;
        m.m[2][3] = z;
        m
    }

    pub fn rotation_x(angle: f32) -> Mat4 {
        // | 1  0  0  0 |
        // | 0  c -s  0 |
        // | 0  s  c  0 |
        // | 0  0  0  1 |
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.m[1][1] = c;
        m.m[1][2] = -s;
        m.m[2][1] = s;
        m.m[2][2] = c;
        m
    }

    pub fn rotation_y(angle: f32) -> Mat4 {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        // |  c  0  s  0 |
        // |  0  1  0  0 |
        // | -s  0  c  0 |
        // |  0  0  0  1 |
        m.m[0][0] = c;
        m.m[0][2] = s;
        m.m[2][0] = -s;
        m.m[2][2] = c;
        m
    }

    pub fn rotation_z(angle: f32) -> Mat4 {
        // | c -s  0  0 |
        // | s  c  0  0 |
        // | 0  0  1  0 |
        // | 0  0  0  1 |
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.m[0][0] = c;
        m.m[0][1] = -s;
        m.m[1][0] = s;
        m.m[1][1] = c;
        m
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                m.m[i][j] = self.m[i][0] * rhs.m[0][j]
                    + self.m[i][1] * rhs.m[1][j]
                    + self.m[i][2] * rhs.m[2][j]
                    + self.m[i][3] * rhs.m[3][j];
            }
        }
        m
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mut result = Vec4::init();
        result.x = self.m[0][0] * rhs.x
            + self.m[0][1] * rhs.y
            + self.m[0][2] * rhs.z
            + self.m[0][3] * rhs.w;
        result.y = self.m[1][0] * rhs.x
            + self.m[1][1] * rhs.y
            + self.m[1][2] * rhs.z
            + self.m[1][3] * rhs.w;
        result.z = self.m[2][0] * rhs.x
            + self.m[2][1] * rhs.y
            + self.m[2][2] * rhs.z
            + self.m[2][3] * rhs.w;
        result.w = self.m[3][0] * rhs.x
            + self.m[3][1] * rhs.y
            + self.m[3][2] * rhs.z
            + self.m[3][3] * rhs.w;
        result
    }
}
