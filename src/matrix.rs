use crate::vector::Vec4;

pub struct Mat4 {
    pub m: [[f32; 4]; 4],
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4 {
            m: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }
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

/// | 1 0  0 0 |
/// | 0 c -s 0 |
/// | 0 s  c 0 |
/// | 0 0  0 1 |
pub fn mat4_make_rotation_x(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    let mut m = mat4_identity();
    m.m[1][1] = c;
    m.m[1][2] = -s;
    m.m[2][1] = s;
    m.m[2][2] = c;

    return m;
}

/// |  c 0 s 0 |
/// |  0 1 0 0 |
/// | -s 0 c 0 |
/// |  0 0 0 1 |
pub fn mat4_make_rotation_y(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    let mut m = mat4_identity();
    m.m[0][0] = c;
    m.m[0][2] = s;
    m.m[2][0] = -s;
    m.m[2][2] = c;

    return m;
}

/// | c -s 0 0 |
/// | s  c 0 0 |
/// | 0  0 1 0 |
/// | 0  0 0 1 |
pub fn mat4_make_rotation_z(angle: f32) -> Mat4 {
    let c = angle.cos();
    let s = angle.sin();
    let mut m = mat4_identity();
    m.m[0][0] = c;
    m.m[0][1] = -s;
    m.m[1][0] = s;
    m.m[1][1] = c;

    return m;
}

/// | (h/w)*1/tan(fov/2)        0               0               0           |
/// |         0            1/tan(fov/2)         0               0           |
/// |         0                 0           zf/(zf-zn)  (-zf*zn)/(zf-zn)    |
/// |         0                 0               1               0           |
pub fn mat4_make_perspective(fov: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
    let mut m: Mat4 = Mat4 {
        ..Default::default()
    };

    m.m[0][0] = aspect * (1.0 / (fov / 2.0).tan());
    m.m[1][1] = 1.0 / (fov / 2.0).tan();
    m.m[2][2] = zfar / (zfar - znear);
    m.m[2][3] = (-zfar * znear) / (zfar - znear);
    m.m[3][2] = 1.0;

    return m;
}

pub fn mat4_mul_vec4_project(mat_proj: &Mat4, v: &Vec4) -> Vec4 {
    // multiply the projection matrix by our original vector
    let mut result = mat4_mul_vec4(mat_proj, v);

    // perform perspective divide with original z-value that is now stored in w
    if result.w != 0.0 {
        result.x /= result.w;
        result.y /= result.w;
        result.z /= result.w;
    }

    return result;
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

/// Multiplies a matrix (4d) with a matrix (4d), returning a mat4
pub fn mat4_mul_mat4(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut mat: Mat4 = Mat4 {
        ..Default::default()
    };

    for i in 0..4 {
        for j in 0..4 {
            mat.m[i][j] = a.m[i][0] * b.m[0][j]
                + a.m[i][1] * b.m[1][j]
                + a.m[i][2] * b.m[2][j]
                + a.m[i][3] * b.m[3][j];
        }
    }

    return mat;
}
