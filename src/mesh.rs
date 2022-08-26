use std::f32::consts::PI;

use sdl2::pixels::Color;

use crate::{
    application::{HEIGHT, WIDTH},
    camera::Camera,
    face::{Face, ProjectedFace},
    math::{mat4::Mat4, vec2::Vec2, vec3::Vec3, vec4::Vec4},
    parser::parse_obj,
};

pub struct Mesh {
    rotation: Vec3,
    translation: Vec3,
    scale: Vec3,
    verticies: Vec<Vec3>,
    faces: Vec<Face>,
    pub projected_faces: Vec<ProjectedFace>,
    projection_matrix: Mat4,
}

impl Mesh {
    pub fn from_obj(obj_path: &str, projection_matrix: Mat4) -> Self {
        let (verticies, faces) = parse_obj(obj_path);
        let rotation = Vec3::init();
        let translation = Vec3::init();
        let scale = Vec3::new(1., 1., 1.);
        let projected_faces = vec![([Vec2::init(); 3], Color::WHITE); faces.len()];

        Mesh {
            rotation,
            translation,
            scale,
            verticies,
            faces,
            projected_faces,
            projection_matrix,
        }
    }

    pub fn init(projection_matrix: Mat4) -> Self {
        let rotation = Vec3::init();
        let verticies = vec![];
        let translation = Vec3::init();
        let scale = Vec3::new(1., 1., 1.);
        let faces = vec![];
        let projected_faces = vec![([Vec2::init(); 3], Color::WHITE); faces.len()];

        Mesh {
            rotation,
            translation,
            scale,
            verticies,
            faces,
            projected_faces,
            projection_matrix,
        }
    }

    pub fn new_cube(projection_matrix: Mat4) -> Self {
        let mut mesh = Mesh::init(projection_matrix);
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
            Face::new([1, 2, 3], Color::WHITE),
            Face::new([1, 3, 4], Color::WHITE),
            Face::new([4, 3, 5], Color::RED),
            Face::new([4, 5, 6], Color::RED),
            Face::new([6, 5, 7], Color::GREEN),
            Face::new([6, 7, 8], Color::GREEN),
            Face::new([8, 7, 2], Color::BLUE),
            Face::new([8, 2, 1], Color::BLUE),
            Face::new([2, 7, 5], Color::MAGENTA),
            Face::new([2, 5, 3], Color::MAGENTA),
            Face::new([6, 8, 1], Color::YELLOW),
            Face::new([6, 1, 4], Color::YELLOW),
        ];

        mesh.verticies = verticies;
        mesh.faces = faces;
        mesh.projected_faces = vec![];
        mesh
    }

    pub fn update(&mut self, camera: Camera, cull: bool) {
        // Reset the last frames faces
        self.projected_faces = vec![];
        let mut projected_faces: Vec<(ProjectedFace, f32)> = vec![];

        self.rotation.x += 0.01;
        self.rotation.y += 0.01;
        self.rotation.z += 0.01;
        self.translation.z = 5.;

        let scale_matrix = Mat4::scale(self.scale.x, self.scale.y, self.scale.z);
        let translation_matrix =
            Mat4::translation(self.translation.x, self.translation.y, self.translation.z);
        let rot_x_matrix = Mat4::rotation_x(self.rotation.x);
        let rot_y_matrix = Mat4::rotation_y(self.rotation.y);
        let rot_z_matrix = Mat4::rotation_z(self.rotation.z);

        let transform_vertex = |v_index| {
            let vertex = self.verticies[v_index - 1];
            let vertex4 = Vec4::from(vertex);

            let mut world_matrix = Mat4::identity();
            world_matrix = scale_matrix * world_matrix;
            world_matrix = rot_z_matrix * world_matrix;
            world_matrix = rot_y_matrix * world_matrix;
            world_matrix = rot_x_matrix * world_matrix;
            world_matrix = translation_matrix * world_matrix;

            let transformed_vertex = world_matrix * vertex4;
            Vec3::from(transformed_vertex)
        };

        for face in &self.faces {
            let transformed_verticies: [Vec3; 3] = face
                .verticies_idx
                .iter()
                .map(transform_vertex)
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
                .map(|vertex| {
                    let v4 = Vec4::from(*vertex);
                    let projected = Mat4::project(self.projection_matrix, v4);
                    let mut v2 = Vec2::from(projected);

                    v2.x *= WIDTH as f32 / 2.;
                    v2.y *= HEIGHT as f32 / 2.;

                    v2.centered()
                })
                .collect::<Vec<Vec2>>()
                .try_into()
                .unwrap();

            // Store the depth for sorting
            let face_with_depth = ((projected_face, face.color), average_depth);

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

        let ab = (vb - va).unit_vector();
        let ac = (vc - va).unit_vector();

        let normal = ab.cross(ac).unit_vector();

        let camera_ray = camera.pos - va;
        let dot_normal_camera = normal.dot(camera_ray);

        dot_normal_camera < 0.
    }
}
