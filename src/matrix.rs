use crate::vector::Vec4;

pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

/// | 1 0 0 0 |
/// | 0 1 0 0 |
/// | 0 0 1 0 |
/// | 0 0 0 1 |
pub fn mat4_identity() -> Mat4 {
    Mat4 {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

/// | sx 0 0 0 |
/// | 0 sy 0 0 |
/// | 0 0 sz 0 |
/// | 0 0  0 1 |
pub fn mat4_make_scale(sx: f32, sy: f32, sz: f32) -> Mat4 {
    Mat4 {
        m: [
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, sz, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

/// | 1 0 0 tx |
/// | 0 1 0 ty |
/// | 0 0 1 tz |
/// | 0 0 0 1  |
pub fn mat4_make_translation(tx: f32, ty: f32, tz: f32) -> Mat4 {
    Mat4 {
        m: [
            [1.0, 0.0, 0.0, tx],
            [0.0, 1.0, 0.0, ty],
            [0.0, 0.0, 1.0, tz],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

/// Multiplies a matrix (4d) with a Vec4, returning a Vec4
pub fn mat4_mul_vec4(m: &Mat4, v: &Vec4) -> Vec4 {
    Vec4 {
        x: m.m[0][0] * v.x + m.m[0][1] * v.y + m.m[0][2] * v.z + m.m[0][3] * v.w,
        y: m.m[1][0] * v.x + m.m[1][1] * v.y + m.m[1][2] * v.z + m.m[1][3] * v.w,
        z: m.m[2][0] * v.x + m.m[2][1] * v.y + m.m[2][2] * v.z + m.m[2][3] * v.w,
        w: m.m[3][0] * v.x + m.m[3][1] * v.y + m.m[3][2] * v.z + m.m[3][3] * v.w,
    }
}
