use crate::colors::Color;
use crate::utils::{CANVAS_BOUND_X, CANVAS_BOUND_Y, CANVAS_HEIGHT, CANVAS_WIDTH, interpolate};
use crate::vertex::Point2D;
use std::panic;

pub struct Canvas {
    pub buffer: Vec<u32>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        Canvas {
            buffer: vec![0x1B1E1F; CANVAS_HEIGHT * CANVAS_WIDTH],
        }
    }

    fn put_pixel(&mut self, p: Point2D, color: &Color) {
        if p.x < -CANVAS_BOUND_X || p.x >= CANVAS_BOUND_X {
            panic!("{p:?} lies outside screen width!");
        }

        if p.y < -CANVAS_BOUND_Y || p.y >= CANVAS_BOUND_Y {
            panic!("{p:?} lies outside screen height!");
        }

        let screenx: usize = (CANVAS_BOUND_X + p.x) as usize;
        let screeny: usize = (CANVAS_BOUND_Y - p.y) as usize;
        self.buffer[screeny * CANVAS_WIDTH + screenx] = color.as_u32();
    }

    pub fn draw_line(&mut self, mut p0: Point2D, mut p1: Point2D, color: &Color) {
        if (p0.x - p1.x).abs() > (p0.y - p1.y).abs() {
            // Mostly horizontal line
            if p0.x > p1.x {
                std::mem::swap(&mut p0, &mut p1);
            }
            let ys = interpolate(p0.x, p0.y as f64, p1.x, p1.y as f64);
            for x in p0.x..=p1.x {
                let y = ys[(x - p0.x) as usize];
                self.put_pixel(Point2D::new(x, y.round() as i32, None), color);
            }
        } else {
            // Mostly vertical line
            if p0.y > p1.y {
                std::mem::swap(&mut p0, &mut p1);
            }
            let xs = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
            for y in p0.y..=p1.y {
                let x = xs[(y - p0.y) as usize];
                self.put_pixel(Point2D::new(x.round() as i32, y, None), color);
            }
        }
    }

    pub fn draw_filled_triangle(
        &mut self,
        mut p0: Point2D,
        mut p1: Point2D,
        mut p2: Point2D,
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

        let x01 = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
        let h01 = interpolate(p0.y, p0.h, p1.y, p1.h);

        let x12 = interpolate(p1.y, p1.x as f64, p2.y, p2.x as f64);
        let h12 = interpolate(p1.y, p1.h, p2.y, p2.h);

        let x02 = interpolate(p0.y, p0.x as f64, p2.y, p2.x as f64);
        let h02 = interpolate(p0.y, p0.h, p2.y, p2.h);

        let mut x012 = x01;
        x012.pop();
        x012.extend(x12);

        let mut h012 = h01;
        h012.pop();
        h012.extend(h12);

        let m = x02.len() / 2;
        let (x_left, h_left, x_right, h_right) = if x02[m] < x012[m] {
            (x02, h02, x012, h012)
        } else {
            (x012, h012, x02, h02)
        };

        for y in p0.y..=p2.y {
            let x_l = x_left[(y - p0.y) as usize].round() as i32;
            let x_r = x_right[(y - p0.y) as usize].round() as i32;

            let h_segment = interpolate(
                x_l,
                h_left[(y - p0.y) as usize],
                x_r,
                h_right[(y - p0.y) as usize],
            );

            for x in x_l..=x_r {
                let shaded_color = color.scaled(h_segment[(x - x_l) as usize]);
                self.put_pixel(Point2D::new(x, y, None), &shaded_color);
            }
        }
    }
}
