use glam::{Affine3A, Vec3, vec3};

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
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

    pub fn scaled(&self, factor: f32) -> Self {
        Color {
            r: (self.r as f32 * factor).round().clamp(0.0, 255.0) as u8,
            g: (self.g as f32 * factor).round().clamp(0.0, 255.0) as u8,
            b: (self.b as f32 * factor).round().clamp(0.0, 255.0) as u8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    pub v0: usize,
    pub v1: usize,
    pub v2: usize,
    pub color: Color,
}

impl Triangle {
    pub fn new(v0: usize, v1: usize, v2: usize, color: u32) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            color: Color::new(color),
        }
    }
}

#[derive(Clone)]
pub struct Model {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
}

pub struct Instance {
    pub model: Model,
    pub transform: Affine3A,
    pub bounding_sphere: BoundingSphere,
}

impl Instance {
    pub fn new(model: Model, scale: f32, rotation: f32, translation: Vec3) -> Self {
        let mut instance = Instance {
            model,
            transform: Affine3A::from_translation(translation)
                * Affine3A::from_rotation_y(rotation.to_radians())
                * Affine3A::from_scale(vec3(scale, scale, scale)),
            bounding_sphere: BoundingSphere {
                center: Vec3::ZERO,
                radius: 0.,
            },
        };

        let transformed_vertices: Vec<Vec3> = instance
            .model
            .vertices
            .iter()
            .map(|v| instance.transform.transform_point3(*v))
            .collect();

        let center =
            transformed_vertices.iter().copied().sum::<Vec3>() / transformed_vertices.len() as f32;

        let radius = transformed_vertices
            .iter()
            .map(|v| (v - center).length())
            .reduce(f32::max)
            .unwrap();

        instance.bounding_sphere.center = center;
        instance.bounding_sphere.radius = radius;
        instance
    }
}

pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    pub fn new(normal: Vec3, d: f32) -> Self {
        Plane { normal, d }
    }
}
