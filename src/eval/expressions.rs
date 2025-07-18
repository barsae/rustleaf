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
            _ => Err(RuntimeError::new(
                format!("{} is not callable", func_value.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn evaluate_property_access(
        &mut self,
        object: &AstNode,
        property: &str,
    ) -> Result<Value, RuntimeError> {
        let obj_value = self.evaluate(object)?;

        match obj_value {
            Value::Object(obj) => {
                if let Some(value) = obj.fields.get(property) {
                    Ok(value.clone())
                } else {
                    Err(RuntimeError::new(
                        format!(
                            "'{}' object has no attribute '{}'",
                            obj.class_name, property
                        ),
                        ErrorType::AttributeError,
                    ))
                }
            }
            Value::Dict(dict) => {
                if let Some(value) = dict.get(property) {
                    Ok(value.clone())
                } else {
                    Ok(Value::Null)
                }
            }
            Value::Null => Err(RuntimeError::new(
                "Cannot access property of null".to_string(),
                ErrorType::AttributeError,
            )),
            _ => Err(RuntimeError::new(
                format!(
                    "'{}' object has no attribute '{}'",
                    obj_value.type_name(),
                    property
                ),
                ErrorType::AttributeError,
            )),
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

    pub(crate) fn call_user_function(
        &mut self,
        function: &Function,
        args: &[Value],
    ) -> Result<Value, RuntimeError> {
        // Create new scope for function execution
        self.environment.push_scope();

        // Add closure variables to the scope if present
        if let Some(closure) = &function.closure {
            for (name, value) in closure {
                self.environment.define(name.clone(), value.clone());
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
}
