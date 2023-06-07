use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, Mutex, RwLock,
    },
    thread::JoinHandle,
};

use rt::Renderer;

use crate::block::Block;

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
            block.render(&renderer.read().unwrap());
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
