use std::fs;
use std::io::{BufRead, BufReader, Error};

use crate::{triangle::Face, vector::Vec3};

// ===================================================================
// Variables & definitions
// ===================================================================
pub struct Mesh {
    vertices: Vec<Vec3>,
    faces: Vec<Face>,
    rotation: Vec3,
}

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            vertices: Vec::new(),
            faces: Vec::new(),
            rotation: Vec3 {
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
            let parsed = sscanf::scanf!(line, "v {} {} {}", f32, f32, f32);
            println!("{:?}\n", parsed);

            // todo...
        }
    }

    return Ok(mesh);
}
