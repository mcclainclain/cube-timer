use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    pub running: bool,
    pub time: Duration,
    pub now: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            time: Duration::new(0, 0),
            running: false,
            now: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.now = Instant::now();
        self.running = true;
    }

    pub fn stop(&mut self) -> Duration {
        self.running = false;
        self.time = self.now.elapsed();
        return self.time;
    }

    pub fn reset(&mut self) {
        self.time = Duration::new(0, 0);
    }

    pub fn get_time(&mut self) -> Duration {
        if self.running {
            return self.now.elapsed();
        } else {
            return self.time;
        }
    }
}
