use crate::canvas::Color;

pub const CANVAS_WIDTH: usize = 1280;
pub const CANVAS_HEIGHT: usize = 720;
pub const CANVAS_BOUND_X: i32 = (CANVAS_WIDTH / 2) as i32;
pub const CANVAS_BOUND_Y: i32 = (CANVAS_HEIGHT / 2) as i32;

// Colors

#[allow(dead_code)]
pub mod colors {
    use super::Color;
    pub const RED: Color = Color::from(0xff0000);
    pub const GREEN: Color = Color::from(0x00ff00);
    pub const BLUE: Color = Color::from(0x0000ff);
    pub const WHITE: Color = Color::from(0xffffff);
}
