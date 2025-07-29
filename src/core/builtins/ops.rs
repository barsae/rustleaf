use crate::core::Args;
use crate::core::{RustValue, Value};
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
            method_func,
        }
    }
}

impl RustValue for BoundMethod {
    crate::impl_rust_value_any!(BoundMethod);

    fn call(&self, args: Args) -> Result<Value> {
        (self.method_func)(&self.self_value, args)
    }
}

// General operator implementations that work for all value types
pub fn op_add(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_add");

    // For strings, support multiple arguments for efficient concatenation
    if let Value::String(first_str) = self_value {
        let mut result = first_str.clone();

        // Concatenate all remaining arguments
        let remaining_args = args.rest();
        for arg in remaining_args {
            match arg {
                Value::String(s) => result.push_str(&s),
                _ => {
                    return Err(anyhow!(
                        "String concatenation requires all arguments to be strings, got {:?}",
                        arg
                    ))
                }
            }
        }

        args.complete()?;
        return Ok(Value::String(result));
    }

    // For non-strings, fall back to binary operation
    let other = args.expect("other")?;
    args.complete()?;

    match (self_value, &other) {
        // Integer arithmetic
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),

        // Float arithmetic (includes mixed int/float operations)
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),

        _ => Err(anyhow!("Cannot add {:?} to {:?}", other, self_value)),
    }
}

pub fn op_sub(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_mul(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_div(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_neg(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_neg");
    args.complete()?;

    match self_value {
        Value::Int(n) => Ok(Value::Int(-n)),
        Value::Float(f) => Ok(Value::Float(-f)),
        _ => Err(anyhow!("Cannot negate {:?}", self_value)),
    }
}

// Comparison operators
pub fn op_eq(self_value: &Value, mut args: Args) -> Result<Value> {
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

        // List comparisons
        (Value::List(a), Value::List(b)) => {
            let list_a = a.borrow();
            let list_b = b.borrow();

            // Check length first
            if list_a.len() != list_b.len() {
                return Ok(Value::Bool(false));
            }

            // Compare elements recursively
            for (elem_a, elem_b) in list_a.iter().zip(list_b.iter()) {
                // Use op_eq recursively for each element
                let eq_result = op_eq(elem_a, Args::positional(vec![elem_b.clone()]))?;
                match eq_result {
                    Value::Bool(false) => return Ok(Value::Bool(false)),
                    Value::Bool(true) => continue,
                    _ => unreachable!("op_eq should always return Bool"),
                }
            }

            true
        }

        // Dictionary comparisons
        (Value::Dict(a), Value::Dict(b)) => {
            let dict_a = a.borrow();
            let dict_b = b.borrow();

            // Check if they have the same number of keys
            if dict_a.len() != dict_b.len() {
                return Ok(Value::Bool(false));
            }

            // Compare each key-value pair
            for (key, value_a) in dict_a.iter() {
                match dict_b.get(key) {
                    Some(value_b) => {
                        // Use op_eq recursively for each value
                        let eq_result = op_eq(value_a, Args::positional(vec![value_b.clone()]))?;
                        match eq_result {
                            Value::Bool(false) => return Ok(Value::Bool(false)),
                            Value::Bool(true) => continue,
                            _ => unreachable!("op_eq should always return Bool"),
                        }
                    }
                    None => return Ok(Value::Bool(false)), // Key not found in dict_b
                }
            }

            true
        }

        // Everything else is false
        _ => false,
    };

    Ok(Value::Bool(result))
}

pub fn op_ne(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_lt(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_le(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_gt(self_value: &Value, mut args: Args) -> Result<Value> {
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

pub fn op_ge(self_value: &Value, mut args: Args) -> Result<Value> {
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
pub fn op_mod(self_value: &Value, mut args: Args) -> Result<Value> {
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

        _ => Err(anyhow!(
            "Cannot compute modulo of {:?} and {:?}",
            self_value,
            other
        )),
    }
}

pub fn op_pow(self_value: &Value, mut args: Args) -> Result<Value> {
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

        _ => Err(anyhow!(
            "Cannot compute power of {:?} and {:?}",
            self_value,
            other
        )),
    }
}

// Bitwise operators (only work on integers)
pub fn op_bitwise_and(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_and");
    let other = args.expect("other")?;
    args.complete()?;

    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a & b)),
        _ => Err(anyhow!(
            "Bitwise AND requires integers, got {:?} and {:?}",
            self_value,
            other
        )),
    }
}

pub fn op_bitwise_or(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_or");
    let other = args.expect("other")?;
    args.complete()?;

    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a | b)),
        _ => Err(anyhow!(
            "Bitwise OR requires integers, got {:?} and {:?}",
            self_value,
            other
        )),
    }
}

pub fn op_bitwise_xor(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_xor");
    let other = args.expect("other")?;
    args.complete()?;

    match (self_value, &other) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a ^ b)),
        _ => Err(anyhow!(
            "Bitwise XOR requires integers, got {:?} and {:?}",
            self_value,
            other
        )),
    }
}

pub fn op_bitwise_not(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_bitwise_not");
    args.complete()?;

    match self_value {
        Value::Int(n) => Ok(Value::Int(!n)),
        _ => Err(anyhow!(
            "Bitwise NOT requires integer, got {:?}",
            self_value
        )),
    }
}

pub fn op_lshift(self_value: &Value, mut args: Args) -> Result<Value> {
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
        _ => Err(anyhow!(
            "Left shift requires integers, got {:?} and {:?}",
            self_value,
            other
        )),
    }
}

