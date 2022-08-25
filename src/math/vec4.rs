pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub fn init() -> Self {
        Vec4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 0.,
        }
    }
}
