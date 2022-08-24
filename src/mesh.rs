use std::f32::consts::PI;

use crate::{
    application::{HEIGHT, WIDTH},
    camera::Camera,
    face::{Face, ProjectedFace},
    parser::parse_obj,
    vec3::{Vec2, Vec3},
};

pub struct Mesh {
    rotation: Vec3,
    verticies: Vec<Vec3>,
    faces: Vec<Face>,
    pub projected_faces: Vec<ProjectedFace>,
}

impl Mesh {
    pub fn new(obj_path: &str) -> Self {
        let (verticies, faces) = parse_obj(obj_path);
        let rotation = Vec3::init();
        let projected_faces = vec![[Vec2::init(); 3]; faces.len()];

        Mesh {
            rotation,
            verticies,
            faces,
            projected_faces,
        }
    }

    pub fn init() -> Self {
        let rotation = Vec3::init();

        let verticies = vec![];
        let faces = vec![];
        let projected_faces = vec![[Vec2::init(); 3]];

        Mesh {
            rotation,
            verticies,
            faces,
            projected_faces,
        }
    }

    pub fn new_cube() -> Self {
        let mut mesh = Mesh::init();
        let verticies = vec![
            Vec3::new(-1., -1., -1.),
            Vec3::new(-1., 1., -1.),
            Vec3::new(1., 1., -1.),
            Vec3::new(1., -1., -1.),
            Vec3::new(1., 1., 1.),
            Vec3::new(1., -1., 1.),
            Vec3::new(-1., 1., 1.),
            Vec3::new(-1., -1., 1.),
        ];

        let faces = vec![
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

        mesh.verticies = verticies;
        mesh.faces = faces;
        mesh.projected_faces = vec![[Vec2::init(); 3]; mesh.faces.len()];
        mesh
    }

    pub fn update(&mut self, camera: Camera) {
        // self.rotation.x = -PI;
        self.rotation.x += 0.02;
        // self.rotation.y += 0.02;

        self.project_faces(camera);
    }

    fn project_faces(&mut self, camera: Camera) {
        for i in 0..self.faces.len() {
            let face = self.faces[i];

            for (j, v_index) in face.iter().enumerate() {
                let vertex = self.verticies[v_index - 1];
                let rotated_vertex = vertex.rotate(self.rotation);
                let transformed_vertex = rotated_vertex - camera.pos;

                let projected_vertex = transformed_vertex.project(camera.fov);
                let centered_vertex = projected_vertex.centered();

                self.projected_faces[i][j] = centered_vertex
            }
        }
    }
}
