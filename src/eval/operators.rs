use super::core::Evaluator;
use crate::parser::{AstNode, BinaryOperator, UnaryOperator};
use crate::value::types::{ErrorType, RuntimeError, Value};

impl Evaluator {
    pub(crate) fn evaluate_binary_op(
        &mut self,
        left: &AstNode,
        operator: &BinaryOperator,
        right: &AstNode,
    ) -> Result<Value, RuntimeError> {
        // Handle short-circuit evaluation for logical operators
        match operator {
            BinaryOperator::And => {
                let left_val = self.evaluate(left)?;
                if !left_val.is_truthy()? {
                    return Ok(left_val);
                }
                self.evaluate(right)
            }
            BinaryOperator::Or => {
                let left_val = self.evaluate(left)?;
                if left_val.is_truthy()? {
                    return Ok(left_val);
                }
                self.evaluate(right)
            }
            _ => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                self.apply_binary_operator(&left_val, operator, &right_val)
            }
        }
    }

    fn apply_binary_operator(
        &self,
        left: &Value,
        operator: &BinaryOperator,
        right: &Value,
    ) -> Result<Value, RuntimeError> {
        match operator {
            // Arithmetic
            BinaryOperator::Add => self.add_values(left, right),
            BinaryOperator::Subtract => self.subtract_values(left, right),
            BinaryOperator::Multiply => self.multiply_values(left, right),
            BinaryOperator::Divide => self.divide_values(left, right),
            BinaryOperator::Modulo => self.modulo_values(left, right),
            BinaryOperator::Power => self.power_values(left, right),

            // Comparison
            BinaryOperator::Equal => Ok(Value::Bool(left == right)),
            BinaryOperator::NotEqual => Ok(Value::Bool(left != right)),
            BinaryOperator::Less => self.compare_values(left, right, |a, b| a < b),
            BinaryOperator::Greater => self.compare_values(left, right, |a, b| a > b),
            BinaryOperator::LessEqual => self.compare_values(left, right, |a, b| a <= b),
            BinaryOperator::GreaterEqual => self.compare_values(left, right, |a, b| a >= b),

            // Bitwise
            BinaryOperator::BitwiseAnd => self.bitwise_and(left, right),
            BinaryOperator::BitwiseOr => self.bitwise_or(left, right),
            BinaryOperator::BitwiseXor => self.bitwise_xor(left, right),
            BinaryOperator::LeftShift => self.left_shift(left, right),
            BinaryOperator::RightShift => self.right_shift(left, right),

            _ => Err(RuntimeError::new(
                format!("Binary operator {:?} not implemented", operator),
                ErrorType::RuntimeError,
            )),
        }
    }

    pub(crate) fn add_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Int(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Float(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Int(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Float(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(RuntimeError::new(
                format!("Cannot add {} and {}", left.type_name(), right.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn subtract_values(
        &self,
        left: &Value,
        right: &Value,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(RuntimeError::new(
                format!(
                    "Cannot subtract {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn multiply_values(
        &self,
        left: &Value,
        right: &Value,
    ) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(RuntimeError::new(
                format!(
                    "Cannot multiply {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn divide_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::new(
                        "Division by zero".to_string(),
                        ErrorType::ZeroDivisionError,
                    ))
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / *b as f64)),
            _ => Err(RuntimeError::new(
                format!(
                    "Cannot divide {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(RuntimeError::new(
                        "Modulo by zero".to_string(),
                        ErrorType::ZeroDivisionError,
                    ))
                } else {
                    Ok(Value::Int(a % b))
                }
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 % b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a % *b as f64)),
            _ => Err(RuntimeError::new(
                format!(
                    "Cannot modulo {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                ErrorType::TypeError,
            )),
        }
    }

    fn power_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b < 0 {
                    Ok(Value::Float((*a as f64).powf(*b as f64)))
                } else {
                    Ok(Value::Int(a.pow(*b as u32)))
                }
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(*b as f64))),
            _ => Err(RuntimeError::new(
                format!(
                    "Cannot exponentiate {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                ErrorType::TypeError,
            )),
        }
    }

    fn compare_values<F>(&self, left: &Value, right: &Value, op: F) -> Result<Value, RuntimeError>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(op(*a as f64, *b as f64))),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(op(*a, *b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(op(*a as f64, *b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(op(*a, *b as f64))),
            (Value::String(a), Value::String(b)) => {
                let cmp_result = a.cmp(b);
                let result = match cmp_result {
                    std::cmp::Ordering::Less => op(-1.0, 0.0),
                    std::cmp::Ordering::Greater => op(1.0, 0.0),
                    std::cmp::Ordering::Equal => op(0.0, 0.0),
                };
                Ok(Value::Bool(result))
            }
            _ => Err(RuntimeError::new(
                format!(
                    "Cannot compare {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                ErrorType::TypeError,
            )),
        }
    }

    fn bitwise_and(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a & b)),
            _ => Err(RuntimeError::new(
                "Bitwise AND requires integers".to_string(),
                ErrorType::TypeError,
            )),
        }
    }

    fn bitwise_or(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a | b)),
            _ => Err(RuntimeError::new(
                "Bitwise OR requires integers".to_string(),
                ErrorType::TypeError,
            )),
        }
    }

    fn bitwise_xor(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a ^ b)),
            _ => Err(RuntimeError::new(
                "Bitwise XOR requires integers".to_string(),
                ErrorType::TypeError,
            )),
        }
    }

    fn left_shift(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b < 0 {
                    Err(RuntimeError::new(
                        "Negative shift count".to_string(),
                        ErrorType::ValueError,
                    ))
                } else {
                    Ok(Value::Int(a << b))
                }
            }
            _ => Err(RuntimeError::new(
                "Left shift requires integers".to_string(),
                ErrorType::TypeError,
            )),
        }
    }

    fn right_shift(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => {
                if *b < 0 {
                    Err(RuntimeError::new(
                        "Negative shift count".to_string(),
                        ErrorType::ValueError,
                    ))
                } else {
                    Ok(Value::Int(a >> b))
                }
            }
            _ => Err(RuntimeError::new(
                "Right shift requires integers".to_string(),
                ErrorType::TypeError,
            )),
        }
    }

    pub(crate) fn evaluate_unary_op(
        &mut self,
        operator: &UnaryOperator,
        operand: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let value = self.evaluate(operand)?;

        match operator {
            UnaryOperator::Plus => match value {
                Value::Int(_) | Value::Float(_) => Ok(value),
                _ => Err(RuntimeError::new(
                    format!("Cannot apply unary + to {}", value.type_name()),
                    ErrorType::TypeError,
                )),
            },
            UnaryOperator::Minus => match value {
                Value::Int(i) => Ok(Value::Int(-i)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(RuntimeError::new(
                    format!("Cannot apply unary - to {}", value.type_name()),
                    ErrorType::TypeError,
                )),
            },
            UnaryOperator::Not => {
                let truthy = value.is_truthy()?;
                Ok(Value::Bool(!truthy))
            }
            UnaryOperator::BitwiseNot => match value {
                Value::Int(i) => Ok(Value::Int(!i)),
                _ => Err(RuntimeError::new(
                    "Bitwise NOT requires integer".to_string(),
                    ErrorType::TypeError,
                )),
            },
        }
    }
}
