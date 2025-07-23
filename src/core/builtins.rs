use super::Value;
use crate::core::{Args, RustValue};

use anyhow::{anyhow, Result};
use std::sync::Mutex;

// Global capture for print output during testing
static PRINT_CAPTURE: Mutex<Option<Vec<String>>> = Mutex::new(None);

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

pub fn print(args: Args) -> Result<Value> {
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

pub fn assert(args: Args) -> Result<Value> {
    args.set_function_name("assert");
    let condition = args.expect("condition")?;
    let message = args.optional("message", Value::String("Assertion failed".to_string()));
    args.complete()?;
    
    // Check if condition is truthy
    let is_truthy = match condition {
        Value::Bool(b) => b,
        Value::Unit | Value::Null => false,
        _ => true, // All other values are truthy
    };
    
    if !is_truthy {
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

pub fn is_unit(args: Args) -> Result<Value> {
    args.set_function_name("is_unit");
    let value = args.expect("value")?;
    args.complete()?;
    
    let result = matches!(value, Value::Unit);
    Ok(Value::Bool(result))
}

pub fn str_conversion(args: Args) -> Result<Value> {
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