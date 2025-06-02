mod canvas;
mod colors;
mod instance;
mod model;
mod utils;
mod vertex;

use canvas::Canvas;
use instance::{Instance, Transform};
use minifb::{Key, Window, WindowOptions};
use model::{Model, Triangle};
use utils::{CANVAS_HEIGHT, CANVAS_WIDTH};
use vertex::Point3D;

fn main() {
    let mut canvas = Canvas::new();
    let mut window = Window::new(
        "Rastar - ESC to exit",
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let cube_model = Model {
        name: String::from("Cube"),
        vertices: vec![
            Point3D::new(1.0, 1.0, 1.0, None),
            Point3D::new(-1.0, 1.0, 1.0, None),
            Point3D::new(-1.0, -1.0, 1.0, None),
            Point3D::new(1.0, -1.0, 1.0, None),
            Point3D::new(1.0, 1.0, -1.0, None),
            Point3D::new(-1.0, 1.0, -1.0, None),
            Point3D::new(-1.0, -1.0, -1.0, None),
            Point3D::new(1.0, -1.0, -1.0, None),
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

    let cube1 = Instance {
        model: &cube_model,
        transform: Transform {
            scale: 1.0,
            rotation: 0.0,
            translation: Point3D::new(-1.5, 0.0, 7.0, None),
        },
    };

    let cube2 = Instance {
        model: &cube_model,
        transform: Transform {
            scale: 0.8,
            rotation: 45.0,
            translation: Point3D::new(1.5, 1.5, 7.0, None),
        },
    };

    canvas.render_scene(&vec![cube1, cube2]);

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&canvas.buffer, CANVAS_WIDTH, CANVAS_HEIGHT)
            .unwrap();
    }
}
