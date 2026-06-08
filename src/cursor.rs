use std::time::{Duration, Instant};

pub struct Cursor {
    pub visible: bool,
    pub last_blink: Instant,
    pub delay_ms: u64,
}

impl Cursor {
    pub fn new() -> Self {
        Self { 
            visible: true,
            last_blink: Instant::now(),
            delay_ms: 500,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn update(&mut self) {
        if self.last_blink.elapsed() >= Duration::from_millis(self.delay_ms) {
            self.toggle();
            self.last_blink = Instant::now();
        }
    }
}
