use crate::vector::Vec2;

pub struct Face {
    a: i32,
    b: i32,
    c: i32,
}

pub struct Triangle {
    points: [Vec2; 2],
}
