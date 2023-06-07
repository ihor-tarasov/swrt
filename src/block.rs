use std::ops::AddAssign;

use rt::{Vec3, Renderer};

use crate::color;

pub struct Block {
    pub x: usize,
    pub y: usize,
    pub block_size: usize,
    pub sample: usize,
    pub block: Vec<Vec3>,
    pub samples_to_do: usize,
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

    pub fn render(&mut self, renderer: &Renderer) {
        let block_size = self.block_size;
        let block_x = self.x;
        let block_y = self.y;
        self.block.iter_mut().enumerate().for_each(|(i, p)| {
            for _ in 0..self.samples_to_do {
                p.add_assign(renderer.render_pixel(
                    ((i % block_size) + block_x * block_size) as f32,
                    ((i / block_size) + block_y * block_size) as f32,
                ))
            }
        });
        self.sample += self.samples_to_do;
    }
}
