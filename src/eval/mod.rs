mod class;
mod compiler;
mod constructors;
mod evaluator;
mod function;
mod params;
mod scope;
mod structs;
mod type_constant;

pub use class::{BoundMethod, Class, ClassInstance, StaticMethod};
pub use compiler::Compiler;
pub use constructors::Eval;
pub use evaluator::{ControlFlow, ErrorKind, EvalResult, Evaluator};
pub use function::RustLeafFunction;
pub use params::Params;
pub use scope::Scope;
pub use structs::*;
pub use type_constant::TypeConstant;

// Convenience function for backward compatibility with tests
pub fn evaluate(program: crate::core::Program) -> anyhow::Result<crate::core::Value> {
    evaluate_with_dir(program, None)
}

// Evaluate with optional custom current directory
pub fn evaluate_with_dir(
    program: crate::core::Program,
    current_dir: Option<std::path::PathBuf>,
) -> anyhow::Result<crate::core::Value> {
    let eval_ir = Compiler::compile(program)?;
    let mut evaluator = match current_dir {
        Some(dir) => Evaluator::new_with_dir(dir),
        None => Evaluator::new(),
    };
    let result = evaluator.eval(&eval_ir);
    result.map_err(|control_flow| {
        match control_flow {
            ControlFlow::Error(ErrorKind::SystemError(err)) => err,
            ControlFlow::Error(ErrorKind::RaisedError(value)) => {
                // Convert raised value to string for error display
                match value {
                    crate::core::Value::String(s) => anyhow::anyhow!("{}", s),
                    _ => anyhow::anyhow!("{:?}", value),
                }
            }
            ControlFlow::Return(val) => anyhow::anyhow!("Unexpected return: {:?}", val),
            ControlFlow::Break(val) => anyhow::anyhow!("Unexpected break: {:?}", val),
            ControlFlow::Continue => anyhow::anyhow!("Unexpected continue"),
        }
    })
}
