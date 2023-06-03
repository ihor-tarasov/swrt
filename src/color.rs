use crate::{Vec3, vec3};

pub fn from_rgb(r: u8, g: u8, b: u8) -> u32 {
    u32::from_be_bytes([0xFF, r, g, b])
}

pub fn from_vec_255(color: Vec3) -> u32 {
    from_rgb(color.x as u8, color.y as u8, color.z as u8)
}

fn clamp(color: Vec3) -> Vec3 {
    vec3(
        color.x.clamp(0.0, 0.999),
        color.y.clamp(0.0, 0.999),
        color.z.clamp(0.0, 0.999),
    )
}

pub fn from_vec(color: Vec3) -> u32 {
    from_vec_255(clamp(color) * 256.0)
}

pub fn apply_sampling(color: Vec3, samples: usize) -> Vec3 {
    let v = color * (1.0 / samples as f32);
    vec3(v.x.sqrt(), v.y.sqrt(), v.z.sqrt())
}
