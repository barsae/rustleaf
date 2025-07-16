use std::collections::HashMap;
use crate::value::value::{Value, RuntimeError, ErrorType};
use crate::parser::AstNode;
use super::core::Evaluator;

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
                    if let Some(builtin) = self.environment.get_builtin(&func.name.unwrap_or_default()) {
                        (builtin.function)(&args, &mut self.environment)
                    } else {
                        Err(RuntimeError::new(
                            "Builtin function not found".to_string(),
                            ErrorType::RuntimeError,
                        ))
                    }
                } else {
                    // User-defined function - not implemented yet
                    Err(RuntimeError::new(
                        "User-defined functions not implemented yet".to_string(),
                        ErrorType::RuntimeError,
                    ))
                }
            },
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
                        format!("'{}' object has no attribute '{}'", obj.class_name, property),
                        ErrorType::AttributeError,
                    ))
                }
            },
            Value::Dict(dict) => {
                if let Some(value) = dict.get(property) {
                    Ok(value.clone())
                } else {
                    Ok(Value::Null)
                }
            },
            Value::Null => Err(RuntimeError::new(
                "Cannot access property of null".to_string(),
                ErrorType::AttributeError,
            )),
            _ => Err(RuntimeError::new(
                format!("'{}' object has no attribute '{}'", obj_value.type_name(), property),
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
            },
            Value::Dict(dict) => {
                if let Value::String(key) = index_value {
                    Ok(dict.get(&key).cloned().unwrap_or(Value::Null))
                } else {
                    Err(RuntimeError::new(
                        "Dict keys must be strings".to_string(),
                        ErrorType::TypeError,
                    ))
                }
            },
            Value::String(s) => {
                if let Value::Int(i) = index_value {
                    let len = s.len() as i64;
                    let idx = if i < 0 { len + i } else { i };
                    
                    if idx >= 0 && (idx as usize) < s.len() {
                        Ok(Value::String(s.chars().nth(idx as usize).unwrap().to_string()))
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
            },
            _ => Err(RuntimeError::new(
                format!("'{}' object is not subscriptable", obj_value.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn evaluate_list_literal(&mut self, elements: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut list = Vec::new();
        for element in elements {
            list.push(self.evaluate(element)?);
        }
        Ok(Value::List(list))
    }

    pub(crate) fn evaluate_dict_literal(&mut self, entries: &[(AstNode, AstNode)]) -> Result<Value, RuntimeError> {
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
}