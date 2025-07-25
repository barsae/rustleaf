use super::scope::ScopeRef;
use crate::{core::*, eval::Eval};
use anyhow::anyhow;
use std::path::{Path, PathBuf};

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
    params: Vec<(String, Option<Value>, ParameterKind)>, // (name, default_value, kind)
    body: Eval,
    closure_env: ScopeRef, // Capture the environment where function was defined
}

impl RustLeafFunction {
    fn into_value(self) -> Value {
        Value::from_rust(self)
    }
}

impl RustValue for RustLeafFunction {
    fn call(&self, mut args: Args) -> anyhow::Result<Value> {
        // Analyze parameter structure
        let regular_params: Vec<_> = self
            .params
            .iter()
            .filter(|(_, _, kind)| matches!(kind, ParameterKind::Regular))
            .collect();
        let rest_param = self
            .params
            .iter()
            .find(|(_, _, kind)| matches!(kind, ParameterKind::Rest));

        // Count required parameters (regular params without defaults)
        let required_params = regular_params
            .iter()
            .filter(|(_, default, _)| default.is_none())
            .count();
        let max_regular_params = regular_params.len();

        // Check argument count - with rest params, we accept unlimited args
        if rest_param.is_some() {
            // With rest param: need at least required regular params
            if args.len() < required_params {
                return Err(anyhow!(
                    "Function expects at least {} arguments, got {}",
                    required_params,
                    args.len()
                ));
            }
        } else {
            // Without rest param: standard validation
            if args.len() < required_params || args.len() > max_regular_params {
                return Err(anyhow!(
                    "Function expects {} to {} arguments, got {}",
                    required_params,
                    max_regular_params,
                    args.len()
                ));
            }
        }

        // Create new scope for function execution
        let function_scope = self.closure_env.child();

        // Set function name for better error messages
        args.set_function_name("user function");

        // Bind regular parameters to arguments using fluent API
        for (param_name, default_value, kind) in &self.params {
            match kind {
                ParameterKind::Regular => {
                    let arg_value = if let Some(default) = default_value {
                        // Parameter has default - use optional with default
                        args.optional(param_name, default.clone())
                    } else {
                        // Required parameter
                        args.expect(param_name)?
                    };
                    function_scope.define(param_name.clone(), arg_value);
                }
                ParameterKind::Rest => {
                    // Collect remaining arguments into a list
                    let rest_args = args.rest();
                    let list_value = Value::new_list_with_values(rest_args);
                    function_scope.define(param_name.clone(), list_value);
                }
                ParameterKind::Keyword => {
                    // TODO: Implement keyword parameters later
                    return Err(anyhow!("Keyword parameters not yet implemented"));
                }
            }
        }

        // Validate all args consumed
        args.complete()?;

        // Create evaluator with function scope - use same globals as parent
        let mut evaluator = Evaluator {
            globals: self.closure_env.clone(), // Use closure environment as globals for now
            current_env: function_scope,
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
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
    pub current_dir: PathBuf,
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
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        };

        // Register built-in functions
        evaluator.register_builtins();
        evaluator
    }

