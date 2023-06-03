use std::{
    collections::HashSet,
    sync::{Arc, Mutex, RwLock},
    time::Duration, fs::File,
};

use swrt::{
    renderer::Block,
    utils, vec3, Renderer, Window,
};

fn main() {
    let renderer = if let Ok(file) = File::open("setup.json") {
        serde_json::de::from_reader(file).unwrap()
    } else {
        let renderer = Renderer::default();
        File::create("setup.json").and_then(|file| {
            serde_json::ser::to_writer_pretty(file, &renderer).unwrap();
            Ok(())
        }).unwrap();
        renderer
    };

    let renderer = Arc::new(RwLock::new(renderer));

    let threads = 12;

    let block_size = 64;

    let mut width_blocks_count = renderer.read().unwrap().width / block_size;
    if renderer.read().unwrap().width % block_size != 0 {
        width_blocks_count += 1;
    }

    let mut height_blocks_count = renderer.read().unwrap().height / block_size;
    if renderer.read().unwrap().height % block_size != 0 {
        height_blocks_count += 1;
    }

    let mut done_registry = HashSet::new();
    let mut blocks_to_render_queue = Vec::with_capacity(width_blocks_count * height_blocks_count);
    for y in 0..height_blocks_count {
        for x in 0..width_blocks_count {
            blocks_to_render_queue.push(Block {
                x,
                y,
                block_size,
                sample: 0,
                block: vec![vec3(0.0, 0.0, 0.0); block_size * block_size],
            });
            done_registry.insert((x, y));
        }
    }

    fastrand::shuffle(&mut blocks_to_render_queue);

    let blocks_to_render_queue = Arc::new(Mutex::new(blocks_to_render_queue));

    let (_handles, sender, receiver) = utils::spawn_threads(threads);

    let _task_sender = {
        let blocks_to_render_queue = blocks_to_render_queue.clone();
        let renderer = renderer.clone();
        std::thread::spawn(move || loop {
            let entry = blocks_to_render_queue.lock().unwrap().pop();
            if let Some(entry) = entry {
                sender.send((renderer.clone(), entry)).unwrap();
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        })
    };

    let mut buffer = vec![0u32; renderer.read().unwrap().width * renderer.read().unwrap().height];
    let mut window = Window::new("SwRt", renderer.read().unwrap().width, renderer.read().unwrap().height);

    let mut percent_done = 0.0;
    let step_value = 100.0
        / ((renderer.read().unwrap().samples_per_pixel / renderer.read().unwrap().samples_per_step) * width_blocks_count * height_blocks_count)
            as f32;

    while window.is_open() {
        let block = match receiver.recv_timeout(Duration::from_millis(16)) {
            Ok(data) => data,
            Err(error) => match error {
                std::sync::mpsc::RecvTimeoutError::Timeout => {
                    window.update(&buffer, renderer.read().unwrap().width, renderer.read().unwrap().height, percent_done);
                    continue;
                }
                std::sync::mpsc::RecvTimeoutError::Disconnected => panic!(),
            },
        };

        block.draw(&mut buffer, renderer.read().unwrap().width, renderer.read().unwrap().height);

        if block.sample < renderer.read().unwrap().samples_per_pixel {
            let mut blocks_to_render_queue = blocks_to_render_queue.lock().unwrap();
            let index = if blocks_to_render_queue.len() > 1 {
                fastrand::usize(0..blocks_to_render_queue.len())
            } else {
                0
            };
            blocks_to_render_queue.insert(index, block);
        } else {
            done_registry.remove(&(block.x, block.y));

            if done_registry.is_empty() {
                utils::save_result("result.png", &buffer, renderer.read().unwrap().width, renderer.read().unwrap().height);
                return;
            }
        }

        window.update(&buffer, renderer.read().unwrap().width, renderer.read().unwrap().height, percent_done);

        percent_done += step_value;
    }
}
