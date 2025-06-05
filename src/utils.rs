use glam::{Vec2, Vec3, vec2};

use crate::structs::{Instance, Plane, Triangle};
use arrayvec::ArrayVec;

pub const CANVAS_WIDTH: f32 = 640.;
pub const CANVAS_HEIGHT: f32 = 640.;
pub const CANVAS_BOUND_X: f32 = CANVAS_WIDTH / 2.;
pub const CANVAS_BOUND_Y: f32 = CANVAS_HEIGHT / 2.;

pub const VIEWPORT_HEIGHT: f32 = 1.;
pub const VIEWPORT_WIDTH: f32 = 1.;
pub const VIEWPORT_DISTANCE: f32 = 1.;

pub fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = vec![];
    let a = (d1 - d0) / (i1 - i0) as f32;
    for i in i0..=i1 {
        let value = d0 + a * (i - i0) as f32;
        values.push(value);
    }
    values
}

pub fn project_vertex(v: Vec3) -> Vec2 {
    vec2(
        (v.x * VIEWPORT_DISTANCE * CANVAS_WIDTH) / (VIEWPORT_WIDTH * v.z),
        (v.y * VIEWPORT_DISTANCE * CANVAS_HEIGHT) / (VIEWPORT_HEIGHT * v.z),
    )
}

fn signed_distance(p: &Plane, v: Vec3) -> f32 {
    v.dot(p.normal) + p.d
}

fn intersect(p: &Plane, a: Vec3, b: Vec3) -> Vec3 {
    let t = (-p.d - p.normal.dot(a)) / p.normal.dot(b - a);
    a + t * (b - a)
}

fn clip_triangle(vertices: &mut Vec<Vec3>, t: &Triangle, plane: &Plane) -> ArrayVec<Triangle, 2> {
    let mut point_distances = [t.v0, t.v1, t.v2].map(|v| (signed_distance(plane, vertices[v]), v));
    point_distances.sort_unstable_by(|p0, p1| p0.0.total_cmp(&p1.0));

    let mut result = ArrayVec::new();
    let color = t.color.as_u32();

    if point_distances[0].0.is_sign_positive() {
        result.push(*t);
    } else if point_distances[1].0.is_sign_positive() {
        let [(_, c), (_, a), (_, b)] = point_distances;

        let a1 = vertices.len();
        vertices.push(intersect(plane, vertices[a], vertices[c]));

        let b1 = vertices.len();
        vertices.push(intersect(plane, vertices[b], vertices[c]));

        result.push(Triangle::new(a, b, a1, color));
        result.push(Triangle::new(a1, b, b1, color));
    } else if point_distances[2].0.is_sign_positive() {
        let [(_, a), (_, b), (_, c)] = point_distances;

        let b1 = vertices.len();
        vertices.push(intersect(plane, vertices[a], vertices[b]));

        let c1 = vertices.len();
        vertices.push(intersect(plane, vertices[a], vertices[c]));

        result.push(Triangle::new(a, b1, c1, color));
    }

    result
}

fn clip_triangles_against_plane(
    vertices: &mut Vec<Vec3>,
    triangles: Vec<Triangle>,
    plane: &Plane,
) -> Vec<Triangle> {
    let mut clipped_triangles = vec![];
    for t in triangles {
        clipped_triangles.extend(clip_triangle(vertices, &t, plane));
    }
    clipped_triangles
}

fn clip_instance_against_plane(mut instance: Instance, plane: &Plane) -> Option<Instance> {
    let dist = signed_distance(plane, instance.bounding_sphere.center);
    let radi = instance.bounding_sphere.radius;
    if dist > radi {
        Some(instance)
    } else if dist < -radi {
        None
    } else {
        instance.model.triangles = clip_triangles_against_plane(
            &mut instance.model.vertices,
            instance.model.triangles,
            plane,
        );
        Some(instance)
    }
}

fn clip_instance(mut instance: Instance, planes: &[Plane]) -> Option<Instance> {
    for p in planes {
        instance = clip_instance_against_plane(instance, p)?;
    }
    Some(instance)
}

pub fn clip_scene(scene: Vec<Instance>, planes: &[Plane]) -> Vec<Instance> {
    let mut clipped_scene = vec![];
    for instance in scene {
        clipped_scene.extend(clip_instance(instance, planes));
    }
    clipped_scene
}
