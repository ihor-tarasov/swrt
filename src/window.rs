use std::time::Duration;

use minifb::{Key, ScaleMode, WindowOptions};

use crate::Timer;

pub struct Window {
    window: minifb::Window,
    timer: Timer,
    frames: u32,
    title: Box<str>,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let options = WindowOptions {
            resize: true,
            scale_mode: ScaleMode::Stretch,
            ..Default::default()
        };

        Self {
            window: minifb::Window::new(title, width, height, options).unwrap(),
            timer: Timer::new(Duration::from_secs(1)),
            frames: 0,
            title: title.to_string().into_boxed_str(),
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn update(&mut self, buffer: &[u32], width: usize, height: usize, percent_done: f32) {
        self.window
            .update_with_buffer(buffer, width, height)
            .unwrap();

        self.frames = self.frames.wrapping_add(1);
        if self.timer.update() {
            self.window
                .set_title(format!("{} | FPS: {} | Progress: {:.2}%", self.title, self.frames, percent_done).as_str());
            self.frames = 0;
        }
    }
}