    pub fn new_with_dir(current_dir: PathBuf) -> Self {
        let globals = ScopeRef::new();

        let mut evaluator = Evaluator {
            globals: globals.clone(),
            current_env: globals,
            current_dir,
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

                // Special handling for class constructors
                if let Value::RustValue(rust_value_ref) = &func_value {
                    let rust_value = rust_value_ref.borrow();
                    if let Some((name, field_names, field_defaults, methods)) =
                        rust_value.as_class_constructor()
                    {
                        drop(rust_value); // Release borrow
                        let class_data =
                            crate::eval::Class::new(name, field_names, field_defaults, methods);
                        return self.handle_class_constructor(&class_data, args);
                    }
                }

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
            Eval::DeclarePattern(pattern, init_expr) => {
                let value = self.eval(init_expr)?;
                self.match_pattern(pattern, &value)?;
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
                // Convert Vec<String> to Vec<(String, Option<Value>, ParameterKind)> - lambdas don't have defaults or rest
                let param_data: Vec<(String, Option<Value>, ParameterKind)> = params
                    .iter()
                    .map(|name| (name.clone(), None, ParameterKind::Regular))
                    .collect();

                let function = RustLeafFunction {
                    params: param_data,
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
            Eval::With(resources, body) => {
                // Create new scope for with block
                let with_scope = self.current_env.child();
                let previous_env = std::mem::replace(&mut self.current_env, with_scope);

                let mut resource_values = Vec::new();

                // Evaluate and bind resources, call op_open on each
                for (name, resource_expr) in resources {
                    match self.eval(resource_expr) {
                        Ok(resource_value) => {
                            // Bind resource to scope
                            self.current_env
                                .define(name.clone(), resource_value.clone());

                            // Call op_open() if it exists - use the same logic as GetAttr evaluation
                            let open_method =
                                self.get_method_from_value(&resource_value, "op_open");
                            if let Some(open_method) = open_method {
                                // Bound methods already have self bound, so call with no arguments
                                let args = Args::positional(vec![]);
                                if let Err(e) = open_method.call(args) {
                                    // If op_open fails, cleanup already opened resources and propagate error
                                    self.cleanup_resources(&resource_values);
                                    self.current_env = previous_env;
                                    return Err(ControlFlow::Error(ErrorKind::SystemError(e)));
                                }
                            }

                            resource_values.push((name.clone(), resource_value));
                        }
                        Err(e) => {
                            // If resource evaluation fails, cleanup already opened resources and propagate error
                            self.cleanup_resources(&resource_values);
                            self.current_env = previous_env;
                            return Err(e);
                        }
                    }
                }

                // Execute the body
                let result = match self.eval(body) {
                    Ok(val) => val,
                    Err(e) => {
                        // Cleanup on error
                        self.cleanup_resources(&resource_values);
                        self.current_env = previous_env;
                        return Err(e);
                    }
                };

                // Cleanup resources (call op_close in reverse order)
                self.cleanup_resources(&resource_values);

                // Restore previous scope
                self.current_env = previous_env;
                Ok(result)
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
            Eval::Import { module, items } => {
                // Load and evaluate the module
                let module_scope = self.load_module(module)?;

                // Built-in names to exclude from imports
                let builtin_names: std::collections::HashSet<&str> = [
                    "print", "assert", "is_unit", "str", "raise", "Null", "Unit", "Bool", "Int",
                    "Float", "String", "List", "Dict", "Range", "Function",
                ]
                .iter()
                .cloned()
                .collect();

                // Import items into current scope (ignoring pub visibility for now)
                match items {
                    ImportItems::All => {
                        // Import all non-builtin items from the module
                        for (name, value) in module_scope.iter() {
                            if !builtin_names.contains(name.as_str()) {
                                self.current_env.define(name, value);
                            }
                        }
                    }
                    ImportItems::Specific(import_items) => {
                        for import_item in import_items {
                            let item_name = &import_item.name;
                            let alias = import_item.alias.as_ref().unwrap_or(item_name);

                            match module_scope.get(item_name) {
                                Some(value) => {
                                    self.current_env.define(alias.clone(), value);
                                }
                                None => {
                                    // For debugging, let's see what's actually in the module scope (excluding built-ins)
                                    let available_items: Vec<_> = module_scope
                                        .iter()
                                        .into_iter()
                                        .filter(|(k, _)| !builtin_names.contains(k.as_str()))
                                        .map(|(k, _)| k)
                                        .collect();
                                    return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                        "Module '{}' does not have item '{}'. Available items: {:?}",
                                        module,
                                        item_name,
                                        available_items
                                    ))));
                                }
                            }
                        }
                    }
                }

                Ok(Value::Unit)
            }
            Eval::Match { expr, cases } => {
                let match_value = self.eval(expr)?;

                for case in cases {
                    // Check if pattern matches
                    if self.match_pattern_matches(&case.pattern, &match_value)? {
                        // Check guard if present
                        if let Some(guard) = &case.guard {
                            let guard_result = self.eval(guard)?;
                            if !Self::is_truthy(&guard_result) {
                                continue; // Guard failed, try next case
                            }
                        }

                        // Pattern matches and guard passes (if any), execute body
                        // Create new scope for pattern variable bindings
                        let match_scope = self.current_env.child();
                        let previous_env = std::mem::replace(&mut self.current_env, match_scope);

                        // Bind pattern variables if needed
                        if let crate::eval::core::EvalMatchPattern::Variable(var_name) =
                            &case.pattern
                        {
                            self.current_env
                                .define(var_name.clone(), match_value.clone());
                        }

                        let result = self.eval(&case.body);

                        // Restore previous scope
                        self.current_env = previous_env;

                        return result;
                    }
                }

                // No case matched
                Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                    "No matching case found in match expression for value: {:?}",
                    match_value
                ))))
            }
        }
    }

    /// Helper method to get a method from a value, using the same logic as GetAttr evaluation
    fn get_method_from_value(&self, obj_value: &Value, attr_name: &str) -> Option<Value> {
        // Special handling for class instance method access
        if let Value::RustValue(rust_val_ref) = obj_value {
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
                    return Some(Value::from_rust(bound_method));
                }
            }
        }

        // Fall back to standard attribute access
        obj_value.get_attr(attr_name)
    }

    /// Handle class constructor calls by evaluating default field expressions
    fn handle_class_constructor(
        &mut self,
        class: &crate::eval::Class,
        args: &[crate::eval::Eval],
    ) -> EvalResult {
        use std::collections::HashMap;

        // Constructor call - create new instance
        if !args.is_empty() {
            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow::anyhow!(
                "Class constructor takes no arguments"
            ))));
        }

        // Create new instance with properly evaluated default field values
        let mut fields = HashMap::new();
        for (i, field_name) in class.field_names.iter().enumerate() {
            let value = if let Some(default_expr) = &class.field_defaults[i] {
                // Evaluate the default expression
                self.eval(default_expr)?
            } else {
                Value::Null
            };
            fields.insert(field_name.clone(), value);
        }

        Ok(Value::from_rust(crate::eval::ClassInstance {
            class_name: class.name.clone(),
            fields,
            methods: class.methods.clone(),
        }))
    }

    /// Helper method to cleanup resources by calling op_close() in reverse order
    fn cleanup_resources(&self, resources: &[(String, Value)]) {
        // Cleanup in reverse order
        for (_name, resource_value) in resources.iter().rev() {
            let close_method = self.get_method_from_value(resource_value, "op_close");
            if let Some(close_method) = close_method {
                // Bound methods already have self bound, so call with no arguments
                let args = Args::positional(vec![]);
                // Ignore errors during cleanup - we don't want cleanup errors to mask the original error
                let _ = close_method.call(args);
            }
        }
    }

    /// Load and evaluate a module, returning its scope
    fn load_module(&self, module_name: &str) -> Result<ScopeRef, ControlFlow> {
        // Resolve module path
        let module_path = self.resolve_module_path(module_name)?;

        // Read module file
        let module_content = std::fs::read_to_string(&module_path).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to read module '{}' at path '{}': {}",
                module_name,
                module_path.display(),
                e
            )))
        })?;

        // Parse module content
        let tokens = crate::lexer::Lexer::tokenize(&module_content).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to tokenize module '{}': {}",
                module_name,
                e
            )))
        })?;

        let ast = crate::parser::Parser::parse(tokens).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to parse module '{}': {}",
                module_name,
                e
            )))
        })?;

        // Compile to eval IR
        let eval_ir = crate::eval::Compiler::compile(ast).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to compile module '{}': {}",
                module_name,
                e
            )))
        })?;

        // Create new evaluator for module with correct current_dir
        let module_dir = module_path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut module_evaluator = Evaluator::new_with_dir(module_dir);

        // Evaluate the module - handle module-level definitions specially
        match eval_ir {
            Eval::Block(statements, final_expr) => {
                // For modules, evaluate statements directly in the module scope (not in a child scope)
                for stmt in &statements {
                    module_evaluator.eval(stmt)?;
                }
                // Evaluate final expression if present
                if let Some(final_expr) = &final_expr {
                    module_evaluator.eval(final_expr)?;
                }
            }
            _ => {
                // For non-block evaluation, evaluate normally
                module_evaluator.eval(&eval_ir)?;
            }
        }

        // Return the module's current scope (which includes both built-ins and module items)
        // We'll filter out built-ins at import time instead
        Ok(module_evaluator.current_env)
    }

    /// Resolve module path based on current directory and module name
    fn resolve_module_path(&self, module_name: &str) -> Result<PathBuf, ControlFlow> {
        // For now, implement simple relative path resolution
        // Convert module::submodule to module/submodule.rustleaf
        let relative_path = module_name.replace("::", "/") + ".rustleaf";
        let full_path = self.current_dir.join(relative_path);

        if !full_path.exists() {
            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Module file not found: {}",
                full_path.display()
            ))));
        }

        Ok(full_path)
    }

    /// Pattern matching helper - binds variables from patterns
    fn match_pattern(
        &mut self,
        pattern: &crate::eval::core::EvalPattern,
        value: &Value,
    ) -> Result<(), ControlFlow> {
        use crate::eval::core::EvalPattern;

        match pattern {
            EvalPattern::Variable(name) => {
                // Simple variable binding
                self.current_env.define(name.clone(), value.clone());
                Ok(())
            }
            EvalPattern::List(patterns) => {
                // List destructuring
                match value {
                    Value::List(list_ref) => {
                        let list = list_ref.borrow();

                        if list.len() != patterns.len() {
                            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                "Cannot destructure list of length {} into pattern with {} elements",
                                list.len(),
                                patterns.len()
                            ))));
                        }

                        // Recursively match each pattern with corresponding list element
                        for (pattern, list_value) in patterns.iter().zip(list.iter()) {
                            self.match_pattern(pattern, list_value)?;
                        }

                        Ok(())
                    }
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot destructure non-list value {:?} with list pattern",
                        value
                    )))),
                }
            }
            EvalPattern::ListRest(patterns, rest_name) => {
                // List destructuring with rest capture: [first, *rest]
                match value {
                    Value::List(list_ref) => {
                        let list = list_ref.borrow();

                        // Must have at least as many elements as fixed patterns
                        if list.len() < patterns.len() {
                            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                "Cannot destructure list of length {} with rest pattern expecting at least {} elements",
                                list.len(),
                                patterns.len()
                            ))));
                        }

                        // Match fixed patterns at the beginning
                        for (i, pattern) in patterns.iter().enumerate() {
                            let list_value = &list[i];
                            self.match_pattern(pattern, list_value)?;
                        }

                        // Capture the rest as a new list (if rest_name is provided)
                        if let Some(rest_var) = rest_name {
                            let rest_values: Vec<Value> = list[patterns.len()..].to_vec();
                            let rest_list = Value::new_list_with_values(rest_values);
                            self.current_env.define(rest_var.clone(), rest_list);
                        }

                        Ok(())
                    }
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot destructure non-list value {:?} with list rest pattern",
                        value
                    )))),
                }
            }
            EvalPattern::Dict(dict_patterns) => {
                // Dict destructuring
                match value {
                    Value::Dict(dict_ref) => {
                        let dict = dict_ref.borrow();

                        for dict_pattern in dict_patterns {
                            // Look up the value in the dict using string key directly
                            let dict_value = dict.get(&dict_pattern.key).ok_or_else(|| {
                                ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                    "Key '{}' not found in dict during destructuring",
                                    dict_pattern.key
                                )))
                            })?;

                            // Determine the variable name (alias if present, otherwise key)
                            let var_name = dict_pattern.alias.as_ref().unwrap_or(&dict_pattern.key);

                            // Bind the value to the variable
                            self.current_env
                                .define(var_name.clone(), dict_value.clone());
                        }
                        Ok(())
                    }
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot destructure non-dict value {:?} with dict pattern",
                        value
                    )))),
                }
            }
        }
    }

    // Helper method to check if a match pattern matches a value
    fn match_pattern_matches(
        &self,
        pattern: &crate::eval::core::EvalMatchPattern,
        value: &Value,
    ) -> Result<bool, ControlFlow> {
        use crate::eval::core::EvalMatchPattern;

        match pattern {
            EvalMatchPattern::Literal(literal) => {
                // Compare values for equality
                Ok(literal == value)
            }
            EvalMatchPattern::Variable(_) => {
                // Variables always match (and bind the value)
                Ok(true)
            }
            EvalMatchPattern::Wildcard => {
                // Wildcard always matches
                Ok(true)
            }
        }
    }

    // Helper method to check if a value is truthy
    fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(list) => !list.borrow().is_empty(),
            Value::Dict(dict) => !dict.borrow().is_empty(),
            _ => true, // Other values are truthy
        }
    }
}
