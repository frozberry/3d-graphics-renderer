use crate::math::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct Light {
    pub direction: Vec3,
}

impl Light {
    pub fn new() -> Self {
        Light {
            direction: Vec3::new(0., 0., 1.),
            // direction: Vec3::new(),
        }
    }
}
