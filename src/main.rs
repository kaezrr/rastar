mod canvas;
mod colors;
mod utils;
mod vertex;

use canvas::Canvas;
use colors::Color;
use minifb::{Key, Window, WindowOptions};
use utils::{
    CANVAS_HEIGHT, CANVAS_WIDTH,
    colors::{BLUE, GREEN, RED},
    project_vertex,
};
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

    let v_af = Point3D::new(-2.0, -0.5, 5.0, None);
    let v_bf = Point3D::new(-2.0, 0.5, 5.0, None);
    let v_cf = Point3D::new(-1.0, 0.5, 5.0, None);
    let v_df = Point3D::new(-1.0, -0.5, 5.0, None);

    let v_ab = Point3D::new(-2.0, -0.5, 6.0, None);
    let v_bb = Point3D::new(-2.0, 0.5, 6.0, None);
    let v_cb = Point3D::new(-1.0, 0.5, 6.0, None);
    let v_db = Point3D::new(-1.0, -0.5, 6.0, None);

    canvas.draw_line(
        project_vertex(v_af),
        project_vertex(v_bf),
        &Color::new(BLUE),
    );
    canvas.draw_line(
        project_vertex(v_bf),
        project_vertex(v_cf),
        &Color::new(BLUE),
    );
    canvas.draw_line(
        project_vertex(v_cf),
        project_vertex(v_df),
        &Color::new(BLUE),
    );
    canvas.draw_line(
        project_vertex(v_df),
        project_vertex(v_af),
        &Color::new(BLUE),
    );

    canvas.draw_line(project_vertex(v_ab), project_vertex(v_bb), &Color::new(RED));
    canvas.draw_line(project_vertex(v_bb), project_vertex(v_cb), &Color::new(RED));
    canvas.draw_line(project_vertex(v_cb), project_vertex(v_db), &Color::new(RED));
    canvas.draw_line(project_vertex(v_db), project_vertex(v_ab), &Color::new(RED));

    canvas.draw_line(
        project_vertex(v_af),
        project_vertex(v_ab),
        &Color::new(GREEN),
    );
    canvas.draw_line(
        project_vertex(v_bf),
        project_vertex(v_bb),
        &Color::new(GREEN),
    );
    canvas.draw_line(
        project_vertex(v_cf),
        project_vertex(v_cb),
        &Color::new(GREEN),
    );
    canvas.draw_line(
        project_vertex(v_df),
        project_vertex(v_db),
        &Color::new(GREEN),
    );

    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&canvas.buffer, CANVAS_WIDTH, CANVAS_HEIGHT)
            .unwrap();
    }
}
