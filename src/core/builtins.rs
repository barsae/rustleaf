/// Built-in functions for RustLeaf
use anyhow::Result;
use super::Value;

/// Print values to stdout
pub fn print(args: Vec<Value>) -> Result<Value> {
    let strings: Vec<String> = args.iter()
        .map(|v| super::to_string(v))
        .collect();
    
    println!("{}", strings.join(" "));
    Ok(Value::Unit)
}

/// Get the type of a value
pub fn type_of_builtin(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            "type() takes exactly 1 argument ({} given)", args.len()
        ));
    }
    
    let type_name = super::type_of(&args[0]);
    Ok(Value::String(type_name.to_string()))
}

/// Get the length of a collection or string
pub fn len(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            "len() takes exactly 1 argument ({} given)", args.len()
        ));
    }
    
    match &args[0] {
        Value::String(s) => Ok(Value::Int(s.chars().count() as i64)),
        Value::List(list) => Ok(Value::Int(list.borrow().len() as i64)),
        Value::Dict(dict) => Ok(Value::Int(dict.borrow().len() as i64)),
        _ => Err(anyhow::anyhow!(
            "object of type '{}' has no len()", super::type_of(&args[0])
        )),
    }
}

/// Convert value to integer
pub fn int(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            format!("int() takes exactly 1 argument ({} given)", args.len())
        ));
    }
    
    match &args[0] {
        Value::Int(n) => Ok(Value::Int(*n)),
        Value::Float(f) => Ok(Value::Int(*f as i64)),
        Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
        Value::String(s) => {
            s.parse::<i64>()
                .map(Value::Int)
                .map_err(|_| anyhow::anyhow!(
                    format!("invalid literal for int(): '{}'", s)
                ))
        }
        _ => Err(anyhow::anyhow!(
            "int() argument must be a number or string, not '{}'", 
            super::type_of(&args[0])
        )),
    }
}

/// Convert value to float
pub fn float(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            format!("float() takes exactly 1 argument ({} given)", args.len())
        ));
    }
    
    match &args[0] {
        Value::Float(f) => Ok(Value::Float(*f)),
        Value::Int(n) => Ok(Value::Float(*n as f64)),
        Value::String(s) => {
            s.parse::<f64>()
                .map(Value::Float)
                .map_err(|_| anyhow::anyhow!(
                    format!("invalid literal for float(): '{}'", s)
                ))
        }
        _ => Err(anyhow::anyhow!(
            "float() argument must be a number or string, not '{}'", 
            super::type_of(&args[0])
        )),
    }
}

/// Convert value to string
pub fn str(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            format!("str() takes exactly 1 argument ({} given)", args.len())
        ));
    }
    
    Ok(Value::String(super::to_string(&args[0])))
}

/// Check if value is unit
pub fn is_unit(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            format!("is_unit() takes exactly 1 argument ({} given)", args.len())
        ));
    }
    
    Ok(Value::Bool(matches!(args[0], Value::Unit)))
}

/// Create a range
pub fn range(args: Vec<Value>) -> Result<Value> {
    // TODO: Implement range creation
    // This will return a range iterator
    Err(anyhow::anyhow!(
        "range() not yet implemented".to_string()
    ))
}

/// Assert a condition
pub fn assert(args: Vec<Value>) -> Result<Value> {
    if args.is_empty() || args.len() > 2 {
        return Err(anyhow::anyhow!(
            format!("assert() takes 1 or 2 arguments ({} given)", args.len())
        ));
    }
    
    let condition = &args[0];
    let message = args.get(1);
    
    // Check truthiness
    let is_true = match condition {
        Value::Bool(b) => *b,
        Value::Null => false,
        _ => return Err(anyhow::anyhow!(
            "Only null and bool have truthiness"
        )),
    };
    
    if !is_true {
        let msg = if let Some(Value::String(s)) = message {
            s.clone()
        } else {
            "Assertion failed".to_string()
        };
        Err(anyhow::anyhow!(msg))
    } else {
        Ok(Value::Unit)
    }
}

/// Raise an error
pub fn raise(args: Vec<Value>) -> Result<Value> {
    if args.len() != 1 {
        return Err(anyhow::anyhow!(
            format!("raise() takes exactly 1 argument ({} given)", args.len())
        ));
    }
    
    match &args[0] {
        Value::String(msg) => Err(anyhow::anyhow!(msg.clone())),
        _ => Err(anyhow::anyhow!(
            "raise() argument must be a string".to_string()
        )),
    }
}