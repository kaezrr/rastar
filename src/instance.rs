use crate::model::Model;
use crate::vertex::Point3D;

pub struct Instance<'a> {
    pub model: &'a Model,
    pub transform: Transform,
}

pub struct Transform {
    pub scale: f64,
    pub rotation: f64,
    pub translation: Point3D,
}

impl Transform {
    pub fn get_transformed_vertex(&self, v: &Point3D) -> Point3D {
        let mut result = *v;
        result.x *= self.scale;
        result.y *= self.scale;
        result.z *= self.scale;

        result.x += self.translation.x;
        result.y += self.translation.y;
        result.z += self.translation.z;

        result
    }
}
