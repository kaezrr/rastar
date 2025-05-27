use crate::utils::{CANVAS_BOUND_X, CANVAS_BOUND_Y, CANVAS_HEIGHT, CANVAS_WIDTH};
use std::panic;

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    pub fn from(x: i32, y: i32) -> Self {
        Point2D { x, y }
    }
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn from(hexcode: u32) -> Self {
        Color {
            r: (hexcode >> 16) as u8,
            g: (hexcode >> 8) as u8,
            b: hexcode as u8,
        }
    }

    pub fn as_u32(&self) -> u32 {
        u32::from(self.r) << 16 | u32::from(self.g) << 8 | u32::from(self.b)
    }

    pub fn scale(&mut self, factor: f64) -> &Self {
        self.r = (self.r as f64 * factor).round().clamp(0.0, 255.0) as u8;
        self.g = (self.g as f64 * factor).round().clamp(0.0, 255.0) as u8;
        self.b = (self.b as f64 * factor).round().clamp(0.0, 255.0) as u8;
        self
    }
}

pub struct Canvas {
    pub buffer: Vec<u32>,
}

#[allow(dead_code)]
impl Canvas {
    pub fn new() -> Self {
        Canvas {
            buffer: vec![0x000000; CANVAS_HEIGHT * CANVAS_WIDTH],
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

    fn draw_line(&mut self, mut p0: Point2D, mut p1: Point2D, color: &Color) {
        if (p0.x - p1.x).abs() > (p0.y - p1.y).abs() {
            // Mostly horizontal line
            if p0.x > p1.x {
                std::mem::swap(&mut p0, &mut p1);
            }
            let ys = interpolate(p0.x, p0.y as f64, p1.x, p1.y as f64);
            for x in p0.x..=p1.x {
                let y = ys[(x - p0.x) as usize];
                self.put_pixel(Point2D::from(x, y.round() as i32), color);
            }
        } else {
            // Mostly vertical line
            if p0.y > p1.y {
                std::mem::swap(&mut p0, &mut p1);
            }
            let xs = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
            for y in p0.y..=p1.y {
                let x = xs[(y - p0.y) as usize];
                self.put_pixel(Point2D::from(x.round() as i32, y), color);
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
        let x12 = interpolate(p1.y, p1.x as f64, p2.y, p2.x as f64);
        let x02 = interpolate(p0.y, p0.x as f64, p2.y, p2.x as f64);

        let mut x012 = x01;
        x012.pop();
        x012.extend(x12);

        let m = x02.len() / 2;
        let (x_left, x_right) = if x02[m] < x012[m] {
            (x02, x012)
        } else {
            (x012, x02)
        };

        for y in p0.y..=p2.y {
            let leftx = x_left[(y - p0.y) as usize].round() as i32;
            let rightx = x_right[(y - p0.y) as usize].round() as i32;

            for x in leftx..=rightx {
                self.put_pixel(Point2D::from(x, y), color);
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
