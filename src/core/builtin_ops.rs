use super::{Value, RustValue};
use crate::core::Args;
use anyhow::{anyhow, Result};

// Helper struct that captures the self value for bound methods
#[derive(Debug)]
pub struct BoundMethod {
    self_value: Value,
    method_func: fn(&Value, Args) -> Result<Value>,
}

impl BoundMethod {
    pub fn new(self_value: &Value, method_func: fn(&Value, Args) -> Result<Value>) -> Self {
        Self { 
            self_value: self_value.clone(),
            method_func 
        }
    }
}

impl RustValue for BoundMethod {
    fn call(&self, args: Args) -> Result<Value> {
        (self.method_func)(&self.self_value, args)
    }
}

// General operator implementations that work for all value types
pub fn op_add(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_add");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        // Integer arithmetic
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
        
        // Float arithmetic (includes mixed int/float operations)
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
        
        // String concatenation
        (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
        
        _ => Err(anyhow!("Cannot add {:?} to {:?}", other, self_value)),
    }
}

pub fn op_sub(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_sub");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        // Integer arithmetic
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
        
        // Float arithmetic (includes mixed int/float operations)
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
        
        _ => Err(anyhow!("Cannot subtract {:?} from {:?}", other, self_value)),
    }
}

pub fn op_mul(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_mul");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        // Integer arithmetic
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
        
        // Float arithmetic (includes mixed int/float operations)
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
        
        _ => Err(anyhow!("Cannot multiply {:?} by {:?}", self_value, other)),
    }
}

pub fn op_div(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_div");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        // All division returns float (including int/int)
        (Value::Int(a), Value::Int(b)) => {
            if *b == 0 {
                Err(anyhow!("Division by zero"))
            } else {
                Ok(Value::Float(*a as f64 / *b as f64))
            }
        }
        (Value::Int(a), Value::Float(b)) => {
            if *b == 0.0 {
                Err(anyhow!("Division by zero"))
            } else {
                Ok(Value::Float(*a as f64 / b))
            }
        }
        (Value::Float(a), Value::Float(b)) => {
            if *b == 0.0 {
                Err(anyhow!("Division by zero"))
            } else {
                Ok(Value::Float(a / b))
            }
        }
        (Value::Float(a), Value::Int(b)) => {
            if *b == 0 {
                Err(anyhow!("Division by zero"))
            } else {
                Ok(Value::Float(a / *b as f64))
            }
        }
        
        _ => Err(anyhow!("Cannot divide {:?} by {:?}", self_value, other)),
    }
}

pub fn op_neg(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_neg");
    args.complete()?;
    
    match self_value {
        Value::Int(n) => Ok(Value::Int(-n)),
        Value::Float(f) => Ok(Value::Float(-f)),
        _ => Err(anyhow!("Cannot negate {:?}", self_value)),
    }
}