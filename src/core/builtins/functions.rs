use crate::core::Value;
use crate::core::{Args, RustValue};

use anyhow::{anyhow, Result};

#[derive(Clone)]
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

#[crate::rust_value_any]
impl RustValue for RustFunction {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn call(&self, args: Args) -> Result<Value> {
        (self.func)(args)
    }
}

// Internal helper to write to stdout
fn write_output(output: &str) {
    println!("{output}");
}

pub fn print(mut args: Args) -> Result<Value> {
    args.set_function_name("print");
    let arg = args.expect("arg")?;
    args.complete()?;

    // Use str() conversion for consistent string representation
    let str_result = str(Args::new(vec![arg], Default::default()))?;
    let output = match str_result {
        Value::String(s) => s,
        _ => unreachable!("str() should always return a string"),
    };

    write_output(&output);
    Ok(Value::Unit)
}

pub fn assert(mut args: Args) -> Result<Value> {
    args.set_function_name("assert");
    let condition = args.expect("condition")?;
    let message = args.optional("message", Value::String("Assertion failed".to_string()));
    args.complete()?;

    if !condition.is_truthy() {
        let message_str = match message {
            Value::String(s) => s,
            other => format!("{other:?}"),
        };
        return Err(anyhow!("Assertion failed: {}", message_str));
    }

    Ok(Value::Unit)
}

pub fn is_unit(mut args: Args) -> Result<Value> {
    args.set_function_name("is_unit");
    let value = args.expect("value")?;
    args.complete()?;

    let result = matches!(value, Value::Unit);
    Ok(Value::Bool(result))
}

pub fn str(mut args: Args) -> Result<Value> {
    args.set_function_name("str");
    let value = args.expect("value")?;
    args.complete()?;

    Ok(Value::String(value.str()))
}

pub fn raise(mut args: Args) -> Result<Value> {
    args.set_function_name("raise");
    let error_value = args.expect("error")?;
    args.complete()?;

    // Return a special Error value that the evaluator will detect
    Ok(Value::Raised(Box::new(error_value)))
}

pub fn parse_builtin(mut args: Args) -> Result<Value> {
    args.set_function_name("parse");
    let template_result = args.expect("template")?;
    args.complete()?;

    let code_string = match template_result {
        Value::String(s) => s,
        _ => return Err(anyhow!("parse() expects a string")),
    };

    // Use existing lexer/parser/compiler infrastructure
    let tokens = crate::lexer::Lexer::tokenize(&code_string)
        .map_err(|e| anyhow!("Failed to tokenize: {}:\n{}", e, code_string))?;
    let ast = crate::parser::Parser::parse(tokens)
        .map_err(|e| anyhow!("Failed to parse: {}:\n{}", e, code_string))?;
    let eval_ir = crate::eval::Compiler::compile(ast)
        .map_err(|e| anyhow!("Failed to compile: {}:\n{}", e, code_string))?;

    // Return the Eval directly
    Ok(eval_ir)
}

pub fn macro_identity_builtin(mut args: Args) -> Result<Value> {
    args.set_function_name("macro");
    let eval_node = args.expect("eval_node")?;
    args.complete()?;

    // Identity macro: just return the input node unchanged
    Ok(eval_node)
}

pub fn join_builtin(mut args: Args) -> Result<Value> {
    args.set_function_name("join");
    let list = args.expect("list")?;
    let separator = args.expect("separator")?;
    args.complete()?;

    let sep_str = match separator {
        Value::String(s) => s,
        _ => return Err(anyhow!("join() separator must be a string")),
    };

    match list {
        Value::List(list_ref) => {
            let list_data = list_ref.borrow();
            let string_parts: Vec<String> = list_data
                .iter()
                .map(|item| match item {
                    Value::String(s) => s.clone(),
                    other => format!("{other:?}"), // Convert other types to string representation
                })
                .collect();
            Ok(Value::String(string_parts.join(&sep_str)))
        }
        _ => Err(anyhow!("join() expects a list as first argument")),
    }
}

pub fn int(mut args: Args) -> Result<Value> {
    args.set_function_name("int");
    let value = args.expect("value")?;
    args.complete()?;

    match value {
        Value::Int(i) => Ok(Value::Int(i)),
        Value::Float(f) => Ok(Value::Int(f.trunc() as i64)),
        _ => Err(anyhow!("Cannot convert {:?} to int", value.type_name())),
    }
}

pub fn float(mut args: Args) -> Result<Value> {
    args.set_function_name("float");
    let value = args.expect("value")?;
    args.complete()?;

    match value {
        Value::Float(f) => Ok(Value::Float(f)),
        Value::Int(i) => Ok(Value::Float(i as f64)),
        _ => Err(anyhow!("Cannot convert {:?} to float", value.type_name())),
    }
}

