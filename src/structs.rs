use glam::{Affine3A, Vec3, vec3};

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn new(hexcode: u32) -> Self {
        Color {
            r: (hexcode >> 16) as u8,
            g: (hexcode >> 8) as u8,
            b: hexcode as u8,
        }
    }

    pub fn as_u32(&self) -> u32 {
        u32::from(self.r) << 16 | u32::from(self.g) << 8 | u32::from(self.b)
    }

    pub fn scaled(&self, factor: f64) -> Self {
        Color {
            r: (self.r as f64 * factor).round().clamp(0.0, 255.0) as u8,
            g: (self.g as f64 * factor).round().clamp(0.0, 255.0) as u8,
            b: (self.b as f64 * factor).round().clamp(0.0, 255.0) as u8,
        }
    }
}

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
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance<'a> {
    pub model: &'a Model,
    pub transform: Affine3A,
}

impl<'a> Instance<'a> {
    pub fn new(model: &'a Model, scale: f32, rotation: f32, translation: Vec3) -> Self {
        Instance {
            model,
            transform: Affine3A::from_translation(translation)
                * Affine3A::from_rotation_y(rotation.to_radians())
                * Affine3A::from_scale(vec3(scale, scale, scale)),
        }
    }
}
