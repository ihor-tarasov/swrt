use std::ops::AddAssign;
use crate::{color, object::Hit, Camera, Vec3};

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

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    pub camera: Camera,
    pub world: Hit,
    pub ray_depth: usize,
    pub samples_per_step: usize,
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
