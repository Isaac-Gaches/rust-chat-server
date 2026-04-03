mod logger;
mod level;
mod macros;
mod message;
mod buffer;

pub use message::enabled;
pub use logger::log_internal;
pub use logger::init;
pub use level::Level;
pub use logger::get_logs;