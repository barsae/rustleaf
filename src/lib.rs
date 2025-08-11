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
    let mut evaluator = eval::Evaluator::new();
    evaluator.eval_str(source)
}

/// Evaluate a RustLeaf file and return the result
pub fn eval_file<P: AsRef<Path>>(path: P) -> Result<core::Value> {
    let source = fs::read_to_string(path)?;
    run(source)
}

/// Run a RustLeaf program from source code
pub fn run(source: String) -> Result<core::Value> {
    let mut evaluator = eval::Evaluator::new();
    evaluator.eval_str(&source)
}

/// Run a RustLeaf program and print the result
pub fn run_and_print(source: String) -> Result<()> {
    let result = run(source)?;

    // Only print non-unit values
    if !matches!(result, core::Value::Unit) {
        println!("{result:?}");
    }

    Ok(())
}
