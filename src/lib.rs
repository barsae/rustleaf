/// RustLeaf interpreter library
pub mod core;
pub mod eval;
pub mod lexer;
pub mod parser;

// Re-export proc macros
pub use rustleaf_macros::rust_value_any;

use anyhow::Result;
use std::fs;
use std::path::Path;

/// Evaluate a RustLeaf expression from a string and return the result
pub fn eval_str(source: &str) -> Result<core::Value> {
    run(source.to_string())
}

/// Evaluate a RustLeaf file and return the result
pub fn eval_file<P: AsRef<Path>>(path: P) -> Result<core::Value> {
    let source = fs::read_to_string(path)?;
    run(source)
}

/// Run a RustLeaf program from source code
pub fn run(source: String) -> Result<core::Value> {
    // Parsing (includes lexical analysis)
    let ast = parser::Parser::parse_str(&source)?;

    // Compilation to evaluation IR
    let eval_ir = eval::Compiler::compile(ast)?;

    // Evaluation
    let mut evaluator = eval::Evaluator::new();
    evaluator.eval(&eval_ir).map_err(|control_flow| {
        match control_flow {
            eval::ControlFlow::Error(eval::ErrorKind::SystemError(err)) => err,
            eval::ControlFlow::Error(eval::ErrorKind::RaisedError(value)) => {
                // Convert raised value to string for error display
                match value {
                    core::Value::String(s) => anyhow::anyhow!("{}", s),
                    _ => anyhow::anyhow!("{:?}", value),
                }
            }
            eval::ControlFlow::Return(val) => anyhow::anyhow!("Unexpected return: {:?}", val),
            eval::ControlFlow::Break(val) => anyhow::anyhow!("Unexpected break: {:?}", val),
            eval::ControlFlow::Continue => anyhow::anyhow!("Unexpected continue"),
        }
    })
}

/// Run a RustLeaf program and print the result
pub fn run_and_print(source: String) -> Result<()> {
    let result = run(source)?;

    // Only print non-unit values
    if !matches!(result, core::Value::Unit) {
        println!("{:?}", result);
    }

    Ok(())
}
