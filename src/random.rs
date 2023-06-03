use std::ops::Range;

use cgmath::{vec3, InnerSpace, Vector3};

pub fn f32(r: Range<f32>) -> f32 {
    r.start + (r.end - r.start) * fastrand::f32()
}

pub fn vec(r: Range<f32>) -> Vector3<f32> {
    vec3(f32(r.clone()), f32(r.clone()), f32(r))
}

/*pub fn in_unit_sphere() -> Vector3<f32> {
    let p = vec((-1.0)..1.0);
    let len = f32(0.0..1.0);
    p.normalize() * len
}*/

pub fn in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = vec((-1.0)..1.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}

pub fn unit_vector() -> Vector3<f32> {
    in_unit_sphere().normalize()
}

pub fn in_hemisphere(normal: Vector3<f32>) -> Vector3<f32> {
    let in_unit_sphere = in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn in_unit_disk() -> Vector3<f32> {
    loop {
        let p = vec3(f32((-1.0)..1.0), f32((-1.0)..1.0), 0.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}
