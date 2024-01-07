pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    fn default() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

// ===============================================================================
// Vector 2D functions
// ===============================================================================
pub fn vec2_length(v: Vec2) -> f32 {
    return (v.x * v.x + v.y * v.y).sqrt();
}

pub fn vec2_add(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x + b.x,
        y: a.y + b.y,
    }
}

pub fn vec2_sub(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}

pub fn vec2_mul(v: Vec2, factor: f32) -> Vec2 {
    Vec2 {
        x: v.x * factor,
        y: v.y * factor,
    }
}

pub fn vec2_div(v: Vec2, factor: f32) -> Vec2 {
    Vec2 {
        x: v.x / factor,
        y: v.y / factor,
    }
}

pub fn vec2_dot(a: Vec2, b: Vec2) -> f32 {
    return (a.x * b.x) + (a.y * b.y);
}

pub fn vec2_normalize(v: &mut Vec2) {
    let length: f32 = (v.x * v.x + v.y * v.y).sqrt();
    v.x /= length;
    v.y /= length;
}

// ===============================================================================
// Vector 3D functions
// ===============================================================================
pub fn vec3_length(v: Vec3) -> f32 {
    return (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
}

pub fn vec3_add(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
    }
}

pub fn vec3_sub(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
}

pub fn vec3_mul(v: Vec3, factor: f32) -> Vec3 {
    Vec3 {
        x: v.x * factor,
        y: v.y * factor,
        z: v.z * factor,
    }
}

pub fn vec3_div(v: Vec3, factor: f32) -> Vec3 {
    Vec3 {
        x: v.x / factor,
        y: v.y / factor,
        z: v.z / factor,
    }
}

pub fn vec3_cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

pub fn vec3_dot(a: Vec3, b: Vec3) -> f32 {
    return (a.x * b.x) + (a.y * b.y) + (a.z * b.z);
}

pub fn vec3_normalize(v: &mut Vec3) {
    let length: f32 = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    v.x /= length;
    v.y /= length;
    v.z /= length;
}

pub fn vec3_rotate_x(v: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: v.x,
        y: v.y * angle.cos() - v.z * angle.sin(),
        z: v.y * angle.sin() + v.z * angle.cos(),
    }
}

pub fn vec3_rotate_y(v: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: v.x * angle.cos() - v.z * angle.sin(),
        y: v.y,
        z: v.x * angle.sin() + v.z * angle.cos(),
    }
}

pub fn vec3_rotate_z(v: Vec3, angle: f32) -> Vec3 {
    Vec3 {
        x: v.x * angle.cos() - v.y * angle.sin(),
        y: v.x * angle.sin() + v.y * angle.cos(),
        z: v.z,
    }
}
