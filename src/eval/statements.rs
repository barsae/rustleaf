use super::core::Evaluator;
use crate::parser::{AssignmentOperator, AstNode, Parameter};
use crate::value::types::{ErrorType, Function, RuntimeError, Value};

impl Evaluator {
    pub(crate) fn evaluate_block(&mut self, statements: &[AstNode]) -> Result<Value, RuntimeError> {
        self.environment.push_scope();

        let mut result = Value::Null;
        for (i, stmt) in statements.iter().enumerate() {
            if i == statements.len() - 1 {
                // Last statement/expression is the block value
                result = self.evaluate(stmt)?;
            } else {
                self.evaluate(stmt)?;
            }
        }

        self.environment.pop_scope();
        Ok(result)
    }

    pub(crate) fn evaluate_variable_declaration(
        &mut self,
        name: &str,
        value: &Option<Box<AstNode>>,
    ) -> Result<Value, RuntimeError> {
        let val = if let Some(value_node) = value {
            self.evaluate(value_node)?
        } else {
            Value::Null
        };

        self.environment.define(name.to_string(), val.clone());
        Ok(val)
    }

    pub(crate) fn evaluate_assignment(
        &mut self,
        target: &AstNode,
        operator: &AssignmentOperator,
        value: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let new_value = self.evaluate(value)?;

        match target {
            AstNode::Identifier(name, _) => {
                let final_value = match operator {
                    AssignmentOperator::Assign => new_value,
                    _ => {
                        let current = self.environment.get(name)?;
                        match operator {
                            AssignmentOperator::AddAssign => {
                                self.add_values(&current, &new_value)?
                            }
                            AssignmentOperator::SubtractAssign => {
                                self.subtract_values(&current, &new_value)?
                            }
                            AssignmentOperator::MultiplyAssign => {
                                self.multiply_values(&current, &new_value)?
                            }
                            AssignmentOperator::DivideAssign => {
                                self.divide_values(&current, &new_value)?
                            }
                            AssignmentOperator::ModuloAssign => {
                                self.modulo_values(&current, &new_value)?
                            }
                            _ => unreachable!(),
                        }
                    }
                };

                self.environment.set(name, final_value.clone())?;
                Ok(final_value)
            }
            _ => Err(RuntimeError::new(
                "Complex assignment targets not implemented yet".to_string(),
                ErrorType::RuntimeError,
            )),
        }
    }

    pub(crate) fn evaluate_program(&mut self, items: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut result = Value::Null;
        for item in items {
            result = self.evaluate(item)?;
        }
        Ok(result)
    }

    pub(crate) fn evaluate_function_declaration(
        &mut self,
        name: &str,
        parameters: &[Parameter],
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        // Capture current environment for closures (if not at global scope)
        let closure = if self.environment.is_nested_scope() {
            Some(self.environment.capture_closure())
        } else {
            None
        };

        let function = Function {
            name: Some(name.to_string()),
            parameters: parameters.to_vec(),
            body: body.clone(),
            closure,
            is_builtin: false,
        };

        let value = Value::Function(function);
        self.environment.define(name.to_string(), value.clone());
        Ok(value)
    }

    pub(crate) fn evaluate_anonymous_function(
        &mut self,
        parameters: &[Parameter],
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        // Capture current environment for closures
        let closure = if self.environment.is_nested_scope() {
            Some(self.environment.capture_closure())
        } else {
            None
        };

        let function = Function {
            name: None,
            parameters: parameters.to_vec(),
            body: body.clone(),
            closure,
            is_builtin: false,
        };

        Ok(Value::Function(function))
    }

    pub(crate) fn evaluate_return_statement(
        &mut self,
        value: &Option<Box<AstNode>>,
    ) -> Result<Value, RuntimeError> {
        if let Some(value_node) = value {
            let result = self.evaluate(value_node)?;
            // Use a special error type to signal return
            Err(
                RuntimeError::new("RETURN_VALUE".to_string(), ErrorType::RuntimeError)
                    .with_return_value(result),
            )
        } else {
            Err(
                RuntimeError::new("RETURN_NULL".to_string(), ErrorType::RuntimeError)
                    .with_return_value(Value::Null),
            )
        }
    }

    pub(crate) fn evaluate_for_statement(
        &mut self,
        variable: &str,
        index_variable: &Option<String>,
        iterable: &AstNode,
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let iterable_value = self.evaluate(iterable)?;

        // Create new scope for loop
        self.environment.push_scope();

        let mut result = Value::Null;

        match iterable_value {
            Value::List(list) => {
                for (index, item) in list.iter().enumerate() {
                    // Bind loop variable
                    self.environment.define(variable.to_string(), item.clone());

                    // Bind index variable if present
                    if let Some(index_var) = index_variable {
                        self.environment
                            .define(index_var.clone(), Value::Int(index as i64));
                    }

                    // Execute body
                    match self.evaluate(body) {
                        Ok(value) => result = value,
                        Err(err) => {
                            if err.is_return() {
                                // Return from function, not just loop
                                self.environment.pop_scope();
                                return Err(err);
                            }
                            // For now, ignore other errors (break/continue would go here)
                        }
                    }
                }
            }
            Value::String(s) => {
                for (index, ch) in s.chars().enumerate() {
                    // Bind loop variable
                    self.environment
                        .define(variable.to_string(), Value::String(ch.to_string()));

                    // Bind index variable if present
                    if let Some(index_var) = index_variable {
                        self.environment
                            .define(index_var.clone(), Value::Int(index as i64));
                    }

                    // Execute body
                    match self.evaluate(body) {
                        Ok(value) => result = value,
                        Err(err) => {
                            if err.is_return() {
                                // Return from function, not just loop
                                self.environment.pop_scope();
                                return Err(err);
                            }
                            // For now, ignore other errors (break/continue would go here)
                        }
                    }
                }
            }
            _ => {
                self.environment.pop_scope();
                return Err(RuntimeError::new(
                    format!("'{}' object is not iterable", iterable_value.type_name()),
                    ErrorType::TypeError,
                ));
            }
        }

        self.environment.pop_scope();
        Ok(result)
    }
}
