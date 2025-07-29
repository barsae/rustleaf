use crate::core::{Args, Value};
use crate::eval::Evaluator;
use anyhow::{anyhow, Result};

fn value_to_f64(value: &Value) -> Result<f64> {
    match value {
        Value::Int(n) => Ok(*n as f64),
        Value::Float(f) => Ok(*f),
        _ => Err(anyhow!("Expected a number, got {:?}", value)),
    }
}

pub fn sqrt(mut args: Args) -> Result<Value> {
    args.set_function_name("sqrt");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    if f < 0.0 {
        return Err(anyhow!("sqrt() of negative number"));
    }
    Ok(Value::Float(f.sqrt()))
}

pub fn sin(mut args: Args) -> Result<Value> {
    args.set_function_name("sin");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    Ok(Value::Float(f.sin()))
}

pub fn cos(mut args: Args) -> Result<Value> {
    args.set_function_name("cos");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    Ok(Value::Float(f.cos()))
}

pub fn tan(mut args: Args) -> Result<Value> {
    args.set_function_name("tan");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    Ok(Value::Float(f.tan()))
}

pub fn abs(mut args: Args) -> Result<Value> {
    args.set_function_name("abs");
    let arg = args.expect("value")?;
    args.complete()?;

    match arg {
        Value::Int(n) => Ok(Value::Int(n.abs())),
        Value::Float(f) => Ok(Value::Float(f.abs())),
        _ => Err(anyhow!("abs() requires a number, got {:?}", arg)),
    }
}

pub fn floor(mut args: Args) -> Result<Value> {
    args.set_function_name("floor");
    let arg = args.expect("value")?;
    args.complete()?;

    match arg {
        Value::Int(n) => Ok(Value::Int(n)),
        Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
        _ => Err(anyhow!("floor() requires a number, got {:?}", arg)),
    }
}

pub fn ceil(mut args: Args) -> Result<Value> {
    args.set_function_name("ceil");
    let arg = args.expect("value")?;
    args.complete()?;

    match arg {
        Value::Int(n) => Ok(Value::Int(n)),
        Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
        _ => Err(anyhow!("ceil() requires a number, got {:?}", arg)),
    }
}

pub fn round(mut args: Args) -> Result<Value> {
    args.set_function_name("round");
    let arg = args.expect("value")?;
    args.complete()?;

    match arg {
        Value::Int(n) => Ok(Value::Int(n)),
        Value::Float(f) => Ok(Value::Int(f.round() as i64)),
        _ => Err(anyhow!("round() requires a number, got {:?}", arg)),
    }
}

pub fn min(mut args: Args) -> Result<Value> {
    args.set_function_name("min");
    let first = args.expect("first")?;
    let second = args.expect("second")?;
    args.complete()?;

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.min(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).min(b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.min(b as f64))),
        _ => Err(anyhow!("min() requires two numbers")),
    }
}

pub fn max(mut args: Args) -> Result<Value> {
    args.set_function_name("max");
    let first = args.expect("first")?;
    let second = args.expect("second")?;
    args.complete()?;

    match (first, second) {
        (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.max(b))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(b))),
        (Value::Int(a), Value::Float(b)) => Ok(Value::Float((a as f64).max(b))),
        (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.max(b as f64))),
        _ => Err(anyhow!("max() requires two numbers")),
    }
}

pub fn log(mut args: Args) -> Result<Value> {
    args.set_function_name("log");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    if f <= 0.0 {
        return Err(anyhow!("log() of non-positive number"));
    }
    Ok(Value::Float(f.ln()))
}

pub fn log10(mut args: Args) -> Result<Value> {
    args.set_function_name("log10");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    if f <= 0.0 {
        return Err(anyhow!("log10() of non-positive number"));
    }
    Ok(Value::Float(f.log10()))
}

pub fn exp(mut args: Args) -> Result<Value> {
    args.set_function_name("exp");
    let arg = args.expect("value")?;
    args.complete()?;

    let f = value_to_f64(&arg)?;
    Ok(Value::Float(f.exp()))
}

pub fn register_math(evaluator: &mut Evaluator) {
    evaluator.register_builtin_fn("sqrt", sqrt);
    evaluator.register_builtin_fn("sin", sin);
    evaluator.register_builtin_fn("cos", cos);
    evaluator.register_builtin_fn("tan", tan);
    evaluator.register_builtin_fn("abs", abs);
    evaluator.register_builtin_fn("floor", floor);
    evaluator.register_builtin_fn("ceil", ceil);
    evaluator.register_builtin_fn("round", round);
    evaluator.register_builtin_fn("min", min);
    evaluator.register_builtin_fn("max", max);
    evaluator.register_builtin_fn("log", log);
    evaluator.register_builtin_fn("log10", log10);
    evaluator.register_builtin_fn("exp", exp);
}
