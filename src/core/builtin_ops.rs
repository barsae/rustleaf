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

// Comparison operators
pub fn op_eq(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_eq");
    let other = args.expect("other")?;
    args.complete()?;
    
    let result = match (self_value, &other) {
        // Same type comparisons
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Null, Value::Null) => true,
        (Value::Unit, Value::Unit) => true,
        
        // Mixed numeric comparisons
        (Value::Int(a), Value::Float(b)) => *a as f64 == *b,
        (Value::Float(a), Value::Int(b)) => *a == *b as f64,
        
        // Everything else is false
        _ => false,
    };
    
    Ok(Value::Bool(result))
}

pub fn op_ne(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_ne");
    let other = args.expect("other")?;
    args.complete()?;
    
    // Use op_eq and negate the result
    let eq_result = op_eq(self_value, Args::positional(vec![other]))?;
    match eq_result {
        Value::Bool(b) => Ok(Value::Bool(!b)),
        _ => unreachable!("op_eq should always return Bool"),
    }
}

pub fn op_lt(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_lt");
    let other = args.expect("other")?;
    args.complete()?;
    
    let result = match (self_value, &other) {
        // Integer comparisons
        (Value::Int(a), Value::Int(b)) => a < b,
        
        // Float comparisons (includes mixed int/float)
        (Value::Int(a), Value::Float(b)) => (*a as f64) < *b,
        (Value::Float(a), Value::Float(b)) => a < b,
        (Value::Float(a), Value::Int(b)) => *a < (*b as f64),
        
        // String comparisons
        (Value::String(a), Value::String(b)) => a < b,
        
        _ => return Err(anyhow!("Cannot compare {:?} < {:?}", self_value, other)),
    };
    
    Ok(Value::Bool(result))
}

pub fn op_le(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_le");
    let other = args.expect("other")?;
    args.complete()?;
    
    let result = match (self_value, &other) {
        // Integer comparisons
        (Value::Int(a), Value::Int(b)) => a <= b,
        
        // Float comparisons (includes mixed int/float)
        (Value::Int(a), Value::Float(b)) => (*a as f64) <= *b,
        (Value::Float(a), Value::Float(b)) => a <= b,
        (Value::Float(a), Value::Int(b)) => *a <= (*b as f64),
        
        // String comparisons
        (Value::String(a), Value::String(b)) => a <= b,
        
        _ => return Err(anyhow!("Cannot compare {:?} <= {:?}", self_value, other)),
    };
    
    Ok(Value::Bool(result))
}

pub fn op_gt(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_gt");
    let other = args.expect("other")?;
    args.complete()?;
    
    let result = match (self_value, &other) {
        // Integer comparisons
        (Value::Int(a), Value::Int(b)) => a > b,
        
        // Float comparisons (includes mixed int/float)
        (Value::Int(a), Value::Float(b)) => (*a as f64) > *b,
        (Value::Float(a), Value::Float(b)) => a > b,
        (Value::Float(a), Value::Int(b)) => *a > (*b as f64),
        
        // String comparisons
        (Value::String(a), Value::String(b)) => a > b,
        
        _ => return Err(anyhow!("Cannot compare {:?} > {:?}", self_value, other)),
    };
    
    Ok(Value::Bool(result))
}

pub fn op_ge(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_ge");
    let other = args.expect("other")?;
    args.complete()?;
    
    let result = match (self_value, &other) {
        // Integer comparisons
        (Value::Int(a), Value::Int(b)) => a >= b,
        
        // Float comparisons (includes mixed int/float)
        (Value::Int(a), Value::Float(b)) => (*a as f64) >= *b,
        (Value::Float(a), Value::Float(b)) => a >= b,
        (Value::Float(a), Value::Int(b)) => *a >= (*b as f64),
        
        // String comparisons
        (Value::String(a), Value::String(b)) => a >= b,
        
        _ => return Err(anyhow!("Cannot compare {:?} >= {:?}", self_value, other)),
    };
    
    Ok(Value::Bool(result))
}

