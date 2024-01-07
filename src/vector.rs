pub struct vec2 {
    x: f32,
    y: f32,
}

pub struct vec3 {
    x: f32,
    y: f32,
    z: f32,
}

// ===============================================================================
// Vector 2D functions
// ===============================================================================
pub fn vec2_length(v: vec2) -> f32 {
    return (v.x * v.x + v.y * v.y).sqrt();
}

pub fn vec2_add(a: vec2, b: vec2) -> vec2 {
    vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

pub fn vec2_sub(a: vec2, b: vec2) -> vec2 {
    vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

pub fn vec2_mul(v: vec2, factor: f32) -> vec2 {
    vec2 {
        x: v.x * factor,
        y: v.y * factor,
    }
}

pub fn vec2_div(v: vec2, factor: f32) -> vec2 {
    vec2 {
        x: v.x / factor,
        y: v.y / factor,
    }
}

pub fn vec2_dot(a: vec2, b: vec2) -> f32 {
    return (a.x * b.x) + (a.y * b.y);
}

pub fn vec2_normalize(v: &mut vec2) {
    let length: f32 = (v.x * v.x + v.y * v.y).sqrt();
    v.x /= length;
    v.y /= length;
}

// ===============================================================================
// Vector 3D functions
// ===============================================================================
pub fn vec3_length(v: vec3) -> f32 {
    return (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
}

pub fn vec3_add(a: vec3, b: vec3) -> vec3 {
    vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

pub fn vec3_sub(a: vec3, b: vec3) -> vec3 {
    vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

pub fn vec3_mul(v: vec3, factor: f32) -> vec3 {
    vec3 {
        x: v.x * factor,
        y: v.y * factor,
        z: v.z * factor,
    }
}

pub fn vec3_div(v: vec3, factor: f32) -> vec3 {
    vec3 {
        x: v.x / factor,
        y: v.y / factor,
        z: v.z / factor,
    }
}

pub fn vec3_cross(a: vec3, b: vec3) -> vec3 {
    vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn vec3_dot(a: vec3, b: vec3) -> f32 {
    return (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
}

pub fn vec3_normalize(v: &mut vec3) {
    let length: f32 = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    v.x /= length;
    v.y /= length;
    v.z /= length;
}

pub fn vec3_rotate_x(v: vec3, angle: f32) -> vec3 {
    vec3 {
        x: v.x,
        y: v.y * angle.cos() - v.z * angle.sin(),
        z: v.y * angle.sin() + v.z * angle.cos(),
    }
}

pub fn vec3_rotate_y(v: vec3, angle: f32) -> vec3 {
    vec3 {
        x: v.x * angle.cos() - v.z * angle.sin(),
        y: v.y,
        z: v.x * angle.sin() + v.z * angle.cos(),
    }
}

pub fn vec3_rotate_z(v: vec3, angle: f32) -> vec3 {
    vec3 {
        x: v.x * angle.cos() - v.y * angle.sin(),
        y: v.x * angle.sin() + v.y * angle.cos(),
        z: v.z,
    }
}
