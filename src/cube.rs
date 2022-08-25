use crate::{
    application::{HEIGHT, WIDTH},
    camera::Camera,
    face::{Face, ProjectedFace},
    vec::{vec2::Vec2, vec3::Vec3},
};

pub struct Cube {
    rotation: Vec3,
    verticies: [Vec3; 8],
    faces: [Face; 12],
    pub projected_faces: [ProjectedFace; 12],
}

impl Cube {
    pub fn new() -> Self {
        let verticies = [
            Vec3::new(-1., -1., -1.),
            Vec3::new(-1., 1., -1.),
            Vec3::new(1., 1., -1.),
            Vec3::new(1., -1., -1.),
            Vec3::new(1., 1., 1.),
            Vec3::new(1., -1., 1.),
            Vec3::new(-1., 1., 1.),
            Vec3::new(-1., -1., 1.),
        ];

        let faces = [
            [1, 2, 3],
            [1, 3, 4],
            [4, 3, 5],
            [4, 5, 6],
            [6, 5, 7],
            [6, 7, 8],
            [8, 7, 2],
            [8, 2, 1],
            [2, 7, 5],
            [2, 5, 3],
            [6, 8, 1],
            [6, 1, 4],
        ];

        let rotation = Vec3::init();
        let projected_faces = [[Vec2::init(); 3]; 12];

        Cube {
            rotation,
            verticies,
            faces,
            projected_faces,
        }
    }

    pub fn update(&mut self, camera: Camera) {
        self.rotation.x = 10.;
        self.rotation.y += 0.01;
        self.rotation.z += 0.01;

        self.project_faces(camera);
    }

    fn project_faces(&mut self, camera: Camera) {
        for i in 0..self.faces.len() {
            let face = self.faces[i];

            for (j, v_index) in face.iter().enumerate() {
                let vertex = self.verticies[v_index - 1];
                let rotated = vertex.rotate(self.rotation);
                let transformed = rotated - camera.pos;

                let projected = transformed.project(camera.fov);
                let centered = projected.centered();

                self.projected_faces[i][j] = centered
            }
        }
    }
}
