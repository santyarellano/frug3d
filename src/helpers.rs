use std::mem::swap;

/// Sort vertices by ascending y-coordinate (y0 < y1 < y2)
pub fn sort_vertices(
    x0: &mut i32,
    y0: &mut i32,
    x1: &mut i32,
    y1: &mut i32,
    x2: &mut i32,
    y2: &mut i32,
) {
    if y0 > y1 {
        swap(y0, y1);
        swap(x0, x1);
    }
    if y1 > y2 {
        swap(y1, y2);
        swap(x1, x2);
    }
    if y0 > y1 {
        swap(y0, y1);
        swap(x0, x1);
    }
}
