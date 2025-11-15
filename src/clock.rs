use std::time::Instant;

pub struct Clock {
    rate: f64,
    instant: Instant,
    pub count: f64,
}

impl Clock {
    pub fn new(per_second: f64) -> Clock {
        Clock {rate: 1./per_second, count: 0., instant: Instant::now()}
    }
    pub fn ack(&mut self) {
        self.count += 1.
    }
    pub fn rdy(&self) -> bool {
        self.count * self.rate < self.instant.elapsed().as_secs_f64()
    }
}