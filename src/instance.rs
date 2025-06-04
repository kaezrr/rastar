use glam::Vec3;

use crate::model::Model;

pub struct Instance<'a> {
    pub model: &'a Model,
    pub transform: Transform,
}

pub struct Transform {
    pub scale: f64,
    pub rotation: f64,
    pub translation: Vec3,
}

impl Transform {
    pub fn get_transformed_vertex(&self, v: &Vec3) -> Vec3 {
        v + self.translation
    }
}
