use crate::structs::{Color, Instance, Triangle};
use glam::{Vec2, vec2};

use crate::utils::{
    CANVAS_BOUND_X, CANVAS_BOUND_Y, CANVAS_HEIGHT, CANVAS_WIDTH, interpolate, project_vertex,
};
use std::panic;

pub struct Canvas {
    pub buffer: Vec<u32>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        Canvas {
            buffer: vec![0x1B1E1F; (CANVAS_HEIGHT * CANVAS_WIDTH) as usize],
        }
    }

    pub fn put_pixel(&mut self, p: Vec2, color: &Color) {
        let p = vec2(p.x.round(), p.y.round());

        if p.x < -CANVAS_BOUND_X || p.x >= CANVAS_BOUND_X {
            panic!("{p:?} lies outside screen width!");
        }

        if p.y < -CANVAS_BOUND_Y || p.y >= CANVAS_BOUND_Y {
            panic!("{p:?} lies outside screen height!");
        }

        let screen_index = ((CANVAS_BOUND_Y - p.y) * CANVAS_WIDTH) + CANVAS_BOUND_X + p.x;
        self.buffer[screen_index as usize] = color.as_u32();
    }

    pub fn draw_line(&mut self, mut p0: Vec2, mut p1: Vec2, color: &Color) {
        if (p0.x - p1.x).abs() > (p0.y - p1.y).abs() {
            // Mostly horizontal line
            if p0.x > p1.x {
                std::mem::swap(&mut p0, &mut p1);
            }
            let x0 = p0.x as i32;
            let x1 = p1.x as i32;
            let ys = interpolate(x0, p0.y, x1, p1.y);

            for x in x0..=x1 {
                let y = ys[(x - x0) as usize];
                self.put_pixel(vec2(x as f32, y), color);
            }
        } else {
            // Mostly vertical line
            if p0.y > p1.y {
                std::mem::swap(&mut p0, &mut p1);
            }
            let y0 = p0.y as i32;
            let y1 = p1.y as i32;
            let xs = interpolate(y0, p0.x, y1, p1.x);
            for y in y0..=y1 {
                let x = xs[(y - y0) as usize];
                self.put_pixel(vec2(x, y as f32), color);
            }
        }
    }

    // pub fn draw_filled_triangle(
    //     &mut self,
    //     mut p0: Vec3A,
    //     mut p1: Vec3A,
    //     mut p2: Vec3A,
    //     color: &Color,
    // ) {
    //     if p0.y > p1.y {
    //         std::mem::swap(&mut p0, &mut p1);
    //     }
    //     if p0.y > p2.y {
    //         std::mem::swap(&mut p0, &mut p2);
    //     }
    //     if p1.y > p2.y {
    //         std::mem::swap(&mut p1, &mut p2);
    //     }
    //
    //     let x01 = interpolate(p0.y, p0.x, p1.y, p1.x);
    //     let h01 = interpolate(p0.y, p0.z, p1.y, p1.z);
    //
    //     let x12 = interpolate(p1.y, p1.x, p2.y, p2.x);
    //     let h12 = interpolate(p1.y, p1.z, p2.y, p2.z);
    //
    //     let x02 = interpolate(p0.y, p0.x, p2.y, p2.x);
    //     let h02 = interpolate(p0.y, p0.z, p2.y, p2.z);
    //
    //     let mut x012 = x01;
    //     x012.pop();
    //     x012.extend(x12);
    //
    //     let mut h012 = h01;
    //     h012.pop();
    //     h012.extend(h12);
    //
    //     let m = x02.len() / 2;
    //     let (x_left, h_left, x_right, h_right) = if x02[m] < x012[m] {
    //         (x02, h02, x012, h012)
    //     } else {
    //         (x012, h012, x02, h02)
    //     };
    //
    //     for y in p0.y..=p2.y {
    //         let x_l = x_left[(y - p0.y) as usize].round() as i32;
    //         let x_r = x_right[(y - p0.y) as usize].round() as i32;
    //
    //         let h_segment = interpolate(
    //             x_l,
    //             h_left[(y - p0.y) as usize],
    //             x_r,
    //             h_right[(y - p0.y) as usize],
    //         );
    //
    //         for x in x_l..=x_r {
    //             let shaded_color = color.scaled(h_segment[(x - x_l) as usize]);
    //             self.put_pixel(Point2D::new(x, y, None), &shaded_color);
    //         }
    //     }
    // }

    pub fn draw_wireframe_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, color: &Color) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    pub fn render_instance(&mut self, instance: &Instance) {
        let mut projected = vec![];
        for v in instance.model.vertices.iter() {
            let v_t = instance.transform.transform_point3(*v);
            projected.push(project_vertex(v_t));
        }

        for t in instance.model.triangles.iter() {
            self.render_triangle(t, &projected);
        }
    }

    pub fn render_triangle(&mut self, triangle: &Triangle, projected: &[Vec2]) {
        self.draw_wireframe_triangle(
            projected[triangle.v1],
            projected[triangle.v2],
            projected[triangle.v3],
            &triangle.color,
        );
    }

    pub fn render_scene(&mut self, scene: &Vec<Instance>) {
        for instance in scene.iter() {
            self.render_instance(instance);
        }
    }
}
