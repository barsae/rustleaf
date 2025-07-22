use super::Value;
use crate::core::{Args, RustValue};

use anyhow::Result;

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
    println!("{:?}", arg);
    Ok(Value::Unit)
}