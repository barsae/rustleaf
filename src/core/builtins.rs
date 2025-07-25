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

    let output = format!("{:?}", arg);

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

pub fn str_conversion(mut args: Args) -> Result<Value> {
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
        _ => format!("{:?}", value), // Fallback to debug representation
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
