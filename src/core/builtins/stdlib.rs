use crate::core::{Args, Value};
use anyhow::{anyhow, Result};

// String methods
pub fn string_len(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("len");
    args.complete()?;

    if let Value::String(s) = self_value {
        Ok(Value::Int(s.chars().count() as i64))
    } else {
        Err(anyhow!("len() called on non-string value"))
    }
}

pub fn string_to_list(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("to_list");
    args.complete()?;

    if let Value::String(s) = self_value {
        let chars: Vec<Value> = s.chars().map(|c| Value::String(c.to_string())).collect();
        Ok(Value::new_list_with_values(chars))
    } else {
        Err(anyhow!("to_list() called on non-string value"))
    }
}

// List methods
pub fn list_len(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("len");
    args.complete()?;

    if let Value::List(list_ref) = self_value {
        let list = list_ref.borrow();
        Ok(Value::Int(list.len() as i64))
    } else {
        Err(anyhow!("len() called on non-list value"))
    }
}

pub fn list_append(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("append");
    let item = args.expect("item")?;
    args.complete()?;

    if let Value::List(list_ref) = self_value {
        list_ref.borrow_mut().push(item);
        Ok(self_value.clone()) // Return self for chaining
    } else {
        Err(anyhow!("append() called on non-list value"))
    }
}

pub fn list_filter(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("filter");
    let predicate = args.expect("predicate")?;
    args.complete()?;

    if let Value::List(list_ref) = self_value {
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
    } else {
        Err(anyhow!("filter() called on non-list value"))
    }
}

pub fn list_map(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("map");
    let mapper = args.expect("mapper")?;
    args.complete()?;

    if let Value::List(list_ref) = self_value {
        let list = list_ref.borrow();
        let mut mapped = Vec::new();

        for item in list.iter() {
            // Call the mapper function with the item
            let mapper_args = crate::core::Args::positional(vec![item.clone()]);
            let result = mapper.call(mapper_args)?;
            mapped.push(result);
        }

        Ok(Value::new_list_with_values(mapped))
    } else {
        Err(anyhow!("map() called on non-list value"))
    }
}

pub fn list_reduce(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("reduce");
    let reducer = args.expect("reducer")?;
    let initial = args.optional("initial", Value::Null);
    args.complete()?;

    if let Value::List(list_ref) = self_value {
        let list = list_ref.borrow();

        if list.is_empty() {
            if matches!(initial, Value::Null) {
                return Err(anyhow!("reduce() of empty sequence with no initial value"));
            }
            return Ok(initial);
        }

        let is_initial_null = matches!(initial, Value::Null);

        let mut accumulator = if is_initial_null {
            list[0].clone()
        } else {
            initial
        };

        let start_index = if is_initial_null { 1 } else { 0 };

        for item in list.iter().skip(start_index) {
            // Call the reducer function with accumulator and current item
            let reducer_args = crate::core::Args::positional(vec![accumulator, item.clone()]);
            accumulator = reducer.call(reducer_args)?;
        }

        Ok(accumulator)
    } else {
        Err(anyhow!("reduce() called on non-list value"))
    }
}

pub fn list_sum(self_value: &Value, mut args: Args) -> Result<Value> {
    args.set_function_name("sum");
    args.complete()?;

    if let Value::List(list_ref) = self_value {
        let list = list_ref.borrow();

        if list.is_empty() {
            return Ok(Value::Int(0));
        }

        let mut sum = list[0].clone();

        for item in list.iter().skip(1) {
            // Use op_add by calling the bound method directly
            sum = crate::core::builtins::op_add(
                &sum,
                crate::core::Args::positional(vec![item.clone()]),
            )?;
        }

        Ok(sum)
    } else {
        Err(anyhow!("sum() called on non-list value"))
    }
}
