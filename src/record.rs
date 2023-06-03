use glam::{Vec3, vec3};

use crate::{material::{self, Material}, Ray};

#[derive(Clone)]
pub struct Record {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Material,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            point: vec3(0.0, 0.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: material::lambertian(vec3(0.8, 0.8, 0.0)),
        }
    }
}

impl Record {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
