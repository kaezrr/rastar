pub const CANVAS_WIDTH: usize = 1280;
pub const CANVAS_HEIGHT: usize = 720;
pub const CANVAS_BOUND_X: i32 = (CANVAS_WIDTH / 2) as i32;
pub const CANVAS_BOUND_Y: i32 = (CANVAS_HEIGHT / 2) as i32;

pub fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> Vec<f64> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut values = Vec::new();
    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut b = d0;
    for _ in i0..=i1 {
        values.push(b);
        b += a;
    }
    values
}
