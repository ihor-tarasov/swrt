use std::{
    collections::HashSet,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use swrt::{
    material,
    object::{self, Hit},
    random,
    renderer::Block,
    utils, Camera, Renderer, Window, vec3,
};

fn random_scene() -> Hit {
    let mut world = object::hit_list();

    let ground_material = material::lambertian(vec3(0.5, 0.5, 0.5));
    world.push(object::sphere(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in (-11)..11 {
        for b in (-11)..11 {
            let choose_mat = fastrand::f32();
            let center = vec3(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );

            if (center - vec3(4.0, 0.2, 0.0)).length_squared() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = utils::mul_per_comp(random::vec(0.0..1.0), random::vec(0.0..1.0));
                    sphere_material = material::lambertian(albedo);
                    world.push(object::sphere(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random::vec(0.5..1.0);
                    let fuzz = random::f32(0.0..0.5);
                    sphere_material = material::metal(albedo, fuzz);
                    world.push(object::sphere(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = material::dielectric(1.5);
                    world.push(object::sphere(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = material::dielectric(1.5);
    world.push(object::sphere(vec3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = material::lambertian(vec3(0.4, 0.2, 0.1));
    world.push(object::sphere(vec3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = material::metal(vec3(0.7, 0.6, 0.5), 0.0);
    world.push(object::sphere(vec3(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn main() {
    let width = 1920;
    let height = 1080;
    let samples_per_pixel = 120;
    let ray_depth = 10;
    let samples_per_step = 60;
    let threads = 12;

    let block_size = 64;

    let mut width_blocks_count = width / block_size;
    if width % block_size != 0 {
        width_blocks_count += 1;
    }

    let mut height_blocks_count = height / block_size;
    if height % block_size != 0 {
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

    let world = random_scene();

    let lookfrom = vec3(13.0, 2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        vec3(0.0, 1.0, 0.0),
        20.0,
        width as f32,
        height as f32,
        0.1,
        10.0,
    );

    let renderer = Arc::new(RwLock::new(Renderer {
        width,
        height,
        camera,
        world,
        ray_depth,
        samples_per_step,
    }));

    let (_handles, sender, receiver) = utils::spawn_threads(threads);

    let _task_sender = {
        let blocks_to_render_queue = blocks_to_render_queue.clone();
        std::thread::spawn(move || loop {
            let entry = blocks_to_render_queue.lock().unwrap().pop();
            if let Some(entry) = entry {
                sender.send((renderer.clone(), entry)).unwrap();
            } else {
                std::thread::sleep(Duration::from_millis(100));
            }
        })
    };

    let mut buffer = vec![0u32; width * height];
    let mut window = Window::new("SwRt", width, height);

    let mut percent_done = 0.0;
    let step_value = 100.0
        / ((samples_per_pixel / samples_per_step) * width_blocks_count * height_blocks_count)
            as f32;

    while window.is_open() {
        let block = match receiver.recv_timeout(Duration::from_millis(16)) {
            Ok(data) => data,
            Err(error) => match error {
                std::sync::mpsc::RecvTimeoutError::Timeout => {
                    window.update(&buffer, width, height, percent_done);
                    continue;
                }
                std::sync::mpsc::RecvTimeoutError::Disconnected => panic!(),
            },
        };

        block.draw(&mut buffer, width, height);

        if block.sample < samples_per_pixel {
            {
                let mut blocks_to_render_queue = blocks_to_render_queue.lock().unwrap();
                let index = if blocks_to_render_queue.len() > 1 {
                    fastrand::usize(0..blocks_to_render_queue.len())
                } else {
                    0
                };
                blocks_to_render_queue.insert(index, block);
            }
        } else {
            done_registry.remove(&(block.x, block.y));

            if done_registry.is_empty() {
                utils::save_result("result.png", &buffer, width, height);
                return;
            }
        }

        window.update(&buffer, width, height, percent_done);

        percent_done += step_value;
    }
}
