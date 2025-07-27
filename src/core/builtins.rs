use super::Value;
use crate::core::{Args, RustValue};

use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::sync::Mutex;

// Global capture for print output during testing
static PRINT_CAPTURE: Mutex<Option<Vec<String>>> = Mutex::new(None);

// Thread-local capture for assertion counting during testing
thread_local! {
    static ASSERTION_COUNT: RefCell<Option<u32>> = const { RefCell::new(None) };
}

pub struct RustFunction {
    name: &'static str,
    func: fn(Args) -> Result<Value>,
}

impl RustFunction {
    pub fn new(name: &'static str, func: fn(Args) -> Result<Value>) -> Self {
        Self { name, func }
    }
}

impl std::fmt::Debug for RustFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RustFunction")
            .field("name", &self.name)
            .finish()
    }
}

impl RustValue for RustFunction {
    fn call(&self, args: Args) -> Result<Value> {
        (self.func)(args)
    }
}

pub fn print(mut args: Args) -> Result<Value> {
    args.set_function_name("print");
    let arg = args.expect("arg")?;
    args.complete()?;

    // Use str() conversion for consistent string representation
    let str_result = str(Args::new(vec![arg], Default::default()))?;
    let output = match str_result {
        Value::String(s) => s,
        _ => unreachable!("str() should always return a string"),
    };

    // If capture is enabled, store the output instead of printing
    if let Ok(mut capture) = PRINT_CAPTURE.lock() {
        if let Some(ref mut captured) = *capture {
            captured.push(output);
            return Ok(Value::Unit);
        }
    }

    // Normal behavior: print to stdout
    println!("{}", output);
    Ok(Value::Unit)
}

pub fn assert(mut args: Args) -> Result<Value> {
    args.set_function_name("assert");
    let condition = args.expect("condition")?;
    let message = args.optional("message", Value::String("Assertion failed".to_string()));
    args.complete()?;

    // Increment assertion count if capture is enabled
    ASSERTION_COUNT.with(|count| {
        if let Some(ref mut counter) = *count.borrow_mut() {
            *counter += 1;
        }
    });

    if !condition.is_truthy() {
        let message_str = match message {
            Value::String(s) => s,
            other => format!("{:?}", other),
        };
        return Err(anyhow!("Assertion failed: {}", message_str));
    }

    Ok(Value::Unit)
}

// Helper functions for test capture
pub fn start_print_capture() {
    if let Ok(mut capture) = PRINT_CAPTURE.lock() {
        *capture = Some(Vec::new());
    }
}

pub fn get_captured_prints() -> Vec<String> {
    if let Ok(mut capture) = PRINT_CAPTURE.lock() {
        if let Some(captured) = capture.take() {
            return captured;
        }
    }
    Vec::new()
}

pub fn stop_print_capture() {
    if let Ok(mut capture) = PRINT_CAPTURE.lock() {
        *capture = None;
    }
}

// Helper functions for assertion counting
pub fn start_assertion_count() {
    ASSERTION_COUNT.with(|count| {
        *count.borrow_mut() = Some(0);
    });
}

pub fn get_assertion_count() -> u32 {
    ASSERTION_COUNT.with(|count| count.borrow_mut().take().unwrap_or(0))
}

pub fn stop_assertion_count() {
    ASSERTION_COUNT.with(|count| {
        *count.borrow_mut() = None;
    });
}

pub fn is_unit(mut args: Args) -> Result<Value> {
    args.set_function_name("is_unit");
    let value = args.expect("value")?;
    args.complete()?;

    let result = matches!(value, Value::Unit);
    Ok(Value::Bool(result))
}

pub fn str(mut args: Args) -> Result<Value> {
    args.set_function_name("str");
    let value = args.expect("value")?;
    args.complete()?;

    let string_repr = match value {
        Value::Null => "null".to_string(),
        Value::Unit => "unit".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::RustValue(rust_val_ref) => {
            let rust_val = rust_val_ref.borrow();
            rust_val.str()
        }
        _ => format!("{:?}", value), // Fallback to debug representation for other types
    };

    Ok(Value::String(string_repr))
}

pub fn raise(mut args: Args) -> Result<Value> {
    args.set_function_name("raise");
    let error_value = args.expect("error")?;
    args.complete()?;

    // Return a special Error value that the evaluator will detect
    Ok(Value::Raised(Box::new(error_value)))
}

pub fn parse_builtin(mut args: Args) -> Result<Value> {
    args.set_function_name("parse");
    let template_result = args.expect("template")?;
    args.complete()?;

    let code_string = match template_result {
        Value::String(s) => s,
        _ => return Err(anyhow!("parse() expects a string")),
    };

    // Use existing lexer/parser/compiler infrastructure
    let tokens = crate::lexer::Lexer::tokenize(&code_string)
        .map_err(|e| anyhow!("Failed to tokenize: {}:\n{}", e, code_string))?;
    let ast = crate::parser::Parser::parse(tokens)
        .map_err(|e| anyhow!("Failed to parse: {}:\n{}", e, code_string))?;
    let eval_ir = crate::eval::Compiler::compile(ast)
        .map_err(|e| anyhow!("Failed to compile: {}:\n{}", e, code_string))?;

    // Return as EvalNode
    Ok(Value::from_rust(crate::core::EvalNode::new(eval_ir)))
}

pub fn macro_identity_builtin(mut args: Args) -> Result<Value> {
    args.set_function_name("macro");
    let eval_node = args.expect("eval_node")?;
    args.complete()?;

    // Identity macro: just return the input node unchanged
    Ok(eval_node)
}

pub fn join_builtin(mut args: Args) -> Result<Value> {
    args.set_function_name("join");
    let list = args.expect("list")?;
    let separator = args.expect("separator")?;
    args.complete()?;

    let sep_str = match separator {
        Value::String(s) => s,
        _ => return Err(anyhow!("join() separator must be a string")),
    };

    match list {
        Value::List(list_ref) => {
            let list_data = list_ref.borrow();
            let string_parts: Vec<String> = list_data
                .iter()
                .map(|item| match item {
                    Value::String(s) => s.clone(),
                    other => format!("{:?}", other), // Convert other types to string representation
                })
                .collect();
            Ok(Value::String(string_parts.join(&sep_str)))
        }
        _ => Err(anyhow!("join() expects a list as first argument")),
    }
}
