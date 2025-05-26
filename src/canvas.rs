use crate::utils::{CANVAS_BOUND_X, CANVAS_BOUND_Y, CANVAS_HEIGHT, CANVAS_WIDTH};
use std::panic;

#[derive(Debug)]
pub struct Point2D(pub i32, pub i32);

pub struct Color(pub u32, pub u32, pub u32);

impl Color {
    pub fn as_u32(&self) -> u32 {
        (self.0 << 16) | (self.1 << 8) | self.2
    }
}

pub struct Canvas {
    pub buffer: Vec<u32>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            buffer: vec![0x000000; CANVAS_HEIGHT * CANVAS_WIDTH],
        }
    }

    fn put_pixel(&mut self, p: Point2D, color: &Color) {
        if p.0 < -CANVAS_BOUND_X || p.0 >= CANVAS_BOUND_X {
            panic!("{p:?} lies outside screen width!");
        }

        if p.1 < -CANVAS_BOUND_Y || p.1 >= CANVAS_BOUND_Y {
            panic!("{p:?} lies outside screen height!");
        }

        let screenx: usize = (CANVAS_BOUND_X + p.0) as usize;
        let screeny: usize = (CANVAS_BOUND_Y - p.1) as usize;
        self.buffer[screeny * CANVAS_WIDTH + screenx] = color.as_u32();
    }

    pub fn draw_line(&mut self, mut p0: Point2D, mut p1: Point2D, color: &Color) {
        let (x0, y0) = (p0.0 as f64, p0.1 as f64);
        let (x1, y1) = (p1.0 as f64, p1.1 as f64);

        if (x0 - x1).abs() > (y0 - y1).abs() {
            if p0.0 > p1.0 {
                std::mem::swap(&mut p0, &mut p1);
            }

            let ys = interpolate(p0.0, y0, p1.0, y1);
            for x in p0.0..=p1.0 {
                let y = ys[(x - p0.0) as usize];
                self.put_pixel(Point2D(x, y.round() as i32), color);
            }
        } else {
            if p0.1 > p1.1 {
                std::mem::swap(&mut p0, &mut p1);
            }

            let xs = interpolate(p0.1, x0, p1.1, x1);
            for y in p0.1..=p1.1 {
                let x = xs[(y - p0.1) as usize];
                self.put_pixel(Point2D(x.round() as i32, y), color);
            }
        }
    }
}

fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> Vec<f64> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut values = Vec::new();
    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut b = d0;
    for _ in i0..=i1 {
        values.push(b);
        b += a;
    }
    values
}