pub fn op_rshift(self_value: &Value, mut args: Args) -> Result<Value> {
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
        _ => Err(anyhow!(
            "Right shift requires integers, got {:?} and {:?}",
            self_value,
            other
        )),
    }
}

// Container operations
pub fn op_contains(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_contains");
    let item = args.expect("item")?;
    args.complete()?;

    match self_value {
        Value::String(s) => {
            // Check if item is a substring
            match &item {
                Value::String(needle) => Ok(Value::Bool(s.contains(needle))),
                _ => Err(anyhow!(
                    "String contains requires string argument, got {:?}",
                    item
                )),
            }
        }
        Value::List(list_ref) => {
            // Check if item is in the list
            let list = list_ref.borrow();
            for list_item in list.iter() {
                if *list_item == item {
                    return Ok(Value::Bool(true));
                }
            }
            Ok(Value::Bool(false))
        }
        Value::Dict(dict_ref) => {
            // Check if key exists in dictionary
            let dict = dict_ref.borrow();
            let key_str = match &item {
                Value::String(s) => s.clone(),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => {
                    return Err(anyhow!(
                        "Dictionary contains requires string, number, or boolean key, got {:?}",
                        item
                    ))
                }
            };
            Ok(Value::Bool(dict.contains_key(&key_str)))
        }
        Value::Range(range) => {
            // Check if integer is in range
            match &item {
                Value::Int(i) => {
                    let in_range = if range.inclusive {
                        *i >= range.start && *i <= range.end
                    } else {
                        *i >= range.start && *i < range.end
                    };
                    Ok(Value::Bool(in_range))
                }
                _ => Err(anyhow!(
                    "Range contains requires integer argument, got {:?}",
                    item
                )),
            }
        }
        _ => Err(anyhow!(
            "Contains operation not supported for {:?}",
            self_value
        )),
    }
}

// List indexing operations
pub fn op_get_item_list(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_get_item");
    let index = args.expect("index")?;
    args.complete()?;

    match self_value {
        Value::List(list_ref) => {
            let list = list_ref.borrow();
            match &index {
                Value::Int(i) => {
                    let idx = if *i < 0 {
                        // Negative indexing: -1 is last element
                        (list.len() as i64 + i) as usize
                    } else {
                        *i as usize
                    };

                    if idx < list.len() {
                        Ok(list[idx].clone())
                    } else {
                        Err(anyhow!(
                            "List index {} out of range (length: {})",
                            i,
                            list.len()
                        ))
                    }
                }
                _ => Err(anyhow!("List index must be integer, got {:?}", index)),
            }
        }
        _ => Err(anyhow!(
            "op_get_item_list called on non-list: {:?}",
            self_value
        )),
    }
}

pub fn op_set_item_list(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_set_item");
    let index = args.expect("index")?;
    let value = args.expect("value")?;
    args.complete()?;

    match self_value {
        Value::List(list_ref) => {
            let mut list = list_ref.borrow_mut();
            match &index {
                Value::Int(i) => {
                    let idx = if *i < 0 {
                        // Negative indexing: -1 is last element
                        (list.len() as i64 + i) as usize
                    } else {
                        *i as usize
                    };

                    if idx < list.len() {
                        list[idx] = value;
                        Ok(Value::Unit)
                    } else {
                        Err(anyhow!(
                            "List index {} out of range (length: {})",
                            i,
                            list.len()
                        ))
                    }
                }
                _ => Err(anyhow!("List index must be integer, got {:?}", index)),
            }
        }
        _ => Err(anyhow!(
            "op_set_item_list called on non-list: {:?}",
            self_value
        )),
    }
}

// Dictionary indexing operations
pub fn op_get_item_dict(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_get_item");
    let key = args.expect("key")?;
    args.complete()?;

    match self_value {
        Value::Dict(dict_ref) => {
            let dict = dict_ref.borrow();
            let key_str = match &key {
                Value::String(s) => s.clone(),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => {
                    return Err(anyhow!(
                        "Dictionary key must be string, number, or boolean, got {:?}",
                        key
                    ))
                }
            };

            match dict.get(&key_str) {
                Some(value) => Ok(value.clone()),
                None => Err(anyhow!("Key '{}' not found in dictionary", key_str)),
            }
        }
        _ => Err(anyhow!(
            "op_get_item_dict called on non-dict: {:?}",
            self_value
        )),
    }
}

pub fn op_set_item_dict(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("op_set_item");
    let key = args.expect("key")?;
    let value = args.expect("value")?;
    args.complete()?;

    match self_value {
        Value::Dict(dict_ref) => {
            let mut dict = dict_ref.borrow_mut();
            let key_str = match &key {
                Value::String(s) => s.clone(),
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => {
                    return Err(anyhow!(
                        "Dictionary key must be string, number, or boolean, got {:?}",
                        key
                    ))
                }
            };

            dict.insert(key_str, value);
            Ok(Value::Unit)
        }
        _ => Err(anyhow!(
            "op_set_item_dict called on non-dict: {:?}",
            self_value
        )),
    }
}

// Range-specific operations
pub fn range_to_list(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("to_list");
    args.complete()?;

    match self_value {
        Value::Range(range) => {
            let mut values = Vec::new();
            let end_value = if range.inclusive {
                range.end + 1
            } else {
                range.end
            };

            for i in range.start..end_value {
                values.push(Value::Int(i));
            }

            Ok(Value::new_list_with_values(values))
        }
        _ => Err(anyhow!("to_list called on non-range: {:?}", self_value)),
    }
}
