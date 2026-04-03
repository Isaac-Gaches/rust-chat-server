use rayon::iter::ParallelIterator;
use std::sync::{Mutex};
use std::collections::VecDeque;
use rayon::iter::IntoParallelRefIterator;
use crate::Level;
use crate::message::LogMessage;

pub struct LogBuffer {
    inner: Mutex<VecDeque<LogMessage>>,
    capacity: usize,
}

impl LogBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
        }
    }

    pub fn push(&self, msg: LogMessage) {
        let mut buf = self.inner.lock().unwrap();

        if buf.len() >= self.capacity {
            buf.pop_front(); // drop oldest
        }

        buf.push_back(msg);
    }

    pub fn get_recent(&self, level: Level) -> Vec<LogMessage> {
        let buf = self.inner.lock().unwrap();

        buf.par_iter()
            .filter(|m| m.level <= level)
            .cloned()
            .collect()
    }
}