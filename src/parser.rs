use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{face::Face, math::vec3};
use sdl2::pixels::Color;
use vec3::Vec3;

pub fn parse_obj(path: &str) -> (Vec<Vec3>, Vec<Face>) {
    let file = File::open(path).expect(&format!("Unable to open file at {}", path));
    let reader = BufReader::new(file);

    let mut verticies: Vec<Vec3> = vec![];
    let mut faces: Vec<Face> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut split_line = line.split_whitespace();

        // If line is not empty
        if let Some(first_word) = split_line.next() {
            // Parse vertex
            // v -1.000000 -1.000000 1.000000
            if first_word == "v" {
                let vertex = Vec3::new(
                    split_line.next().unwrap().parse::<f32>().unwrap(),
                    split_line.next().unwrap().parse::<f32>().unwrap(),
                    split_line.next().unwrap().parse::<f32>().unwrap(),
                );

                verticies.push(vertex);
            }

            // Parse face
            // f 3/1/2 4/2/2 5/3/2 -> 3, 4, 5
            if first_word == "f" {
                let mut face_triples = line.split_whitespace();
                face_triples.next();

                let t1 = face_triples.next().unwrap();
                let f1 = t1.split("/").next().unwrap();

                let t2 = face_triples.next().unwrap();
                let f2 = t2.split("/").next().unwrap();

                let t3 = face_triples.next().unwrap();
                let f3 = t3.split("/").next().unwrap();

                let face = (
                    [
                        f1.parse::<usize>().unwrap(),
                        f2.parse::<usize>().unwrap(),
                        f3.parse::<usize>().unwrap(),
                    ],
                    Color::GREEN,
                );
                faces.push(face)
            }
        }
    }

    (verticies, faces)
}
