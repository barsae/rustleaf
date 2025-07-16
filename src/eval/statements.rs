use crate::value::value::{Value, RuntimeError, ErrorType};
use crate::parser::{AstNode, AssignmentOperator};
use super::core::Evaluator;

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
                            AssignmentOperator::AddAssign => self.add_values(&current, &new_value)?,
                            AssignmentOperator::SubtractAssign => self.subtract_values(&current, &new_value)?,
                            AssignmentOperator::MultiplyAssign => self.multiply_values(&current, &new_value)?,
                            AssignmentOperator::DivideAssign => self.divide_values(&current, &new_value)?,
                            AssignmentOperator::ModuloAssign => self.modulo_values(&current, &new_value)?,
                            _ => unreachable!(),
                        }
                    }
                };
                
                self.environment.set(name, final_value.clone())?;
                Ok(final_value)
            },
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
}