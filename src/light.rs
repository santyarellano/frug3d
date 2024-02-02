use crate::vector::Vec3;

pub struct Light {
    pub direction: Vec3,
}

/// Mutates a color based on an intensity (which should be a percentage 0.0 - 1.0)
pub fn light_apply_intensity(color: &[u8; 4], mut intensity: f32) -> [u8; 4] {
    if intensity < 0.0 {
        intensity = 0.0;
    } else if intensity > 1.0 {
        intensity = 1.0;
    }

    [
        (color[0] as f32 * intensity) as u8,
        (color[1] as f32 * intensity) as u8,
        (color[2] as f32 * intensity) as u8,
        color[3],
    ]
}
