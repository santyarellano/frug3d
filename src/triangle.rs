use crate::{consts::C_MAGENTA, vector::Vec2};

pub struct Face {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub rgba: [u8; 4],
}

pub struct Triangle {
    pub points: [Vec2; 3],
    pub rgba: [u8; 4],
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
            rgba: C_MAGENTA,
        }
    }
}
