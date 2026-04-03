use std::time::{Instant, Duration};

pub struct RateLimiter {
    max: usize,
    window: Duration,
    timestamps: Vec<Instant>,
}

impl RateLimiter {
    pub fn new(max: usize, window: Duration) -> Self {
        Self {
            max,
            window,
            timestamps: Vec::new(),
        }
    }

    pub fn allow(&mut self) -> bool {
        let now = Instant::now();

        self.timestamps
            .retain(|&t| now.duration_since(t) < self.window);

        if self.timestamps.len() >= self.max {
            return false;
        }

        self.timestamps.push(now);
        true
    }
}

#[test]
fn test_rate_limiter() {
    let mut limiter = RateLimiter::new(2, Duration::from_secs(10));

    assert!(limiter.allow());
    assert!(limiter.allow());
    assert!(!limiter.allow());
}