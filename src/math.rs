use glam::{Vec3A, vec3a};

pub type Vec3 = Vec3A;

pub fn vec3(x: f32, y: f32, z: f32) -> Vec3A {
    vec3a(x, y, z)
}
