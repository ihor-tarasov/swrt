use std::ops::Range;

use glam::{Vec3, vec3};

pub fn f32(r: Range<f32>) -> f32 {
    r.start + (r.end - r.start) * fastrand::f32()
}

pub fn vec(r: Range<f32>) -> Vec3 {
    vec3(f32(r.clone()), f32(r.clone()), f32(r))
}

pub fn in_unit_sphere() -> Vec3 {
    loop {
        let p = vec((-1.0)..1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn unit_vector() -> Vec3 {
    in_unit_sphere().normalize()
}

pub fn in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn in_unit_disk() -> Vec3 {
    loop {
        let p = vec3(f32((-1.0)..1.0), f32((-1.0)..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
