use crate::vertex::{Point2D, Point3D};

pub const CANVAS_WIDTH: usize = 640;
pub const CANVAS_HEIGHT: usize = 640;
pub const CANVAS_BOUND_X: i32 = (CANVAS_WIDTH / 2) as i32;
pub const CANVAS_BOUND_Y: i32 = (CANVAS_HEIGHT / 2) as i32;

pub const VIEWPORT_HEIGHT: usize = 1;
pub const VIEWPORT_WIDTH: usize = 1;
pub const VIEWPORT_DISTANCE: usize = 1;

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

pub fn viewport_to_canvas(x: f64, y: f64) -> Point2D {
    Point2D::new(
        (x * (CANVAS_WIDTH / VIEWPORT_WIDTH) as f64).round() as i32,
        (y * (CANVAS_HEIGHT / VIEWPORT_HEIGHT) as f64).round() as i32,
        None,
    )
}

pub fn project_vertex(v: Point3D) -> Point2D {
    viewport_to_canvas(
        v.x * (VIEWPORT_DISTANCE as f64 / v.z),
        v.y * (VIEWPORT_DISTANCE as f64 / v.z),
    )
}
