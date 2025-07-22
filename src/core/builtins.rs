use super::Value;
use crate::core::Args;

use anyhow::Result;

pub fn print(args: Args) -> Result<Value> {
    args.set_function_name("print");
    let arg = args.expect("arg")?;
    args.complete()?;
    println!("{:?}", arg);
    Ok(Value::Unit)
}