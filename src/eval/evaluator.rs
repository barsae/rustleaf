use super::scope::ScopeRef;
use crate::{core::*, eval::Eval};
use anyhow::anyhow;

#[derive(Debug)]
pub enum ErrorKind {
    SystemError(anyhow::Error),
    RaisedError(Value),
}

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    Break(Value),
    Continue,
    Error(ErrorKind),
}

pub type EvalResult = Result<Value, ControlFlow>;

#[derive(Debug, Clone)]
struct RustLeafFunction {
    params: Vec<String>,
    body: Eval,
    closure_env: ScopeRef, // Capture the environment where function was defined
}

impl RustLeafFunction {
    fn into_value(self) -> Value {
        Value::from_rust(self)
    }
}

impl RustValue for RustLeafFunction {
    fn call(&self, args: Args) -> anyhow::Result<Value> {
        // Check argument count first
        if args.len() != self.params.len() {
            return Err(anyhow!(
                "Function expects {} arguments, got {}",
                self.params.len(),
                args.len()
            ));
        }

        // Create new scope for function execution
        let function_scope = self.closure_env.child();

        // Set function name for better error messages
        args.set_function_name("user function");

        // Bind parameters to arguments using fluent API
        for param in &self.params {
            let arg_value = args.expect(param)?;
            function_scope.define(param.clone(), arg_value);
        }

        // Validate all args consumed
        args.complete()?;

        // Create evaluator with function scope - use same globals as parent
        let mut evaluator = Evaluator {
            globals: self.closure_env.clone(), // Use closure environment as globals for now
            current_env: function_scope,
        };

        // Evaluate function body
        match evaluator.eval(&self.body) {
            Ok(value) => Ok(value),
            Err(ControlFlow::Return(value)) => Ok(value),
            Err(ControlFlow::Error(ErrorKind::SystemError(err))) => Err(err),
            Err(ControlFlow::Error(ErrorKind::RaisedError(value))) => {
                // Convert raised value to string for error display
                match value {
                    Value::String(s) => Err(anyhow::anyhow!("{}", s)),
                    _ => Err(anyhow::anyhow!("{:?}", value)),
                }
            }
            Err(other) => Err(anyhow!("Invalid control flow in function: {:?}", other)),
        }
    }
}

#[derive(Debug, Clone)]
struct TypeConstant {
    type_name: String,
}

impl TypeConstant {
    fn new(type_name: &str) -> Self {
        Self {
            type_name: type_name.to_string(),
        }
    }
}

impl RustValue for TypeConstant {
    fn call(&self, args: Args) -> anyhow::Result<Value> {
        Err(anyhow!("Type constants are not callable"))
    }

    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "name" => Some(Value::String(self.type_name.clone())),
            _ => None,
        }
    }

    fn op_is(&self, other: &Value) -> anyhow::Result<Value> {
        // Type checking: check if other value matches this type
        let matches = match self.type_name.as_str() {
            "Null" => matches!(other, Value::Null),
            "Unit" => matches!(other, Value::Unit),
            "Bool" => matches!(other, Value::Bool(_)),
            "Int" => matches!(other, Value::Int(_)),
            "Float" => matches!(other, Value::Float(_)),
            "String" => matches!(other, Value::String(_)),
            "List" => matches!(other, Value::List(_)),
            "Dict" => matches!(other, Value::Dict(_)),
            "Range" => matches!(other, Value::Range(_)),
            "Function" => matches!(other, Value::RustValue(_)), // Functions are RustValues
            _ => false,
        };
        Ok(Value::Bool(matches))
    }
}

pub struct Evaluator {
    pub globals: ScopeRef,
    pub current_env: ScopeRef,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        let globals = ScopeRef::new();

        let mut evaluator = Evaluator {
            globals: globals.clone(),
            current_env: globals,
        };

