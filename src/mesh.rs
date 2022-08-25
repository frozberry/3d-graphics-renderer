use std::f32::consts::PI;

use sdl2::pixels::Color;

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
    pub fn from_obj(obj_path: &str) -> Self {
        let (verticies, faces) = parse_obj(obj_path);
        let rotation = Vec3::init();
        let projected_faces = vec![([Vec2::init(); 3], Color::WHITE); faces.len()];

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
        let projected_faces = vec![([Vec2::init(); 3], Color::WHITE); faces.len()];

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
            ([1, 2, 3], Color::WHITE),
            ([1, 3, 4], Color::WHITE),
            ([4, 3, 5], Color::RED),
            ([4, 5, 6], Color::RED),
            ([6, 5, 7], Color::GREEN),
            ([6, 7, 8], Color::GREEN),
            ([8, 7, 2], Color::BLUE),
            ([8, 2, 1], Color::BLUE),
            ([2, 7, 5], Color::MAGENTA),
            ([2, 5, 3], Color::MAGENTA),
            ([6, 8, 1], Color::YELLOW),
            ([6, 1, 4], Color::YELLOW),
        ];

        mesh.verticies = verticies;
        mesh.faces = faces;
        mesh.projected_faces = vec![];
        mesh
    }

    pub fn update(&mut self, camera: Camera, cull: bool) {
        // self.rotation.x = -PI;
        self.rotation.x += 0.02;
        self.rotation.y += 0.02;

        self.faces_to_projection(camera, cull);
    }

    fn faces_to_projection(&mut self, camera: Camera, cull: bool) {
        // Reset the last frames faces
        self.projected_faces = vec![];

        let mut projected_faces: Vec<(ProjectedFace, f32)> = vec![];

        for face in &self.faces {
            let transformed_verticies: [Vec3; 3] = face
                .0
                .iter()
                .map(|v_index| {
                    let vertex = self.verticies[v_index - 1];
                    let mut rotated_vertex = vertex.rotate(self.rotation);
                    rotated_vertex.z += 5.;
                    rotated_vertex
                })
                .collect::<Vec<Vec3>>()
                .try_into()
                .unwrap();

            if cull && Self::can_cull(transformed_verticies, camera) {
                continue;
            }

            let average_depth = transformed_verticies
                .iter()
                .fold(0., |acc, vertex| acc + vertex.z)
                / 3.;

            let projected_face: [Vec2; 3] = transformed_verticies
                .iter()
                .map(|vertex| vertex.project(camera.fov).centered())
                .collect::<Vec<Vec2>>()
                .try_into()
                .unwrap();

            // Store the depth for sorting
            let face_with_depth = ((projected_face, face.1), average_depth);

            projected_faces.push(face_with_depth);
        }

        // Sort the projected faces by their depth
        projected_faces.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let without_depths = projected_faces.iter().map(|tuple| tuple.0).collect();
        self.projected_faces = without_depths;
    }

    // Checks if a face can be skipped from backface culling
    fn can_cull(vertices: [Vec3; 3], camera: Camera) -> bool {
        let va = vertices[0];
        let vb = vertices[1];
        let vc = vertices[2];

        let ab = vb - va;
        let ac = vc - va;

        let normal = ab.cross(ac);

        let camera_ray = camera.pos - va;
        let dot_normal_camera = normal.dot(camera_ray);

        dot_normal_camera < 0.
    }
}
