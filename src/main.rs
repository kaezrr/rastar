mod canvas;
mod utils;

use canvas::{Canvas, Point2D};
use minifb::{Key, Window, WindowOptions};
use utils::{CANVAS_HEIGHT, CANVAS_WIDTH, WHITE};

fn main() {
    let mut canvas = Canvas::new();
    let mut window = Window::new(
        "Test - ESC to exit",
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    canvas.draw_line(Point2D(-50, -200), Point2D(60, 120), &WHITE);

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&canvas.buffer, CANVAS_WIDTH, CANVAS_HEIGHT)
            .unwrap();
    }
}
