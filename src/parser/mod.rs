/// Parser module - converts tokens to AST with operator desugaring
mod core;

// Re-export public API
pub use core::Parser;