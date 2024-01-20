use std::mem::swap;

/// Sort vertices by ascending y-coordinate (y0 < y1 < y2)
pub fn sort_vertices(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32) {
    if y0 > y1 {
        swap(&mut y0, &mut y1);
        swap(&mut x0, &mut x1);
    }
    if y1 > y2 {
        swap(&mut y1, &mut y2);
        swap(&mut x1, &mut x2);
    }
    if y0 > y1 {
        swap(&mut y0, &mut y1);
        swap(&mut x0, &mut x1);
    }
}
