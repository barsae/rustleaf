use crate::value::types::{Value, RuntimeError, ErrorType};
use crate::eval::Environment;

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
    Ok(Value::Null)
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

        return Err(RuntimeError::new(
            message,
            ErrorType::AssertionError,
        ));
    }

    Ok(Value::Null)
}