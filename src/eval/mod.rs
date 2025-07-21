/// Evaluator module - executes the AST
mod evaluator;
mod scope;

// Re-export public API
pub use evaluator::{evaluate, Evaluator};
pub use scope::Scope;