#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub h: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
    pub h: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64, h: Option<f64>) -> Self {
        Point3D {
            x,
            y,
            z,
            h: h.unwrap_or(1.0),
        }
    }
}

impl Point2D {
    pub fn new(x: i32, y: i32, h: Option<f64>) -> Self {
        Point2D {
            x,
            y,
            h: h.unwrap_or(1.0),
        }
    }
}