// Additional arithmetic operators
pub fn op_mod(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_mod");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        // Integer modulo
        (Value::Int(a), Value::Int(b)) => {
            if *b == 0 {
                Err(anyhow!("Modulo by zero"))
            } else {
                Ok(Value::Int(a % b))
            }
        }
        
        // Float modulo (includes mixed int/float operations)
        (Value::Int(a), Value::Float(b)) => {
            if *b == 0.0 {
                Err(anyhow!("Modulo by zero"))
            } else {
                Ok(Value::Float((*a as f64) % b))
            }
        }
        (Value::Float(a), Value::Float(b)) => {
            if *b == 0.0 {
                Err(anyhow!("Modulo by zero"))
            } else {
                Ok(Value::Float(a % b))
            }
        }
        (Value::Float(a), Value::Int(b)) => {
            if *b == 0 {
                Err(anyhow!("Modulo by zero"))
            } else {
                Ok(Value::Float(a % (*b as f64)))
            }
        }
        
        _ => Err(anyhow!("Cannot compute modulo of {:?} and {:?}", self_value, other)),
    }
}

pub fn op_pow(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_pow");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        // Integer power - return int if exponent is non-negative, float otherwise
        (Value::Int(a), Value::Int(b)) => {
            if *b >= 0 {
                match a.checked_pow(*b as u32) {
                    Some(result) => Ok(Value::Int(result)),
                    None => {
                        // Overflow, fall back to float
                        Ok(Value::Float((*a as f64).powf(*b as f64)))
                    }
                }
            } else {
                // Negative exponent always returns float
                Ok(Value::Float((*a as f64).powf(*b as f64)))
            }
        }
        
        // Float power (includes mixed int/float operations)
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(*b as f64))),
        
        _ => Err(anyhow!("Cannot compute power of {:?} and {:?}", self_value, other)),
    }
}

// Bitwise operators (only work on integers)
pub fn op_bitwise_and(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_and");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a & b)),
        _ => Err(anyhow!("Bitwise AND requires integers, got {:?} and {:?}", self_value, other)),
    }
}

pub fn op_bitwise_or(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_or");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a | b)),
        _ => Err(anyhow!("Bitwise OR requires integers, got {:?} and {:?}", self_value, other)),
    }
}

pub fn op_bitwise_xor(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_xor");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a ^ b)),
        _ => Err(anyhow!("Bitwise XOR requires integers, got {:?} and {:?}", self_value, other)),
    }
}

pub fn op_bitwise_not(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_not");
    args.complete()?;
    
    match self_value {
        Value::Int(n) => Ok(Value::Int(!n)),
        _ => Err(anyhow!("Bitwise NOT requires integer, got {:?}", self_value)),
    }
}

pub fn op_lshift(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_lshift");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => {
            if *b < 0 {
                Err(anyhow!("Cannot left shift by negative amount: {}", b))
            } else if *b >= 64 {
                Err(anyhow!("Cannot left shift by more than 63 bits: {}", b))
            } else {
                Ok(Value::Int(a << b))
            }
        }
        _ => Err(anyhow!("Left shift requires integers, got {:?} and {:?}", self_value, other)),
    }
}

pub fn op_rshift(self_value: &Value, args: Args) -> Result<Value> {
    args.set_function_name("op_rshift");
    let other = args.expect("other")?;
    args.complete()?;
    
    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => {
            if *b < 0 {
                Err(anyhow!("Cannot right shift by negative amount: {}", b))
            } else if *b >= 64 {
                Err(anyhow!("Cannot right shift by more than 63 bits: {}", b))
            } else {
                Ok(Value::Int(a >> b))
            }
        }
        _ => Err(anyhow!("Right shift requires integers, got {:?} and {:?}", self_value, other)),
    }
}