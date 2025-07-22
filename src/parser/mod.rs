mod binary_ops;
mod block;
/// Parser module - converts tokens to AST
mod core;
mod expression;
mod statement;

// Re-export public API
pub use core::Parser;
