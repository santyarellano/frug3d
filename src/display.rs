use crate::{
    consts::{HEIGHT, WIDTH},
    helpers::sort_vertices,
};

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

/// Draws a line from coordinate 0 to coordinate 1 given a color and a frame.
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

fn fill_flat_bottom_triangle(
    frame: &mut [u8],
    rgba: [u8; 4],
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    xm: i32,
    ym: i32,
) {
    // Find the 2 slopes (two triangle legs)
    //  slope1 = c0 -> c1
    //  slope2 = c0 -> cm
    let inverse_slope1: f32 = (x1 - x0) as f32 / (y1 - y0) as f32;
    let inverse_slope2: f32 = (xm - x0) as f32 / (ym - y0) as f32;

    // Start x_start and x_end from the top vertex
    let mut x_start = x0 as f32;
    let mut x_end = x0 as f32;

    // loop all the scanlines from top to bottom
    for y in y0..(ym + 1) {
        draw_line(frame, x_start as i32, y, x_end as i32, y, rgba);
        x_start += inverse_slope1;
        x_end += inverse_slope2;
    }
}

fn fill_flat_top_triangle(
    frame: &mut [u8],
    rgba: [u8; 4],
    x1: i32,
    y1: i32,
    xm: i32,
    ym: i32,
    x2: i32,
    y2: i32,
) {
    // Find the 2 slopes (two triangle legs)
    //  slope1 = c2 -> c1
    //  slope2 = c2 -> cm
    let inverse_slope1: f32 = (x2 - x1) as f32 / (y2 - y1) as f32;
    let inverse_slope2: f32 = (x2 - xm) as f32 / (y2 - ym) as f32;

    // Start x_start and x_end from the bottom vertex
    let mut x_start = x2 as f32;
    let mut x_end = x2 as f32;

    // loop all the scanlines from bottom to top
    for y in (y1..(y2 + 1)).rev() {
        draw_line(frame, x_start as i32, y, x_end as i32, y, rgba);
        x_start -= inverse_slope1;
        x_end -= inverse_slope2;
    }
}

/// Draw a filled triangle with the flat-top/flat-bottom method.
pub fn draw_filled_triangle(
    frame: &mut [u8],
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    rgba: [u8; 4],
) {
    sort_vertices(&mut x0, &mut y0, &mut x1, &mut y1, &mut x2, &mut y2);

    if y1 == y2 {
        // Just draw upper half to avoid division by 0.
        fill_flat_bottom_triangle(frame, rgba, x0, y0, x1, y1, x2, y2);
    } else if y0 == y1 {
        // Just draw the bottom half to avoid division by 0.
        fill_flat_top_triangle(frame, rgba, x0, y0, x1, y1, x2, y2);
    } else {
        // Calculate the new vertex (middle point) with triangle similarity
        let my = y1;
        let mx = ((x2 - x0) * (y1 - y0)) / (y2 - y0) + x0;

        // Draw flat-bottom triangle
        fill_flat_bottom_triangle(frame, rgba, x0, y0, x1, y1, mx, my);

        // Draw flat-top triangle
        fill_flat_top_triangle(frame, rgba, x1, y1, mx, my, x2, y2);
    }
}

pub fn draw_triangle(
    frame: &mut [u8],
    rgba: [u8; 4],
    fill: bool,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) {
    if fill {
        draw_filled_triangle(frame, x0, y0, x1, y1, x2, y2, rgba);
    } else {
        draw_line(frame, x0, y0, x1, y1, rgba);
        draw_line(frame, x1, y1, x2, y2, rgba);
        draw_line(frame, x2, y2, x0, y0, rgba);
    }
}

pub fn draw_rect(
    frame: &mut [u8],
    x_pos: usize,
    y_pos: usize,
    width: usize,
    height: usize,
    rgba: [u8; 4],
) {
    let mut x = x_pos;
    while x < x_pos + width && x < WIDTH as usize {
        let mut y = y_pos;
        while y < y_pos + height && y < HEIGHT as usize {
            draw_pixel(frame, x, y, rgba);
            y += 1;
        }
        x += 1;
    }
}

pub fn draw_grid(frame: &mut [u8], rgba: [u8; 4], gap: usize) {
    let mut x = gap / 2;
    while x < WIDTH as usize {
        let mut y = gap / 2;
        while y < HEIGHT as usize {
            draw_pixel(frame, x, y, rgba);
            y += gap;
        }
        x += gap;
    }
}
