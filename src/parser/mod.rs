mod binary_ops;
/// Parser module - converts tokens to AST
mod core;
mod expression;
mod statement;
mod stream;

// Re-export public API
pub use core::Parser;
