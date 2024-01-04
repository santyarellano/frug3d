use crate::consts::{HEIGHT, WIDTH};

pub fn draw_pixel(frame: &mut [u8], xpos: usize, ypos: usize, rgba: [u8; 4]) {
    if xpos < WIDTH as usize && ypos < HEIGHT as usize {
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4)] = rgba[0];
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4) + 1] = rgba[1];
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4) + 2] = rgba[2];
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4) + 3] = rgba[3];
    }
}