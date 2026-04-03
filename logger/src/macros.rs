#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        if $crate::enabled($level) {
            $crate::log_internal($level, format_args!($($arg)*));
        }
    };
}