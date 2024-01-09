use crate::vector::Vec2;

pub struct Face {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

pub struct Triangle {
    pub points: [Vec2; 3],
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            points: [
                Vec2 {
                    ..Default::default()
                },
                Vec2 {
                    ..Default::default()
                },
                Vec2 {
                    ..Default::default()
                },
            ],
        }
    }
}
