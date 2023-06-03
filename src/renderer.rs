use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

use crate::{
    color, material,
    object::{self, Hit},
    random, utils, vec3, Camera, Vec3,
};

pub struct Block {
    pub x: usize,
    pub y: usize,
    pub block_size: usize,
    pub sample: usize,
    pub block: Vec<Vec3>,
}

impl Block {
    pub fn draw(&self, buffer: &mut [u32], width: usize, height: usize) {
        let offset_x = self.x * self.block_size;
        let offset_y = self.y * self.block_size;

        for local_y in 0..self.block_size {
            let y = offset_y + local_y;

            if y >= height {
                break;
            }

            for local_x in 0..self.block_size {
                let x = offset_x + local_x;

                if x >= width {
                    break;
                }

                let color = self.block[local_y * self.block_size + local_x];
                let color = color::apply_sampling(color, self.sample + 1);
                buffer[y * width + x] = color::from_vec(color);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub samples_per_step: usize,
    pub ray_depth: usize,
    pub camera: Camera,
    pub world: Hit,
}

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

impl Default for Renderer {
    fn default() -> Self {
        let width = 1920;
        let height = 1080;
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

        Self {
            width,
            height,
            samples_per_pixel: 500,
            samples_per_step: 10,
            ray_depth: 50,
            camera,
            world,
        }
    }
}

impl Renderer {
    pub fn render_pixel(&self, x: usize, y: usize) -> Vec3 {
        let u = (x as f32 + fastrand::f32()) / (self.width - 1) as f32;
        let v = (y as f32 + fastrand::f32()) / (self.height - 1) as f32;
        let r = self.camera.get_ray(u, v);
        r.trace(&self.world, self.ray_depth)
    }

    pub fn render_block(&self, entry: &mut Block) {
        let block_size = entry.block_size;
        let block_x = entry.x;
        let block_y = entry.y;
        entry.block.iter_mut().enumerate().for_each(|(i, p)| {
            for _ in 0..self.samples_per_step {
                p.add_assign(self.render_pixel(
                    (i % block_size) + block_x * block_size,
                    (i / block_size) + block_y * block_size,
                ))
            }
        });
        entry.sample += self.samples_per_step;
    }
}
