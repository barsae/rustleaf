#[cfg(feature = "parser-tracing")]
use log;

#[cfg(feature = "parser-tracing")]
#[macro_export]
macro_rules! trace_enter {
    ($func:expr) => {
        log::trace!("→ ENTER {}", $func);
    };
    ($func:expr, $($arg:tt)*) => {
        log::trace!("→ ENTER {} | {}", $func, format_args!($($arg)*));
    };
}

#[cfg(feature = "parser-tracing")]
#[macro_export]
macro_rules! trace_exit {
    ($func:expr) => {
        log::trace!("← EXIT  {}", $func);
    };
    ($func:expr, $result:expr) => {{
        let __result = $result;
        log::trace!("← EXIT  {} | result: {:?}", $func, &__result);
        __result
    }};
}

#[cfg(feature = "parser-tracing")]
#[macro_export]
macro_rules! trace_fail {
    ($func:expr, $err:expr) => {
        log::trace!("✗ FAIL  {} | error: {}", $func, $err);
    };
}

#[cfg(feature = "parser-tracing")]
#[macro_export]
macro_rules! trace_backtrack {
    ($func:expr) => {
        log::trace!("↩ BACK  {} | backtracking", $func);
    };
}

#[cfg(feature = "parser-tracing")]
#[macro_export]
macro_rules! trace_token {
    ($action:expr, $token:expr) => {
        log::trace!("  TOKEN {} | {:?}", $action, $token);
    };
}

#[cfg(not(feature = "parser-tracing"))]
#[macro_export]
macro_rules! trace_enter {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "parser-tracing"))]
#[macro_export]
macro_rules! trace_exit {
    ($func:expr) => {};
    ($func:expr, $result:expr) => {{ $result }};
}

#[cfg(not(feature = "parser-tracing"))]
#[macro_export]
macro_rules! trace_fail {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "parser-tracing"))]
#[macro_export]
macro_rules! trace_backtrack {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "parser-tracing"))]
#[macro_export]
macro_rules! trace_token {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "parser-tracing")]
pub fn init_tracing() {
    // Default to maximum verbosity for parser tracing
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();
    log::info!("Parser tracing initialized with RUST_LOG={}", std::env::var("RUST_LOG").unwrap_or_default());
}

#[cfg(not(feature = "parser-tracing"))]
pub fn init_tracing() {}