pub fn range(mut args: Args) -> Result<Value> {
    args.set_function_name("range");

    // Handle different argument patterns:
    // range(start, end, step=1) → list
    // range(start) → list (shorthand for range(0, start))

    let first_arg = args.expect("start")?;
    let start_val = first_arg.expect_i64("range", "start")?;

    // Check if we have a second argument (end)
    let (start, end, step) = if let Ok(second_arg) = args.expect("end") {
        // Two or three argument form: range(start, end) or range(start, end, step)
        let end_val = second_arg.expect_i64("range", "end")?;
        let step_val = if let Ok(third_arg) = args.expect("step") {
            third_arg.expect_i64("range", "step")?
        } else {
            1i64
        };
        args.complete()?;
        (start_val, end_val, step_val)
    } else {
        // Single argument form: range(start) is shorthand for range(0, start)
        args.complete()?;
        (0i64, start_val, 1i64)
    };

    // Validate step != 0
    if step == 0 {
        return Err(anyhow!("range() step argument must not be zero"));
    }

    // Generate the range
    let mut values = Vec::new();

    if step > 0 {
        // Positive step: start < end
        let mut current = start;
        while current < end {
            values.push(Value::Int(current));
            current += step;
        }
    } else {
        // Negative step: start > end
        let mut current = start;
        while current > end {
            values.push(Value::Int(current));
            current += step;
        }
    }

    Ok(Value::new_list_with_values(values))
}

pub fn sum(mut args: Args) -> Result<Value> {
    args.set_function_name("sum");
    let iterable = args.expect("iterable")?;
    args.complete()?;

    match iterable {
        Value::List(list_ref) => {
            let list = list_ref.borrow();

            if list.is_empty() {
                return Ok(Value::Int(0));
            }

            let mut sum = list[0].clone();

            for item in list.iter().skip(1) {
                // Use the helper method for cleaner code
                sum = sum.op_add(item.clone())?;
            }

            Ok(sum)
        }
        _ => Err(anyhow!("sum() expects a list or iterable")),
    }
}

pub fn filter(mut args: Args) -> Result<Value> {
    args.set_function_name("filter");
    let mut iterable = args.expect("iterable")?;
    let predicate = args.expect("predicate")?;
    args.complete()?;

    match &iterable {
        Value::List(list_ref) => {
            // Handle lists directly for efficiency
            let list = list_ref.borrow();
            let mut filtered = Vec::new();

            for item in list.iter() {
                // Call the predicate function with the item
                let predicate_args = crate::core::Args::positional(vec![item.clone()]);
                let result = predicate.call(predicate_args)?;

                // Check if result is truthy
                if result.is_truthy() {
                    filtered.push(item.clone());
                }
            }

            Ok(Value::new_list_with_values(filtered))
        }
        _ => {
            // Handle any iterator using the op_next helper
            let mut filtered = Vec::new();

            loop {
                // Try to get the next value from the iterator
                match iterable.op_next() {
                    Ok(Some(item)) => {
                        // Call the predicate function with the item
                        let predicate_args = crate::core::Args::positional(vec![item.clone()]);
                        let result = predicate.call(predicate_args)?;

                        // Check if result is truthy
                        if result.is_truthy() {
                            filtered.push(item);
                        }
                    }
                    Ok(None) => break, // Iterator exhausted
                    Err(_) => {
                        // Not an iterator, return error
                        return Err(anyhow!("filter() expects a list or iterable"));
                    }
                }
            }

            Ok(Value::new_list_with_values(filtered))
        }
    }
}

pub fn take_while(mut args: Args) -> Result<Value> {
    args.set_function_name("take_while");
    let mut iterable = args.expect("iterable")?;
    let predicate = args.expect("predicate")?;
    args.complete()?;

    match &iterable {
        Value::List(list_ref) => {
            // Handle lists directly for efficiency
            let list = list_ref.borrow();
            let mut taken = Vec::new();

            for item in list.iter() {
                // Call the predicate function with the item
                let predicate_args = crate::core::Args::positional(vec![item.clone()]);
                let result = predicate.call(predicate_args)?;

                // Check if result is truthy
                if result.is_truthy() {
                    taken.push(item.clone());
                } else {
                    break; // Stop taking once predicate is false
                }
            }

            Ok(Value::new_list_with_values(taken))
        }
        _ => {
            // Handle any iterator using the op_next helper
            let mut taken = Vec::new();

            loop {
                // Try to get the next value from the iterator
                match iterable.op_next() {
                    Ok(Some(item)) => {
                        // Call the predicate function with the item
                        let predicate_args = crate::core::Args::positional(vec![item.clone()]);
                        let result = predicate.call(predicate_args)?;

                        // Check if result is truthy
                        if result.is_truthy() {
                            taken.push(item);
                        } else {
                            break; // Stop taking once predicate is false
                        }
                    }
                    Ok(None) => break, // Iterator exhausted
                    Err(_) => {
                        // Not an iterator, return error
                        return Err(anyhow!("take_while() expects a list or iterable"));
                    }
                }
            }

            Ok(Value::new_list_with_values(taken))
        }
    }
}
