/// Evaluator module - executes the AST
mod compiler;
mod core;
mod evaluator;
mod scope;

// Re-export public API
pub use compiler::Compiler;
pub use core::*;
pub use evaluator::Evaluator;
pub use scope::Scope;

// Convenience function for backward compatibility with tests
pub fn evaluate(program: crate::core::Program) -> anyhow::Result<crate::core::Value> {
    let eval_ir = Compiler::compile(program)?;
    let mut evaluator = Evaluator::new();
    evaluator.eval(&eval_ir)
}
