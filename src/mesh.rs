use crate::{triangle::face, vector::vec3};

// ===================================================================
// Variables & definitions
// ===================================================================
pub struct Mesh {
    vertices: Vec<vec3>,
    faces: Vec<face>,
    rotation: vec3,
}

// ===================================================================
// Functions
// ===================================================================

/// Read contents of the .obj file and load them into mesh data
pub fn load_obj_file_data(filename: str) -> Mesh {}
