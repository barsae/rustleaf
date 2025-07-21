/// Parser module - converts tokens to AST with operator desugaring
mod parser;

// Re-export public API
pub use parser::Parser;