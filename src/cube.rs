use crate::{
    camera::Camera,
    face::Face,
    vec3::{Vec2, Vec3},
};

pub struct Cube {
    rotation: Vec3,
    verticies: [Vec3; 8],
    pub projected_points: [Vec2; 8],
    faces: Vec<Face>,
    pub triangle_points: Vec<Vec2>,
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

        let rotation = Vec3::init();
        let projected_points = [Vec2::init(); 8];

        let triangle_points = vec![];

        Cube {
            rotation,
            verticies,
            faces,
            projected_points,
            triangle_points,
        }
    }

    pub fn update(&mut self, camera: Camera) {
        self.rotation.x = 10.;
        self.rotation.y += 0.01;
        self.rotation.z += 0.01;

        // self.project_verticies(camera);
        // self.update_verticies(camera);
        self.project_faces(camera);
    }

    // fn update_verticies(&mut self, camera: Camera) {
    //     for i in 0..self.verticies.len() {
    //         let mut vertex = self.verticies[i];
    //         vertex = vertex.rot_x(self.rotation.x);
    //         vertex = vertex.rot_y(self.rotation.y);
    //         vertex = vertex.rot_z(self.rotation.z);
    //         vertex.z -= camera.pos.z;

    //         self.verticies[i] = vertex;
    //     }
    // }

    // fn project_verticies(&mut self, camera: Camera) {
    //     for i in 0..self.verticies.len() {
    //         let mut vertex = self.verticies[i];
    //         vertex = vertex.rot_x(self.rotation.x);
    //         vertex = vertex.rot_y(self.rotation.y);
    //         vertex = vertex.rot_z(self.rotation.z);
    //         vertex.z -= camera.pos.z;

    //         let projected_point = vertex.project(camera.fov);
    //         self.projected_points[i] = projected_point
    //     }
    // }

    fn project_faces(&mut self, camera: Camera) {
        self.triangle_points = vec![];
        for i in 0..self.faces.len() {
            let face = self.faces[i];

            for v_index in face {
                let vertex = self.verticies[v_index - 1];
                let rotated = vertex.rotate(self.rotation);
                let transformed = rotated - camera.pos;

                let projected = transformed.project(camera.fov);
                self.triangle_points.push(projected);
            }
        }
    }
}
