use std::f32::consts::PI;

use crate::{
    application::{HEIGHT, WIDTH},
    camera::Camera,
    face::{Face, ProjectedFace},
    parser::parse_obj,
    vec::{vec2::Vec2, vec3::Vec3},
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
        mesh.projected_faces = vec![];
        mesh
    }

    pub fn update(&mut self, camera: Camera) {
        // self.rotation.x = -PI;
        self.rotation.x += 0.02;
        self.rotation.y += 0.02;

        self.projected_faces = vec![];
        self.project_faces(camera);
    }

    fn project_faces(&mut self, camera: Camera) {
        for i in 0..self.faces.len() {
            let face = self.faces[i];

            let mut transformed_verticies = [Vec3::init(); 3];
            for (j, v_index) in face.iter().enumerate() {
                let vertex = self.verticies[v_index - 1];
                let mut rotated_vertex = vertex.rotate(self.rotation);
                rotated_vertex.z += 5.0;

                transformed_verticies[j] = rotated_vertex;
            }

            // Check backface culling

            let va = transformed_verticies[0];
            let vb = transformed_verticies[1];
            let vc = transformed_verticies[2];

            let ab = vb - va;
            let ac = vc - va;

            let normal = ab.cross(ac);

            let camera_ray = camera.pos - va;
            let dot_normal_camera = normal.dot(camera_ray);

            if dot_normal_camera >= 0. {
                let mut projected_triangle = [Vec2::init(); 3];
                for (j, transformed_vertex) in transformed_verticies.iter().enumerate() {
                    let projected_vertex = transformed_vertex.project(camera.fov);
                    let centered_vertex = projected_vertex.centered();

                    projected_triangle[j] = centered_vertex
                }
                self.projected_faces.push(projected_triangle);
            }
        }
    }
}
