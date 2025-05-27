mod canvas;
mod colors;
mod utils;
mod vertex;

use canvas::Canvas;
use colors::constants::GREEN;
use minifb::{Key, Window, WindowOptions};
use utils::{CANVAS_HEIGHT, CANVAS_WIDTH};
use vertex::Vertex;

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

    canvas.draw_filled_triangle(
        Vertex::new(-200, -250, Some(0.5)),
        Vertex::new(200, 50, Some(0.1)),
        Vertex::new(20, 250, Some(1.0)),
        &GREEN,
    );

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&canvas.buffer, CANVAS_WIDTH, CANVAS_HEIGHT)
            .unwrap();
    }
}
