/// Parser tracing module - provides trace! macro for debugging parser operations

/// Trace macro that works like println! but writes to PRINT_CAPTURE when available
/// When the parser-tracing feature is disabled, this becomes a no-op
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "parser-tracing")]
        {
            let msg = format!($($arg)*);
            crate::core::write_to_print_capture(msg);
        }
        #[cfg(not(feature = "parser-tracing"))]
        {
            // No-op when feature is disabled
        }
    };
}
