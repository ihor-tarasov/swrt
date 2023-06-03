use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread::JoinHandle,
};

use crate::{renderer::Block, Renderer, Vec3, vec3};

pub fn near_zero(v: Vec3) -> bool {
    const S: f32 = 1e-8;
    v.x.abs() < S && v.y.abs() < S && v.z.abs() < S
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn mul_per_comp(a: Vec3, b: Vec3) -> Vec3 {
    vec3(a.x * b.x, a.y * b.y, a.z * b.z)
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn spawn_threads(
    count: usize,
) -> (
    Vec<JoinHandle<()>>,
    Sender<(Arc<RwLock<Renderer>>, Block)>,
    Receiver<Block>,
) {
    let mut handles = Vec::new();
    let (sender, out_receiver) = std::sync::mpsc::channel::<(Arc<RwLock<Renderer>>, Block)>();
    let (out_sender, receiver) = std::sync::mpsc::channel();
    let out_receiver = Arc::new(Mutex::new(out_receiver));

    for _ in 0..count {
        let out_receiver = out_receiver.clone();
        let out_sender = out_sender.clone();
        handles.push(std::thread::spawn(move || loop {
            let (renderer, mut block) = out_receiver.lock().unwrap().recv().unwrap();
            renderer.read().unwrap().render_block(&mut block);
            out_sender.send(block).unwrap();
        }));
    }

    (handles, sender, receiver)
}

pub fn save_result(path: &str, buffer: &[u32], width: usize, height: usize) {
    let mut buf = Vec::with_capacity(width * height * 3);
    buffer
        .iter()
        .for_each(|p| p.to_be_bytes().iter().skip(1).for_each(|b| buf.push(*b)));
    image::save_buffer(
        path,
        &buf,
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}
