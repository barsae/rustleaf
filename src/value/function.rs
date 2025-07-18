#![allow(clippy::result_large_err)]

use crate::eval::Environment;
use crate::value::types::{ErrorType, RuntimeError, Value};

pub type BuiltinFunction = fn(&[Value], &mut Environment) -> Result<Value, RuntimeError>;

#[derive(Debug, Clone)]
pub struct BuiltinFunctionInfo {
    pub name: &'static str,
    pub function: BuiltinFunction,
    pub arity: Option<usize>, // None for variadic
}

impl PartialEq for BuiltinFunctionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arity == other.arity
    }
}

pub fn get_builtin_functions() -> Vec<BuiltinFunctionInfo> {
    vec![
        BuiltinFunctionInfo {
            name: "print",
            function: builtin_print,
            arity: None, // Variadic
        },
        BuiltinFunctionInfo {
            name: "len",
            function: builtin_len,
            arity: Some(1),
        },
        BuiltinFunctionInfo {
            name: "type",
            function: builtin_type,
            arity: Some(1),
        },
        BuiltinFunctionInfo {
            name: "assert",
            function: builtin_assert,
            arity: None, // 1-2 arguments
        },
        BuiltinFunctionInfo {
            name: "assert_approx_eq",
            function: builtin_assert_approx_eq,
            arity: None, // 2-3 arguments
        },
        BuiltinFunctionInfo {
            name: "raise",
            function: builtin_raise,
            arity: Some(1),
        },
        BuiltinFunctionInfo {
            name: "is_unit",
            function: builtin_is_unit,
            arity: Some(1),
        },
    ]
}

fn builtin_print(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    let output = if args.is_empty() {
        String::new()
    } else {
        args.iter()
            .map(|v| v.to_display_string())
            .collect::<Vec<_>>()
            .join(" ")
    };

    println!("{}", output);
    Ok(Value::Unit)
}

fn builtin_len(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(
            format!("len() takes exactly 1 argument ({} given)", args.len()),
            ErrorType::TypeError,
        ));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::Int(s.len() as i64)),
        Value::List(list) => Ok(Value::Int(list.len() as i64)),
        Value::Dict(dict) => Ok(Value::Int(dict.len() as i64)),
        _ => Err(RuntimeError::new(
            format!("object of type '{}' has no len()", args[0].type_name()),
            ErrorType::TypeError,
        )),
    }
}

fn builtin_type(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(
            format!("type() takes exactly 1 argument ({} given)", args.len()),
            ErrorType::TypeError,
        ));
    }

    Ok(Value::String(args[0].type_name().to_string()))
}

fn builtin_assert(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    if args.is_empty() || args.len() > 2 {
        return Err(RuntimeError::new(
            format!("assert() takes 1 or 2 arguments ({} given)", args.len()),
            ErrorType::TypeError,
        ));
    }

    let condition = &args[0];
    let is_truthy = condition.is_truthy()?;

    if !is_truthy {
        let message = if args.len() == 2 {
            match &args[1] {
                Value::String(s) => s.clone(),
                _ => args[1].to_display_string(),
            }
        } else {
            "Assertion failed".to_string()
        };

        return Err(RuntimeError::new(message, ErrorType::AssertionError));
    }

    Ok(Value::Unit)
}

fn builtin_assert_approx_eq(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(RuntimeError::new(
            format!(
                "assert_approx_eq() takes 2 or 3 arguments ({} given)",
                args.len()
            ),
            ErrorType::TypeError,
        ));
    }

    let a = &args[0];
    let b = &args[1];

    // Check if both values are numeric (int or float)
    let (a_num, b_num) = match (a, b) {
        (Value::Float(f1), Value::Float(f2)) => (*f1, *f2),
        (Value::Int(i1), Value::Float(f2)) => (*i1 as f64, *f2),
        (Value::Float(f1), Value::Int(i2)) => (*f1, *i2 as f64),
        (Value::Int(i1), Value::Int(i2)) => (*i1 as f64, *i2 as f64),
        _ => {
            return Err(RuntimeError::new(
                "assert_approx_eq() requires numeric values".to_string(),
                ErrorType::TypeError,
            ));
        }
    };

    // Default epsilon is 1e-9, or use third argument if provided
    let epsilon = if args.len() == 3 {
        match &args[2] {
            Value::Float(eps) => *eps,
            Value::Int(eps) => *eps as f64,
            _ => {
                return Err(RuntimeError::new(
                    "assert_approx_eq() epsilon must be numeric".to_string(),
                    ErrorType::TypeError,
                ));
            }
        }
    } else {
        1e-9
    };

    let diff = (a_num - b_num).abs();
    if diff > epsilon {
        let message = format!(
            "Assertion failed: {} is not approximately equal to {} (diff: {}, epsilon: {})",
            a.to_display_string(),
            b.to_display_string(),
            diff,
            epsilon
        );
        return Err(RuntimeError::new(message, ErrorType::AssertionError));
    }

    Ok(Value::Unit)
}

fn builtin_raise(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(
            format!("raise() takes exactly 1 argument ({} given)", args.len()),
            ErrorType::TypeError,
        ));
    }

    // Create a RuntimeError from the raised value
    let error_value = &args[0];
    let message = error_value.to_display_string();

    // Create a custom error that carries the raised value
    let mut error = RuntimeError::new(message, ErrorType::RaisedError);
    error.return_value = Some(error_value.clone());

    Err(error)
}

fn builtin_is_unit(args: &[Value], _env: &mut Environment) -> Result<Value, RuntimeError> {
    if args.len() != 1 {
        return Err(RuntimeError::new(
            format!("is_unit() takes exactly 1 argument ({} given)", args.len()),
            ErrorType::TypeError,
        ));
    }

    let is_unit = matches!(args[0], Value::Unit);
    Ok(Value::Bool(is_unit))
}
