use std::sync::atomic::Ordering;
use crate::level::Level;

#[derive(Clone, Debug)]
pub struct LogMessage {
    pub level: Level,
    pub msg: String,
}

#[inline(always)]
pub fn enabled(level: Level) -> bool {
    if let Some(logger) = crate::logger::LOGGER.get() {
        (level as usize) <= logger.level.load(Ordering::Relaxed)
    } else {
        false
    }
}