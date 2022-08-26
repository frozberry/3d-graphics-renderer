use crate::math::vec3::Vec3;

pub struct Light {
    direction: Vec3,
}

impl Light {
    pub fn new() -> Self {
        Light {
            direction: Vec3::new(0., 0., 0.),
        }
    }
}
