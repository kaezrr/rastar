mod canvas;
mod structs;
mod utils;

use canvas::Canvas;
use minifb::{Key, Window, WindowOptions};
use structs::{Color, Instance, Model, Triangle};
use utils::{CANVAS_HEIGHT, CANVAS_WIDTH};

use glam::vec3;

fn main() {
    let mut canvas = Canvas::new();
    let mut window = Window::new(
        "Rastar - ESC to exit",
        CANVAS_WIDTH as usize,
        CANVAS_HEIGHT as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let cube_model = Model {
        vertices: vec![
            vec3(1.0, 1.0, 1.0),
            vec3(-1.0, 1.0, 1.0),
            vec3(-1.0, -1.0, 1.0),
            vec3(1.0, -1.0, 1.0),
            vec3(1.0, 1.0, -1.0),
            vec3(-1.0, 1.0, -1.0),
            vec3(-1.0, -1.0, -1.0),
            vec3(1.0, -1.0, -1.0),
        ],
        triangles: vec![
            Triangle::new(0, 1, 2, 0xff0000),
            Triangle::new(0, 2, 3, 0xff0000),
            Triangle::new(4, 0, 3, 0x00ff00),
            Triangle::new(4, 3, 7, 0x00ff00),
            Triangle::new(5, 4, 7, 0x0000ff),
            Triangle::new(5, 7, 6, 0x0000ff),
            Triangle::new(1, 5, 6, 0xffff00),
            Triangle::new(1, 6, 2, 0xffff00),
            Triangle::new(4, 5, 1, 0xff00ff),
            Triangle::new(4, 1, 0, 0xff00ff),
            Triangle::new(2, 6, 7, 0x00ffff),
            Triangle::new(2, 7, 3, 0x00ffff),
        ],
    };

    let cube1 = Instance::new(cube_model.clone(), 1.0, 0.0, vec3(-1.5, 0.0, 4.0));
    let cube2 = Instance::new(cube_model.clone(), 1.0, 0.0, vec3(1.5, 1.5, 4.0));
    canvas.render_scene(vec![cube1, cube2]);

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(
                &canvas.buffer,
                CANVAS_WIDTH as usize,
                CANVAS_HEIGHT as usize,
            )
            .unwrap();
    }
}
