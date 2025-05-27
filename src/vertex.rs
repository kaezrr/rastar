#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub h: f64,
}

impl Vertex {
    pub fn new(x: i32, y: i32, h: Option<f64>) -> Self {
        Vertex {
            x,
            y,
            h: h.unwrap_or(1.0),
        }
    }
}