        // Register built-in functions
        evaluator.register_builtins();
        evaluator
    }

    fn register_builtins(&mut self) {
        self.register_builtin_fn("print", print);
        self.register_builtin_fn("assert", crate::core::assert);
        self.register_builtin_fn("is_unit", crate::core::is_unit);
        self.register_builtin_fn("str", crate::core::str_conversion);
        self.register_builtin_fn("raise", crate::core::raise);

        // Register type constants for `is` operator
        self.register_type_constants();
    }

    fn register_type_constants(&mut self) {
        // Create type constants as special values
        self.globals.define(
            "Null".to_string(),
            Value::from_rust(TypeConstant::new("Null")),
        );
        self.globals.define(
            "Unit".to_string(),
            Value::from_rust(TypeConstant::new("Unit")),
        );
        self.globals.define(
            "Bool".to_string(),
            Value::from_rust(TypeConstant::new("Bool")),
        );
        self.globals.define(
            "Int".to_string(),
            Value::from_rust(TypeConstant::new("Int")),
        );
        self.globals.define(
            "Float".to_string(),
            Value::from_rust(TypeConstant::new("Float")),
        );
        self.globals.define(
            "String".to_string(),
            Value::from_rust(TypeConstant::new("String")),
        );
        self.globals.define(
            "List".to_string(),
            Value::from_rust(TypeConstant::new("List")),
        );
        self.globals.define(
            "Dict".to_string(),
            Value::from_rust(TypeConstant::new("Dict")),
        );
        self.globals.define(
            "Range".to_string(),
            Value::from_rust(TypeConstant::new("Range")),
        );
        self.globals.define(
            "Function".to_string(),
            Value::from_rust(TypeConstant::new("Function")),
        );
    }

    fn register_builtin_fn(&mut self, name: &'static str, func: fn(Args) -> anyhow::Result<Value>) {
        let rust_fn = Value::from_rust(RustFunction::new(name, func));
        self.globals.define(name.to_string(), rust_fn);
    }

    pub fn eval(&mut self, eval: &Eval) -> EvalResult {
        match eval {
            Eval::Block(statements, final_expr) => {
                // Create a new scope for the block
                let block_scope = self.current_env.child();
                let previous_env = std::mem::replace(&mut self.current_env, block_scope);

                let mut result = Value::Unit;

                // Execute each statement in the block
                for stmt in statements {
                    match self.eval(stmt) {
                        Ok(_) => {}
                        Err(e) => {
                            // Restore previous scope before propagating error
                            self.current_env = previous_env;
                            return Err(e);
                        }
                    }
                }

                // If there's a final expression, evaluate it and return its value
                if let Some(final_expr) = final_expr {
                    result = match self.eval(final_expr) {
                        Ok(val) => val,
                        Err(e) => {
                            // Restore previous scope before propagating error
                            self.current_env = previous_env;
                            return Err(e);
                        }
                    };
                }

                // Restore the previous scope
                self.current_env = previous_env;
                Ok(result)
            }
            Eval::Literal(value) => Ok(value.clone()),
            Eval::Variable(name) => self.current_env.get(name).ok_or_else(|| {
                ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                    "Undefined variable: {}",
                    name
                )))
            }),
            Eval::Call(func_expr, args) => {
                // Evaluate the function expression
                let func_value = self.eval(func_expr)?;

                // Evaluate all arguments
                let arg_values: Result<Vec<Value>, ControlFlow> =
                    args.iter().map(|arg| self.eval(arg)).collect();
                let arg_values = arg_values?;

                // Create Args object for function call
                let args_obj = Args::positional(arg_values);

                // Call the function
                let result = func_value
                    .call(args_obj)
                    .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))?;

                // Check if the result is a Value::Error (from raise() function)
                if let Value::Error(error_value) = result {
                    return Err(ControlFlow::Error(ErrorKind::RaisedError(*error_value)));
                }

                Ok(result)
            }
            Eval::Declare(name, init_expr) => {
                let value = match init_expr {
                    Some(expr) => self.eval(expr)?,
                    None => Value::Unit,
                };
                self.current_env.define(name.clone(), value);
                Ok(Value::Unit)
            }
            Eval::Function(name, params, body) => {
                let function = RustLeafFunction {
                    params: params.clone(),
                    body: (**body).clone(),
                    closure_env: self.current_env.clone(),
                };
                let function_value = function.into_value();
                self.current_env.define(name.clone(), function_value);
                Ok(Value::Unit)
            }
            Eval::Lambda(params, body) => {
                let function = RustLeafFunction {
                    params: params.clone(),
                    body: (**body).clone(),
                    closure_env: self.current_env.clone(),
                };
                Ok(function.into_value())
            }
            Eval::Assign(name, expr) => {
                let value = self.eval(expr)?;
                self.current_env
                    .set(name, value)
                    .map_err(|err| ControlFlow::Error(ErrorKind::SystemError(anyhow!(err))))?;
                Ok(Value::Unit)
            }
            Eval::If(condition, then_expr, else_expr) => {
                let condition_val = self.eval(condition)?;

                // Check if condition is truthy
                let is_truthy = match condition_val {
                    Value::Bool(b) => b,
                    Value::Unit => false,
                    _ => true, // All other values are truthy (like Python/JavaScript)
                };

                if is_truthy {
                    self.eval(then_expr)
                } else {
                    match else_expr {
                        Some(expr) => self.eval(expr),
                        None => Ok(Value::Unit),
                    }
                }
            }
            Eval::Loop(body) => {
                loop {
                    match self.eval(body) {
                        Ok(_) => {
                            // Normal completion, continue looping
                            continue;
                        }
                        Err(ControlFlow::Break(value)) => {
                            // Break out of loop with value
                            return Ok(value);
                        }
                        Err(ControlFlow::Continue) => {
                            // Continue to next iteration
                            continue;
                        }
                        Err(other) => {
                            // Propagate other control flow (Return, Error)
                            return Err(other);
                        }
                    }
                }
            }
            Eval::While(condition, body) => {
                loop {
                    // Evaluate condition
                    let condition_val = self.eval(condition)?;

                    // Check if condition is truthy
                    let is_truthy = match condition_val {
                        Value::Bool(b) => b,
                        Value::Unit => false,
                        _ => true, // All other values are truthy (like Python/JavaScript)
                    };

                    if !is_truthy {
                        // Condition is false, exit loop
                        return Ok(Value::Unit);
                    }

                    // Execute body
                    match self.eval(body) {
                        Ok(_) => {
                            // Normal completion, continue looping
                            continue;
                        }
                        Err(ControlFlow::Break(value)) => {
                            // Break out of loop with value
                            return Ok(value);
                        }
                        Err(ControlFlow::Continue) => {
                            // Continue to next iteration
                            continue;
                        }
                        Err(other) => {
                            // Propagate other control flow (Return, Error)
                            return Err(other);
                        }
                    }
                }
            }
            Eval::For(var_name, iter_expr, body) => {
                // Evaluate the iterator expression
                let iter_value = self.eval(iter_expr)?;

                // Create a new scope for the loop variable
                let loop_scope = self.current_env.child();
                let previous_env = std::mem::replace(&mut self.current_env, loop_scope);

                let mut result = Value::Unit;

                // Iterate based on the collection type
                match iter_value {
                    Value::List(list_ref) => {
                        let list = list_ref.borrow();
                        for item in list.iter() {
                            // Set the loop variable
                            self.current_env.define(var_name.clone(), item.clone());

                            // Execute body
                            match self.eval(body) {
                                Ok(_) => {
                                    // Normal completion, continue to next iteration
                                }
                                Err(ControlFlow::Break(value)) => {
                                    // Break out of loop with value
                                    result = value;
                                    break;
                                }
                                Err(ControlFlow::Continue) => {
                                    // Continue to next iteration
                                    continue;
                                }
                                Err(other) => {
                                    // Restore scope and propagate other control flow (Return, Error)
                                    self.current_env = previous_env;
                                    return Err(other);
                                }
                            }
                        }
                    }
                    Value::Dict(dict_ref) => {
                        let dict = dict_ref.borrow();
                        for (key, _value) in dict.iter() {
                            // For dictionaries, iterate over keys (like Python)
                            self.current_env
                                .define(var_name.clone(), Value::String(key.clone()));

                            // Execute body
                            match self.eval(body) {
                                Ok(_) => {
                                    // Normal completion, continue to next iteration
                                }
                                Err(ControlFlow::Break(value)) => {
                                    // Break out of loop with value
                                    result = value;
                                    break;
                                }
                                Err(ControlFlow::Continue) => {
                                    // Continue to next iteration
                                    continue;
                                }
                                Err(other) => {
                                    // Restore scope and propagate other control flow (Return, Error)
                                    self.current_env = previous_env;
                                    return Err(other);
                                }
                            }
                        }
                    }
                    Value::Range(range) => {
                        let end_value = if range.inclusive {
                            range.end + 1
                        } else {
                            range.end
                        };
                        for i in range.start..end_value {
                            // Set the loop variable
                            self.current_env.define(var_name.clone(), Value::Int(i));

                            // Execute body
                            match self.eval(body) {
                                Ok(_) => {
                                    // Normal completion, continue to next iteration
                                }
                                Err(ControlFlow::Break(value)) => {
                                    // Break out of loop with value
                                    result = value;
                                    break;
                                }
                                Err(ControlFlow::Continue) => {
                                    // Continue to next iteration
                                    continue;
                                }
                                Err(other) => {
                                    // Restore scope and propagate other control flow (Return, Error)
                                    self.current_env = previous_env;
                                    return Err(other);
                                }
                            }
                        }
                    }
                    _ => {
                        // Restore scope and return error
                        self.current_env = previous_env;
                        return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                            "Cannot iterate over value: {:?}",
                            iter_value
                        ))));
                    }
                }

                // Restore the previous scope
                self.current_env = previous_env;
                Ok(result)
            }
            Eval::Break(expr) => {
                let value = match expr {
                    Some(e) => self.eval(e)?,
                    None => Value::Unit,
                };
                Err(ControlFlow::Break(value))
            }
            Eval::Continue => Err(ControlFlow::Continue),
            Eval::Return(expr) => {
                let value = match expr {
                    Some(e) => self.eval(e)?,
                    None => Value::Unit,
                };
                Err(ControlFlow::Return(value))
            }
            Eval::GetAttr(obj_expr, attr_name) => {
                let obj_value = self.eval(obj_expr)?;

                // Special handling for class instance method access
                if let Value::RustValue(rust_val_ref) = &obj_value {
                    let rust_val = rust_val_ref.borrow();
                    if rust_val.is_class_instance() {
                        // Check if this is a method access
                        if let Some(method) = rust_val.get_class_method(attr_name) {
                            // Create BoundMethod with current evaluator context
                            let bound_method = crate::eval::BoundMethod {
                                instance: obj_value.clone(),
                                method,
                                closure_env: self.globals.clone(),
                            };
                            return Ok(Value::from_rust(bound_method));
                        }
                    }
                }

                // Fall back to standard attribute access
                match obj_value.get_attr(attr_name) {
                    Some(value) => Ok(value),
                    None => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "No attribute '{}' on value {:?}",
                        attr_name,
                        obj_value
                    )))),
                }
            }

            // Built-in operations that don't use method dispatch
            Eval::LogicalAnd(left, right) => {
                let left_val = self.eval(left)?;
                // Short-circuit evaluation
                if !left_val.is_truthy().unwrap_or(true) {
                    Ok(left_val)
                } else {
                    self.eval(right)
                }
            }
            Eval::LogicalOr(left, right) => {
                let left_val = self.eval(left)?;
                // Short-circuit evaluation
                if left_val.is_truthy().unwrap_or(true) {
                    Ok(left_val)
                } else {
                    self.eval(right)
                }
            }
            Eval::LogicalNot(expr) => {
                let val = self.eval(expr)?;
                Ok(Value::Bool(!val.is_truthy().unwrap_or(true)))
            }
            Eval::Is(left, right) => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;

                // Check if right side is a RustValue that implements op_is
                if let Value::RustValue(rust_val_ref) = &right_val {
                    let rust_val = rust_val_ref.borrow();
                    match rust_val.op_is(&left_val) {
                        Ok(result) => return Ok(result),
                        Err(_) => {
                            // Fall through to identity comparison if op_is is not supported
                        }
                    }
                }

                // Fall back to identity comparison
                Ok(Value::Bool(left_val == right_val))
            }

            // Collections
            Eval::List(elements) => {
                let mut list_values = Vec::new();
                for element in elements {
                    list_values.push(self.eval(element)?);
                }
                Ok(Value::new_list_with_values(list_values))
            }
            Eval::Dict(pairs) => {
                let mut dict_map = indexmap::IndexMap::new();
                for (key_expr, value_expr) in pairs {
                    let key_val = self.eval(key_expr)?;
                    let value_val = self.eval(value_expr)?;

                    // Convert key to string
                    let key_str = match key_val {
                        Value::String(s) => s,
                        Value::Int(i) => i.to_string(),
                        Value::Float(f) => f.to_string(),
                        Value::Bool(b) => b.to_string(),
                        _ => {
                            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                "Dictionary keys must be strings, numbers, or booleans, got {:?}",
                                key_val
                            ))))
                        }
                    };

                    dict_map.insert(key_str, value_val);
                }
                Ok(Value::new_dict_with_map(dict_map))
            }
            Eval::GetItem(obj_expr, index_expr) => {
                let obj_value = self.eval(obj_expr)?;
                let index_value = self.eval(index_expr)?;

                // Use operator method system: a[b] → a.op_get_attr("op_get_item").op_call(b)
                match obj_value.get_attr("op_get_item") {
                    Some(method) => {
                        let args = Args::positional(vec![index_value]);
                        method
                            .call(args)
                            .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))
                    }
                    None => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "No op_get_item method on value {:?}",
                        obj_value
                    )))),
                }
            }
            Eval::SetAttr(obj_expr, attr_name, value_expr) => {
                let obj_value = self.eval(obj_expr)?;
                let new_value = self.eval(value_expr)?;

                // For RustValue types, try calling set_attr directly
                match obj_value {
                    Value::RustValue(rv) => match rv.borrow_mut().set_attr(attr_name, new_value) {
                        Ok(_) => Ok(Value::Unit),
                        Err(err) => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(err)))),
                    },
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot set attribute '{}' on value {:?}",
                        attr_name,
                        obj_value
                    )))),
                }
            }
            Eval::SetItem(obj_expr, index_expr, value_expr) => {
                let obj_value = self.eval(obj_expr)?;
                let index_value = self.eval(index_expr)?;
                let new_value = self.eval(value_expr)?;

                // Use operator method system: a[b] = c → a.op_get_attr("op_set_item").op_call(b, c)
                match obj_value.get_attr("op_set_item") {
                    Some(method) => {
                        let args = Args::positional(vec![index_value, new_value]);
                        method
                            .call(args)
                            .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))?;
                        Ok(Value::Unit)
                    }
                    None => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "No op_set_item method on value {:?}",
                        obj_value
                    )))),
                }
            }
            Eval::Try(body, catch_var, catch_body) => {
                // Execute the try body
                match self.eval(body) {
                    Ok(value) => Ok(value),
                    Err(ControlFlow::Error(ErrorKind::RaisedError(error_value))) => {
                        // Create new scope for catch block
                        let catch_scope = self.current_env.child();
                        let previous_env = std::mem::replace(&mut self.current_env, catch_scope);

                        // Bind the error value to the catch variable
                        self.current_env.define(catch_var.clone(), error_value);

                        // Execute catch body
                        let result = self.eval(catch_body);

                        // Restore previous scope
                        self.current_env = previous_env;

                        result
                    }
                    Err(other_error) => Err(other_error), // System errors and other control flow
                }
            }
            Eval::ClassDecl {
                name,
                field_names,
                field_defaults,
                methods,
            } => {
                use crate::eval::Class;

                // Create the class definition
                let class = Class::new(
                    name.clone(),
                    field_names.clone(),
                    field_defaults.clone(),
                    methods.clone(),
                );

                // Store the class in the current scope as a callable value
                let class_value = Value::from_rust(class);
                self.current_env.define(name.clone(), class_value);

                Ok(Value::Unit)
            }
        }
    }
}
