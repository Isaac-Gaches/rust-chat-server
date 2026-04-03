#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}