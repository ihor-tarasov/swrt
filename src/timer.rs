use std::time::{Instant, Duration};

pub struct Timer {
    counter: Instant,
    current: Duration,
    tick: Duration,
}

impl Timer {
    pub fn new(tick: Duration) -> Self {
        Self {
            counter: Instant::now(),
            current: Duration::from_secs(0),
            tick,
        }
    }

    pub fn update(&mut self) -> bool {
        let elapsed = self.counter.elapsed();
        self.counter = Instant::now();
        self.current += elapsed;
        if self.current >= self.tick {
            while self.current >= self.tick {
                self.current -= self.tick;
            }
            true
        } else {
            false
        }
    }
}
