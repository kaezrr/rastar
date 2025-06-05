use core::f32;
use std::{f32::consts::FRAC_1_SQRT_2, panic};

use crate::{
    structs::{Color, Instance, Plane, Triangle},
    utils::{VIEWPORT_DISTANCE, clip_scene},
};
use glam::{Affine3A, Mat3A, Vec2, Vec3, vec2, vec3, vec3a};

use crate::utils::{
    CANVAS_BOUND_X, CANVAS_BOUND_Y, CANVAS_HEIGHT, CANVAS_WIDTH, interpolate, project_vertex,
};

pub struct Canvas {
    pub buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        Canvas {
            buffer: vec![0x1B1E1F; (CANVAS_HEIGHT * CANVAS_WIDTH) as usize],
            depth_buffer: vec![0.; (CANVAS_HEIGHT * CANVAS_WIDTH) as usize],
        }
    }

    pub fn put_pixel(&mut self, p: Vec2, color: &Color) {
        let p = vec2(p.x.round(), p.y.round());

        if p.x < -CANVAS_BOUND_X || p.x >= CANVAS_BOUND_X {
            panic!("{p} lies outside screen bounds");
        }

        if p.y < -CANVAS_BOUND_Y || p.y >= CANVAS_BOUND_Y {
            panic!("{p} lies outside screen bounds");
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

    pub fn draw_filled_triangle(
        &mut self,
        mut p0: Vec3,
        mut p1: Vec3,
        mut p2: Vec3,
        h: (f32, f32, f32),
        color: &Color,
    ) {
        if p0.y > p1.y {
            std::mem::swap(&mut p0, &mut p1);
        }
        if p0.y > p2.y {
            std::mem::swap(&mut p0, &mut p2);
        }
        if p1.y > p2.y {
            std::mem::swap(&mut p1, &mut p2);
        }

        let x01 = interpolate(p0.y as i32, p0.x, p1.y as i32, p1.x);
        let h01 = interpolate(p0.y as i32, h.0, p1.y as i32, h.1);
        let z01 = interpolate(p0.y as i32, p0.z, p1.y as i32, p1.z);

        let x12 = interpolate(p1.y as i32, p1.x, p2.y as i32, p2.x);
        let h12 = interpolate(p1.y as i32, h.1, p2.y as i32, h.2);
        let z12 = interpolate(p1.y as i32, p1.z, p2.y as i32, p2.z);

        let x02 = interpolate(p0.y as i32, p0.x, p2.y as i32, p2.x);
        let h02 = interpolate(p0.y as i32, h.0, p2.y as i32, h.2);
        let z02 = interpolate(p0.y as i32, p0.z, p2.y as i32, p2.z);

        let mut x012 = x01;
        x012.pop();
        x012.extend(x12);

        let mut h012 = h01;
        h012.pop();
        h012.extend(h12);

        let mut z012 = z01;
        z012.pop();
        z012.extend(z12);

        let m = x02.len() / 2;
        let (x_left, h_left, z_left, x_right, h_right, z_right) = if x02[m] < x012[m] {
            (x02, h02, z02, x012, h012, z012)
        } else {
            (x012, h012, z012, x02, h02, z02)
        };

        let y0 = p0.y as i32;
        let y2 = p2.y as i32;

        for y in y0..=y2 {
            let dy = (y - y0) as usize;

            let x_l = x_left[dy].round() as i32;
            let x_r = x_right[dy].round() as i32;

            let h_segment = interpolate(x_l, h_left[dy], x_r, h_right[dy]);
            let z_segment = interpolate(x_l, z_left[dy], x_r, z_right[dy]);

            for x in x_l..=x_r {
                let z = z_segment[(x - x_l) as usize];
                let h = h_segment[(x - x_l) as usize];

                let screen_index =
                    ((CANVAS_BOUND_Y as i32 - y) * CANVAS_WIDTH as i32) + CANVAS_BOUND_X as i32 + x;
                if z > self.depth_buffer[screen_index as usize] {
                    self.depth_buffer[screen_index as usize] = z;
                    let shaded_color = color.scaled(h);
                    self.put_pixel(vec2(x as f32, y as f32), &shaded_color);
                }
            }
        }
    }

    pub fn draw_wireframe_triangle(&mut self, p0: Vec2, p1: Vec2, p2: Vec2, color: &Color) {
        self.draw_line(p0, p1, color);
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p0, color);
    }

    pub fn render_instance(&mut self, instance: &Instance, transform: &Affine3A) {
        let mut projected = vec![];
        for v in instance.model.vertices.iter() {
            let v_t = transform.transform_point3(*v);
            let proj = project_vertex(v_t);
            projected.push(vec3(proj.x, proj.y, 1. / v_t.z));
        }

        for t in &instance.model.triangles {
            self.render_triangle(t, &projected);
        }
    }

    pub fn render_triangle(&mut self, triangle: &Triangle, projected: &[Vec3]) {
        self.draw_filled_triangle(
            projected[triangle.v0],
            projected[triangle.v1],
            projected[triangle.v2],
            (1., 1., 1.),
            &triangle.color,
        );
    }

    pub fn render_scene(&mut self, scene: Vec<Instance>) {
        let m_camera = Affine3A {
            translation: vec3a(0., 0., 0.),
            matrix3: Mat3A::from_rotation_y(0.),
        };

        let planes = vec![
            Plane::new(vec3(0., 0., 1.), -VIEWPORT_DISTANCE),
            Plane::new(vec3(FRAC_1_SQRT_2, 0., FRAC_1_SQRT_2), 0.),
            Plane::new(vec3(-FRAC_1_SQRT_2, 0., FRAC_1_SQRT_2), 0.),
            Plane::new(vec3(0., FRAC_1_SQRT_2, FRAC_1_SQRT_2), 0.),
            Plane::new(vec3(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2), 0.),
        ];

        let clipped_scene = clip_scene(scene, &planes);

        for instance in clipped_scene {
            let m = m_camera * instance.transform;
            self.render_instance(&instance, &m);
        }
    }
}
