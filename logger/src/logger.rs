use std::sync::atomic::{AtomicUsize};
use crossbeam::channel::{Sender};
use std::sync::{Arc, OnceLock};
use crate::level::Level;
use crate::message::LogMessage;
use std::fmt::Arguments;
use crate::buffer::LogBuffer;

pub(crate) static LOGGER: OnceLock<Arc<Logger>> = OnceLock::new();

pub struct Logger {
    pub(crate) level: AtomicUsize,
    sender: Sender<LogMessage>,
    buffer: Arc<LogBuffer>,
}

pub fn init(level: Level,capacity: usize){
    let (tx, rx) = crossbeam::channel::bounded(1024);

    let logger = Arc::new(Logger {
        level: AtomicUsize::new(level as usize),
        sender: tx,
        buffer: Arc::new(LogBuffer::new(capacity)),
    });

    let logger_clone = logger.clone();

    let _ = LOGGER.set(logger);

    std::thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            logger_clone.buffer.push(msg);
        }
    });
}

pub fn log_internal(level: Level, args: Arguments) {
    if let Some(logger) = LOGGER.get() {
        let mut msg = String::with_capacity(128);
        use std::fmt::Write;
        msg.write_fmt(args).ok();

        let _ = logger.sender.try_send(LogMessage { level, msg });
    }
}

pub fn get_logs(level: Level) -> Vec<LogMessage> {
    if let Some(logger) = LOGGER.get() {
        logger.buffer.get_recent(level)
    } else {
        vec![]
    }
}

