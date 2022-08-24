use crate::vec3::Vec3;

pub struct Camera {
    pub fov: f32,
    pub pos: Vec3,
}

impl Camera {
    pub fn new(fov: f32, pos: Vec3) -> Self {
        Camera { fov, pos }
    }
}
