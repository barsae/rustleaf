// TODO: remove these after sufficient implementation is done
#![allow(dead_code)]
#![allow(unused_variables)]

/// RustLeaf interpreter library
pub mod core;
pub mod eval;
pub mod lexer;
pub mod parser;

use anyhow::Result;

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
            eval::ControlFlow::Error(err) => err,
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
