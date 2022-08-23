pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
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
        let y = self.x * angle.sin() + self.z * angle.cos();
        let z = self.z;
        Vec3::new(x, y, z)
    }
}
