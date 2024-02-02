use std::fs;
use std::io::{BufRead, BufReader, Error};

use sscanf::scanf;

use crate::consts::{self, C_BLUE, C_GREEN};
use crate::C_WHITE;
use crate::{triangle::Face, vector::Vec3};

// ===================================================================
// Variables & definitions
// ===================================================================
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub translation: Vec3,
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            rotation: Vec3 {
                ..Default::default()
            },
            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            translation: Vec3 {
                ..Default::default()
            },
        }
    }
}

// ===================================================================
// Functions
// ===================================================================

fn scan_str_to(original: String) {}

/// Read contents of the .obj file and load them into mesh data
pub fn load_obj_file_data(filename: String) -> Result<Mesh, Error> {
    let mut mesh = Mesh {
        ..Default::default()
    };

    let buff_reader = BufReader::new(fs::File::open(filename)?);

    for buf_line in buff_reader.lines() {
        let line = buf_line.expect("Error reading file line.");

        // get vertex data
        if line.starts_with("v ") {
            let mut vertex = Vec3 {
                ..Default::default()
            };
            (vertex.x, vertex.y, vertex.z) = sscanf::scanf!(line, "v {} {} {}", f32, f32, f32)
                .expect("Error reading vertex data.");

            mesh.vertices.push(vertex);
        }

        // get face data
        if line.starts_with("f ") {
            // note: format is <vertex index>/<texture coords>/<normal indices>
            let mut vertex_indices: [i32; 3] = [0; 3];
            let mut texture_indices: [i32; 3] = [0; 3];
            let mut normal_indices: [i32; 3] = [0; 3];

            (
                vertex_indices[0],
                texture_indices[0],
                normal_indices[0],
                vertex_indices[1],
                texture_indices[1],
                normal_indices[1],
                vertex_indices[2],
                texture_indices[2],
                normal_indices[2],
            ) = sscanf::scanf!(
                line,
                "f {}/{}/{} {}/{}/{} {}/{}/{}",
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )
            .expect("Error reading face data.");

            let face = Face {
                a: vertex_indices[0],
                b: vertex_indices[1],
                c: vertex_indices[2],
                rgba: C_WHITE, // Hardcoded
            };

            mesh.faces.push(face);
        }
        //println!("faces len {}", mesh.faces.len());
    }

    return Ok(mesh);
}
