/// Evaluator module - executes the AST
mod compiler;
mod core;
mod evaluator;
mod scope;
mod class;

// Re-export public API
pub use compiler::Compiler;
pub use core::*;
pub use evaluator::{Evaluator, ControlFlow, EvalResult, ErrorKind};
pub use scope::Scope;
pub use class::{Class, ClassInstance, BoundMethod, StaticMethod};

// Convenience function for backward compatibility with tests
pub fn evaluate(program: crate::core::Program) -> anyhow::Result<crate::core::Value> {
    let eval_ir = Compiler::compile(program)?;
    let mut evaluator = Evaluator::new();
    evaluator.eval(&eval_ir).map_err(|control_flow| {
        match control_flow {
            ControlFlow::Error(ErrorKind::SystemError(err)) => err,
            ControlFlow::Error(ErrorKind::RaisedError(value)) => {
                // Convert raised value to string for error display
                match value {
                    crate::core::Value::String(s) => anyhow::anyhow!("{}", s),
                    _ => anyhow::anyhow!("{:?}", value),
                }
            },
            ControlFlow::Return(val) => anyhow::anyhow!("Unexpected return: {:?}", val),
            ControlFlow::Break(val) => anyhow::anyhow!("Unexpected break: {:?}", val),
            ControlFlow::Continue => anyhow::anyhow!("Unexpected continue"),
        }
    })
}
