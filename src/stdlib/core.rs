use std::time::Instant;

pub struct Clock {
    start_time: Instant,
    last_time: Instant,
    delta_time: f32,
    elapsed_time: f32,
}

impl Clock {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_time: now,
            delta_time: 0.0,
            elapsed_time: 0.0,
        }
    }

    /// Updates the clock and calculates delta time
    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_time);
        self.last_time = now;
        
        self.delta_time = delta.as_secs_f32();
        self.elapsed_time = now.duration_since(self.start_time).as_secs_f32();
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub fn elapsed_time(&self) -> f32 {
        self.elapsed_time
    }
}
