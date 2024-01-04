use crate::consts::{HEIGHT, WIDTH};

pub fn draw_pixel(frame: &mut [u8], xpos: usize, ypos: usize, rgba: [u8; 4]) {
    if xpos < WIDTH as usize && ypos < HEIGHT as usize {
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4)] = rgba[0];
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4) + 1] = rgba[1];
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4) + 2] = rgba[2];
        frame[(WIDTH as usize * ypos * 4) + (xpos * 4) + 3] = rgba[3];
    }
}

pub fn clear_color_buffer(frame: &mut [u8], rgba: [u8; 4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&rgba);
    }
}

pub fn draw_line(frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, rgba: [u8; 4]) {
    let delta_x = x1 - x0;
    let delta_y = y1 - y0;

    // obtain the longest side length
    let side_length = if delta_x.abs() >= delta_y.abs() {
        delta_x.abs()
    } else {
        delta_y.abs()
    };

    // Find how much we should increment in both x and y
    let x_inc = delta_x as f32 / side_length as f32;
    let y_inc = delta_y as f32 / side_length as f32;

    let mut current_x = x0 as f32;
    let mut current_y = y0 as f32;

    // loop each step and draw the pixel
    for i in 0..side_length {
        draw_pixel(
            frame,
            current_x.round() as usize,
            current_y.round() as usize,
            rgba,
        );
        current_x += x_inc;
        current_y += y_inc;
    }
}

pub fn draw_triangle(
    frame: &mut [u8],
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    rgba: [u8; 4],
) {
    draw_line(frame, x0, y0, x1, y1, rgba);
    draw_line(frame, x1, y1, x2, y2, rgba);
    draw_line(frame, x2, y2, x0, y0, rgba);
}
