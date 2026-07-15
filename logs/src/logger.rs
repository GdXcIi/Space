use crate::timestamp::timestamp::get_timestamp;

use core::fmt;
use spin::Mutex;

#[derive(Debug, Clone, Copy, PartialEq)]
pun enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
    Panic
}

pub struct LogManager {
    buffer: Mutex<Vec<u8>>,     // for now for 0.1.0 we log only in ram
}

impl LogManager {
    pub const fn new() -> Self {
        Self {
            buffer: Mutex::new(Vec::new())
        }
    }

    pub fn log(&self, level: LogLevel, module: &str, message: &str) {
        let timestamp = self.get_timestamp();   // To implement later
        let log_entry = format!("[{}] [{:?}] [{}] {}", timestamp, level, module, message);
        let mut buffer = self.buffer.lock();

        buffer.extend_from_slice(log_entry.as_bytes());
    }
}
