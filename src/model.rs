use crate::{colors::Color, vertex::Point3D};

pub struct Triangle {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub color: Color,
}

impl Triangle {
    pub fn new(v1: usize, v2: usize, v3: usize, color: u32) -> Self {
        Triangle {
            v1,
            v2,
            v3,
            color: Color::new(color),
        }
    }
}

pub struct Model {
    pub name: String,
    pub vertices: Vec<Point3D>,
    pub triangles: Vec<Triangle>,
}
