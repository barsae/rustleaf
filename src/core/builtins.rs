use super::Value;
use crate::core::{Args, RustValue};

use anyhow::Result;
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
    fn get_attr(&self, _name: &str) -> Option<Value> { None }
    fn set_attr(&mut self, _name: &str, _value: Value) -> Result<(), String> {
        Err("Cannot set attributes on built-in function".to_string())
    }
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