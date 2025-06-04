use glam::{Vec2, Vec3};

pub const CANVAS_WIDTH: f32 = 640.;
pub const CANVAS_HEIGHT: f32 = 640.;
pub const CANVAS_BOUND_X: f32 = CANVAS_WIDTH / 2.;
pub const CANVAS_BOUND_Y: f32 = CANVAS_HEIGHT / 2.;

pub const VIEWPORT_HEIGHT: f32 = 1.;
pub const VIEWPORT_WIDTH: f32 = 1.;
pub const VIEWPORT_DISTANCE: f32 = 1.;

pub fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::with_capacity((i1 - i0 + 1) as usize);
    let a = (d1 - d0) / (i1 - i0) as f32;
    for i in i0..=i1 {
        let value = d0 + a * (i - i0) as f32;
        values.push(value);
    }
    values
}

pub fn viewport_to_canvas(x: f32, y: f32) -> Vec2 {
    Vec2::new(
        x * (CANVAS_WIDTH / VIEWPORT_WIDTH),
        y * (CANVAS_HEIGHT / VIEWPORT_HEIGHT),
    )
}

pub fn project_vertex(v: Vec3) -> Vec2 {
    viewport_to_canvas(
        v.x * (VIEWPORT_DISTANCE / v.z),
        v.y * (VIEWPORT_DISTANCE / v.z),
    )
}
