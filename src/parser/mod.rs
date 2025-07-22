/// Parser module - converts tokens to AST
mod core;
mod statement;
mod expression;
mod binary_ops;

// Re-export public API
pub use core::Parser;
