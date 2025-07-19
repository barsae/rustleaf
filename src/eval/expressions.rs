use super::core::Evaluator;
use crate::parser::AstNode;
use crate::value::types::{ErrorType, Function, RuntimeError, Value};
use std::collections::HashMap;

impl Evaluator {
    pub(crate) fn evaluate_function_call(
        &mut self,
        function: &AstNode,
        arguments: &[crate::parser::Argument],
    ) -> Result<Value, RuntimeError> {
        // Check if this is a method call (function is PropertyAccess)
        if let AstNode::PropertyAccess {
            object, property, ..
        } = function
        {
            return self.evaluate_method_call(object, property, arguments);
        }

        let func_value = self.evaluate(function)?;

        // Evaluate arguments
        let mut args = Vec::new();
        for arg in arguments {
            let value = self.evaluate(&arg.value)?;
            args.push(value);
        }

        match func_value {
            Value::Function(func) => {
                if func.is_builtin {
                    if let Some(builtin) =
                        self.environment.get_builtin(&func.name.unwrap_or_default())
                    {
                        (builtin.function)(&args, &mut self.environment)
                    } else {
                        Err(RuntimeError::new(
                            "Builtin function not found".to_string(),
                            ErrorType::RuntimeError,
                        ))
                    }
                } else {
                    // User-defined function
                    self.call_user_function(&func, &args)
                }
            }
            Value::BoundMethod(method, bound_object) => {
                // For bound methods, we need to handle builtin methods specially
                if method.is_builtin {
                    self.call_builtin_method(&method, &bound_object, &args)
                } else {
                    // For user-defined bound methods, prepend self to args
                    let mut method_args = vec![(*bound_object).clone()];
                    method_args.extend(args);
                    self.call_user_function(&method, &method_args)
                }
            }
            Value::RustValue(ref rust_value) => {
                // Check if this is a class being called (instantiation)
                if let Some(class) = rust_value
                    .as_any()
                    .downcast_ref::<crate::eval::statements::Class>()
                {
                    // Instantiate the class
                    let instance = class.instantiate(self)?;
                    Ok(Value::RustValue(Box::new(instance)))
                } else {
                    Err(RuntimeError::new(
                        format!("{} is not callable", func_value.type_name()),
                        ErrorType::TypeError,
                    ))
                }
            }
            _ => Err(RuntimeError::new(
                format!("{} is not callable", func_value.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    fn evaluate_method_call(
        &mut self,
        object: &AstNode,
        method_name: &str,
        arguments: &[crate::parser::Argument],
    ) -> Result<Value, RuntimeError> {
        let obj_value = self.evaluate(object)?;

        // Evaluate arguments
        let mut args = Vec::new();
        for arg in arguments {
            let value = self.evaluate(&arg.value)?;
            args.push(value);
        }

        // Use the new property access system to get the method, then call it
        match self.evaluate_property_access(object, method_name) {
            Ok(Value::BoundMethod(method, bound_object)) => {
                // Call the bound method
                if method.is_builtin {
                    self.call_builtin_method(&method, &bound_object, &args)
                } else {
                    // For user-defined bound methods, prepend self to args
                    let mut method_args = vec![(*bound_object).clone()];
                    method_args.extend(args);
                    self.call_user_function(&method, &method_args)
                }
            }
            Ok(Value::Function(func)) => {
                // This is a function accessed as a property - call it normally
                if func.is_builtin {
                    if let Some(builtin) =
                        self.environment.get_builtin(&func.name.unwrap_or_default())
                    {
                        (builtin.function)(&args, &mut self.environment)
                    } else {
                        Err(RuntimeError::new(
                            "Builtin function not found".to_string(),
                            ErrorType::RuntimeError,
                        ))
                    }
                } else {
                    self.call_user_function(&func, &args)
                }
            }
            Ok(_) => {
                // Property exists but is not a function
                Err(RuntimeError::new(
                    format!("'{}' is not callable", method_name),
                    ErrorType::TypeError,
                ))
            }
            Err(_) => {
                // Property doesn't exist, try as a method on object
                match obj_value {
                    Value::Object(obj) => {
                        if let Some(method) = obj.methods.get(method_name) {
                            // Add self as first argument
                            let mut method_args = vec![Value::Object(obj.clone())];
                            method_args.extend(args);
                            self.call_user_function(method, &method_args)
                        } else {
                            Err(RuntimeError::new(
                                format!(
                                    "'{}' object has no method '{}'",
                                    obj.class_name, method_name
                                ),
                                ErrorType::AttributeError,
                            ))
                        }
                    }
                    _ => Err(RuntimeError::new(
                        format!(
                            "'{}' object has no method '{}'",
                            obj_value.type_name(),
                            method_name
                        ),
                        ErrorType::AttributeError,
                    )),
                }
            }
        }
    }

    pub(crate) fn evaluate_property_access(
        &mut self,
        object: &AstNode,
        property: &str,
    ) -> Result<Value, RuntimeError> {
        let obj_value = self.evaluate(object)?;

        // Special handling for null
        if matches!(obj_value, Value::Null) {
            return Err(RuntimeError::new(
                "Cannot access property of null".to_string(),
                ErrorType::AttributeError,
            ));
        }

        // Special handling for Dict - they use key access, not attribute access
        if let Value::Dict(dict) = &obj_value {
            return if let Some(value) = dict.get(property) {
                Ok(value.clone())
            } else {
                Ok(Value::Null)
            };
        }

        // Handle legacy Object type for backwards compatibility
        if let Value::Object(obj) = &obj_value {
            if let Some(value) = obj.fields.get(property) {
                return Ok(value.clone());
            } else {
                return Err(RuntimeError::new(
                    format!(
                        "'{}' object has no attribute '{}'",
                        obj.class_name, property
                    ),
                    ErrorType::AttributeError,
                ));
            }
        }

        // Handle RustValue types (Class and ClassInstance)
        if let Value::RustValue(ref rust_value) = obj_value {
            // Handle ClassInstance field access
            if let Some(instance) = rust_value
                .as_any()
                .downcast_ref::<crate::eval::statements::ClassInstance>()
            {
                if let Some(field_value) = instance.get_field(property) {
                    return Ok(field_value.clone());
                } else if instance.get_instance_method(property).is_some() {
                    // Don't return instance methods as functions in property access
                    // They should be handled in method call evaluation
                    return Err(RuntimeError::new(
                        format!(
                            "'{}' object has no attribute '{}'",
                            instance.class_name, property
                        ),
                        ErrorType::AttributeError,
                    ));
                } else {
                    return Err(RuntimeError::new(
                        format!(
                            "'{}' object has no attribute '{}'",
                            instance.class_name, property
                        ),
                        ErrorType::AttributeError,
                    ));
                }
            }
            // Handle Class static method access
            else if let Some(class) = rust_value
                .as_any()
                .downcast_ref::<crate::eval::statements::Class>()
            {
                if let Some(static_method_decl) = class.get_static_method(property) {
                    // Extract the function from the declaration and return it
                    if let AstNode::FunctionDeclaration {
                        parameters, body, ..
                    } = static_method_decl
                    {
                        let func = Function {
                            name: Some(property.to_string()),
                            parameters: parameters.clone(),
                            body: body.as_ref().clone(),
                            is_builtin: false,
                            closure: None,
                        };
                        return Ok(Value::Function(func));
                    } else {
                        return Err(RuntimeError::new(
                            "Invalid static method declaration".to_string(),
                            ErrorType::RuntimeError,
                        ));
                    }
                } else {
                    return Err(RuntimeError::new(
                        format!(
                            "'{}' object has no method '{}'",
                            rust_value.type_name(),
                            property
                        ),
                        ErrorType::AttributeError,
                    ));
                }
            }
        }

        // Use the new op_get_attr system for everything else
        let attr_result = obj_value.op_get_attr(property, &self.environment)?;

        // If we get an UnboundMethod, automatically bind it
        match attr_result {
            Value::UnboundMethod(method) => Ok(obj_value.bind_method(method)),
            other => Ok(other),
        }
    }

    pub(crate) fn evaluate_index_access(
        &mut self,
        object: &AstNode,
        index: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let obj_value = self.evaluate(object)?;
        let index_value = self.evaluate(index)?;

        match obj_value {
            Value::List(list) => {
                if let Value::Int(i) = index_value {
                    let len = list.len() as i64;
                    let idx = if i < 0 { len + i } else { i };

                    if idx >= 0 && (idx as usize) < list.len() {
                        Ok(list[idx as usize].clone())
                    } else {
                        Err(RuntimeError::new(
                            "List index out of range".to_string(),
                            ErrorType::IndexError,
                        ))
                    }
                } else {
                    Err(RuntimeError::new(
                        "List indices must be integers".to_string(),
                        ErrorType::TypeError,
                    ))
                }
            }
            Value::Dict(dict) => {
                if let Value::String(key) = index_value {
                    Ok(dict.get(&key).cloned().unwrap_or(Value::Null))
                } else {
                    Err(RuntimeError::new(
                        "Dict keys must be strings".to_string(),
                        ErrorType::TypeError,
                    ))
                }
            }
            Value::String(s) => {
                if let Value::Int(i) = index_value {
                    let len = s.len() as i64;
                    let idx = if i < 0 { len + i } else { i };

                    if idx >= 0 && (idx as usize) < s.len() {
                        Ok(Value::String(
                            s.chars().nth(idx as usize).unwrap().to_string(),
                        ))
                    } else {
                        Err(RuntimeError::new(
                            "String index out of range".to_string(),
                            ErrorType::IndexError,
                        ))
                    }
                } else {
                    Err(RuntimeError::new(
                        "String indices must be integers".to_string(),
                        ErrorType::TypeError,
                    ))
                }
            }
            _ => Err(RuntimeError::new(
                format!("'{}' object is not subscriptable", obj_value.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn evaluate_list_literal(
        &mut self,
        elements: &[AstNode],
    ) -> Result<Value, RuntimeError> {
        let mut list = Vec::new();
        for element in elements {
            list.push(self.evaluate(element)?);
        }
        Ok(Value::List(list))
    }

    pub(crate) fn evaluate_dict_literal(
        &mut self,
        entries: &[(AstNode, AstNode)],
    ) -> Result<Value, RuntimeError> {
        let mut dict = HashMap::new();
        for (key_node, value_node) in entries {
            let key_value = self.evaluate(key_node)?;
            let value = self.evaluate(value_node)?;

            if let Value::String(key) = key_value {
                dict.insert(key, value);
            } else {
                return Err(RuntimeError::new(
                    "Dict keys must be strings".to_string(),
                    ErrorType::TypeError,
                ));
            }
        }
        Ok(Value::Dict(dict))
    }

    pub(crate) fn evaluate_if_expression(
        &mut self,
        condition: &AstNode,
        then_branch: &AstNode,
        else_ifs: &[(AstNode, AstNode)],
        else_branch: &Option<Box<AstNode>>,
    ) -> Result<Value, RuntimeError> {
        let cond_value = self.evaluate(condition)?;
        if cond_value.is_truthy()? {
            return self.evaluate(then_branch);
        }

        for (elif_cond, elif_body) in else_ifs {
            let elif_value = self.evaluate(elif_cond)?;
            if elif_value.is_truthy()? {
                return self.evaluate(elif_body);
            }
        }

        if let Some(else_body) = else_branch {
            self.evaluate(else_body)
        } else {
            Ok(Value::Null)
        }
    }

    pub(crate) fn call_user_function_with_self(
        &mut self,
        function: &Function,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        // Create new scope for function execution
        self.environment.push_scope();

        // Add closure variables to the scope if present
        if let Some(closure_scope) = &function.closure {
            let closure_vars = closure_scope.get_local_variables();
            for (name, _value) in closure_vars {
                if let Some(value_ref) = closure_scope.get_mut(&name) {
                    self.environment.define_ref(name, value_ref);
                }
            }
        }

        // Bind self parameter first (if we have arguments)
        if !args.is_empty() {
            self.environment.define("self".to_string(), args[0].clone());
        }

        // Bind declared parameters to remaining arguments
        let mut arg_index = 1; // Start from 1 since args[0] is self
        let mut varargs = Vec::new();
        let kwargs = HashMap::new();

        for param in &function.parameters {
            if param.variadic {
                // Collect remaining positional arguments
                while arg_index < args.len() {
                    varargs.push(args[arg_index].clone());
                    arg_index += 1;
                }
                self.environment
                    .define(param.name.clone(), Value::List(varargs.clone()));
            } else if param.keyword_variadic {
                // For now, just create empty dict - keyword arguments not fully implemented
                self.environment
                    .define(param.name.clone(), Value::Dict(kwargs.clone()));
            } else {
                // Regular parameter
                let value = if arg_index < args.len() {
                    args[arg_index].clone()
                } else if let Some(default_expr) = &param.default_value {
                    // Evaluate default value
                    self.evaluate(default_expr)?
                } else {
                    return Err(RuntimeError::new(
                        format!("Missing required argument: {}", param.name),
                        ErrorType::TypeError,
                    ));
                };

                self.environment.define(param.name.clone(), value);
                arg_index += 1;
            }
        }

        // Check for too many arguments (unless we have varargs)
        // Subtract 1 from expected args count since self is implicit
        let expected_args = function.parameters.len();
        let provided_args = args.len() - 1; // Subtract 1 for self

        if provided_args > expected_args && !function.parameters.iter().any(|p| p.variadic) {
            self.environment.pop_scope();
            return Err(RuntimeError::new(
                format!(
                    "Too many arguments: expected {}, got {}",
                    expected_args, provided_args
                ),
                ErrorType::TypeError,
            ));
        }

        // Execute function body
        let result = self.evaluate(&function.body);

        self.environment.pop_scope();
        result
    }

    pub(crate) fn call_user_function(
        &mut self,
        function: &Function,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        // Create new scope for function execution
        self.environment.push_scope();

        // Add closure variables to the scope if present
        if let Some(closure_scope) = &function.closure {
            let closure_vars = closure_scope.get_local_variables();
            for (name, _value) in closure_vars {
                if let Some(value_ref) = closure_scope.get_mut(&name) {
                    self.environment.define_ref(name, value_ref);
                }
            }
        }

        // Bind parameters to arguments
        let mut arg_index = 0;
        let mut varargs = Vec::new();
        let kwargs = HashMap::new();

        for param in &function.parameters {
            if param.variadic {
                // Collect remaining positional arguments
                while arg_index < args.len() {
                    varargs.push(args[arg_index].clone());
                    arg_index += 1;
                }
                self.environment
                    .define(param.name.clone(), Value::List(varargs.clone()));
            } else if param.keyword_variadic {
                // For now, just create empty dict - keyword arguments not fully implemented
                self.environment
                    .define(param.name.clone(), Value::Dict(kwargs.clone()));
            } else {
                // Regular parameter
                let value = if arg_index < args.len() {
                    args[arg_index].clone()
                } else if let Some(default_expr) = &param.default_value {
                    // Evaluate default value
                    self.evaluate(default_expr)?
                } else {
                    return Err(RuntimeError::new(
                        format!("Missing required argument: {}", param.name),
                        ErrorType::TypeError,
                    ));
                };

                self.environment.define(param.name.clone(), value);
                arg_index += 1;
            }
        }

        // Check for too many arguments (unless we have varargs)
        if arg_index < args.len() && !function.parameters.iter().any(|p| p.variadic) {
            self.environment.pop_scope();
            return Err(RuntimeError::new(
                format!(
                    "Too many arguments: expected {}, got {}",
                    function.parameters.len(),
                    args.len()
                ),
                ErrorType::TypeError,
            ));
        }

        // Execute function body
        let result = match self.evaluate(&function.body) {
            Ok(value) => Ok(value),
            Err(err) => {
                if err.is_return() {
                    // Handle return statement
                    Ok(err.return_value.unwrap_or(Value::Unit))
                } else {
                    Err(err)
                }
            }
        };

        // Clean up scope
        self.environment.pop_scope();
        result
    }

    pub(crate) fn evaluate_try_expression(
        &mut self,
        body: &AstNode,
        catch_clause: Option<&crate::parser::CatchClause>,
    ) -> Result<Value, RuntimeError> {
        match self.evaluate(body) {
            Ok(value) => Ok(value),
            Err(error) => {
                if let Some(catch) = catch_clause {
                    // Enter new scope for catch block
                    self.environment.push_scope();

                    // Bind the error value to the catch variable
                    let error_value = if let Some(raised_value) = error.return_value {
                        raised_value
                    } else {
                        // Convert runtime error to string
                        Value::String(error.message)
                    };

                    self.environment.define(catch.variable.clone(), error_value);

                    // Execute catch block
                    let result = self.evaluate(&catch.body);

                    // Clean up scope
                    self.environment.pop_scope();
                    result
                } else {
                    // No catch clause, re-raise the error
                    Err(error)
                }
            }
        }
    }

    fn call_builtin_method(
        &mut self,
        method: &Function,
        bound_object: &Value,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        let unknown_name = "unknown".to_string();
        let method_name = method.name.as_ref().unwrap_or(&unknown_name);

        match method_name.as_str() {
            "len" => {
                if !args.is_empty() {
                    return Err(RuntimeError::new(
                        "len() takes no arguments".to_string(),
                        ErrorType::TypeError,
                    ));
                }

                match bound_object {
                    Value::String(s) => Ok(Value::Int(s.len() as i64)),
                    Value::List(l) => Ok(Value::Int(l.len() as i64)),
                    Value::Dict(d) => Ok(Value::Int(d.len() as i64)),
                    Value::RustValue(rv) => {
                        if let Some(len) = rv.len() {
                            Ok(Value::Int(len))
                        } else {
                            Err(RuntimeError::new(
                                format!("'{}' object has no len()", bound_object.type_name()),
                                ErrorType::AttributeError,
                            ))
                        }
                    }
                    _ => Err(RuntimeError::new(
                        format!("'{}' object has no len()", bound_object.type_name()),
                        ErrorType::AttributeError,
                    )),
                }
            }
            "filter" => {
                if args.len() != 1 {
                    return Err(RuntimeError::new(
                        "filter() takes exactly one argument".to_string(),
                        ErrorType::TypeError,
                    ));
                }

                let filter_func = &args[0];

                match bound_object {
                    Value::RustValue(rv) => {
                        // Check if this is a range object
                        if rv.type_name() == "range" {
                            // Clone the range to iterate through it
                            let mut range_iter = rv.clone_box();
                            let mut filtered_items = Vec::new();

                            // Iterate through the range and apply the filter
                            while let Some(item) = range_iter.iter_next() {
                                // Call the filter function with the item
                                let call_result = match filter_func {
                                    Value::Function(func) => {
                                        self.call_user_function(func, &[item.clone()])
                                    }
                                    _ => {
                                        return Err(RuntimeError::new(
                                            "filter() argument must be a function".to_string(),
                                            ErrorType::TypeError,
                                        ));
                                    }
                                };

                                match call_result {
                                    Ok(Value::Bool(true)) => filtered_items.push(item),
                                    Ok(Value::Bool(false)) => {}
                                    Ok(Value::Null) => {}
                                    Ok(other) => {
                                        return Err(RuntimeError::new(
                                            format!(
                                                "filter function must return bool, got {}",
                                                other.type_name()
                                            ),
                                            ErrorType::TypeError,
                                        ));
                                    }
                                    Err(e) => return Err(e),
                                }
                            }

                            Ok(Value::List(filtered_items))
                        } else {
                            Err(RuntimeError::new(
                                format!("'{}' object has no filter()", bound_object.type_name()),
                                ErrorType::AttributeError,
                            ))
                        }
                    }
                    Value::List(list) => {
                        // Also implement filter for lists
                        let mut filtered_items = Vec::new();

                        for item in list {
                            let call_result = match filter_func {
                                Value::Function(func) => {
                                    self.call_user_function(func, &[item.clone()])
                                }
                                _ => {
                                    return Err(RuntimeError::new(
                                        "filter() argument must be a function".to_string(),
                                        ErrorType::TypeError,
                                    ));
                                }
                            };

                            match call_result {
                                Ok(Value::Bool(true)) => filtered_items.push(item.clone()),
                                Ok(Value::Bool(false)) => {}
                                Ok(Value::Null) => {}
                                Ok(other) => {
                                    return Err(RuntimeError::new(
                                        format!(
                                            "filter function must return bool, got {}",
                                            other.type_name()
                                        ),
                                        ErrorType::TypeError,
                                    ));
                                }
                                Err(e) => return Err(e),
                            }
                        }

                        Ok(Value::List(filtered_items))
                    }
                    _ => Err(RuntimeError::new(
                        format!("'{}' object has no filter()", bound_object.type_name()),
                        ErrorType::AttributeError,
                    )),
                }
            }
            "sum" => {
                if !args.is_empty() {
                    return Err(RuntimeError::new(
                        "sum() takes no arguments".to_string(),
                        ErrorType::TypeError,
                    ));
                }

                match bound_object {
                    Value::List(list) => {
                        let mut sum = Value::Int(0);

                        for item in list {
                            match (&sum, item) {
                                (Value::Int(s), Value::Int(i)) => {
                                    sum = Value::Int(s + i);
                                }
                                (Value::Float(s), Value::Int(i)) => {
                                    sum = Value::Float(s + (*i as f64));
                                }
                                (Value::Int(s), Value::Float(f)) => {
                                    sum = Value::Float((*s as f64) + f);
                                }
                                (Value::Float(s), Value::Float(f)) => {
                                    sum = Value::Float(s + f);
                                }
                                _ => {
                                    return Err(RuntimeError::new(
                                        format!(
                                            "Cannot sum '{}' and '{}'",
                                            sum.type_name(),
                                            item.type_name()
                                        ),
                                        ErrorType::TypeError,
                                    ));
                                }
                            }
                        }

                        Ok(sum)
                    }
                    _ => Err(RuntimeError::new(
                        format!("'{}' object has no sum()", bound_object.type_name()),
                        ErrorType::AttributeError,
                    )),
                }
            }
            _ => Err(RuntimeError::new(
                format!("Unknown builtin method: {}", method_name),
                ErrorType::RuntimeError,
            )),
        }
    }
}
