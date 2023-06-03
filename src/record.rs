use cgmath::{vec3, InnerSpace, Vector3};

use crate::{material::{self, Material}, Ray};

#[derive(Clone)]
pub struct Record {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
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
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}
