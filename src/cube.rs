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
            Face::new(1, 2, 3),
            Face::new(1, 3, 4),
            Face::new(4, 3, 5),
            Face::new(4, 5, 6),
            Face::new(6, 5, 7),
            Face::new(6, 7, 8),
            Face::new(8, 7, 2),
            Face::new(8, 2, 1),
            Face::new(2, 7, 5),
            Face::new(2, 5, 3),
            Face::new(6, 8, 1),
            Face::new(6, 1, 4),
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

            let mut v1 = self.verticies[face.a - 1];
            let mut v2 = self.verticies[face.b - 1];
            let mut v3 = self.verticies[face.c - 1];

            v1 = v1.rotate(self.rotation) - camera.pos;
            v2 = v2.rotate(self.rotation) - camera.pos;
            v3 = v3.rotate(self.rotation) - camera.pos;

            let p1 = v1.project(camera.fov);
            let p2 = v2.project(camera.fov);
            let p3 = v3.project(camera.fov);

            self.triangle_points.push(p1);
            self.triangle_points.push(p2);
            self.triangle_points.push(p3);
        }
    }
}
