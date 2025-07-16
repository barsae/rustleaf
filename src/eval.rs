use std::collections::HashMap;
use crate::value::value::{Value, RuntimeError, ErrorType, Function};
use crate::value::function::{get_builtin_functions, BuiltinFunctionInfo};
use crate::parser::{AstNode, BinaryOperator, UnaryOperator, AssignmentOperator};
use crate::lexer::LiteralValue;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    builtins: HashMap<String, BuiltinFunctionInfo>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            scopes: vec![HashMap::new()], // Global scope
            builtins: HashMap::new(),
        };
        
        // Add builtin functions
        for builtin in get_builtin_functions() {
            env.builtins.insert(builtin.name.to_string(), builtin);
        }
        
        env
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        // Check builtins
        if let Some(_builtin) = self.builtins.get(name) {
            return Ok(Value::Function(Function {
                name: Some(name.to_string()),
                parameters: vec![], // Builtins handle their own parameters
                body: AstNode::Block { statements: vec![], location: Default::default() },
                closure: None,
                is_builtin: true,
            }));
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'", name),
            ErrorType::NameError,
        ))
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'", name),
            ErrorType::NameError,
        ))
    }

    pub fn get_builtin(&self, name: &str) -> Option<&BuiltinFunctionInfo> {
        self.builtins.get(name)
    }
}

impl Default for crate::parser::SourceLocation {
    fn default() -> Self {
        crate::parser::SourceLocation {
            line: 0,
            column: 0,
            byte_offset: 0,
        }
    }
}

pub struct Evaluator {
    environment: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::new(),
        }
    }

    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match node {
            // Literals
            AstNode::Literal(literal, _) => self.evaluate_literal(literal),

            // Identifiers
            AstNode::Identifier(name, _) => self.environment.get(name),

            // Binary operations
            AstNode::BinaryOp { left, operator, right, .. } => {
                self.evaluate_binary_op(left, operator, right)
            },

            // Unary operations
            AstNode::UnaryOp { operator, operand, .. } => {
                self.evaluate_unary_op(operator, operand)
            },

            // Function calls
            AstNode::FunctionCall { function, arguments, .. } => {
                self.evaluate_function_call(function, arguments)
            },

            // Property access
            AstNode::PropertyAccess { object, property, .. } => {
                self.evaluate_property_access(object, property)
            },

            // Index access
            AstNode::IndexAccess { object, index, .. } => {
                self.evaluate_index_access(object, index)
            },

            // List literals
            AstNode::ListLiteral { elements, .. } => {
                self.evaluate_list_literal(elements)
            },

            // Dict literals
            AstNode::DictLiteral { entries, .. } => {
                self.evaluate_dict_literal(entries)
            },

            // If expressions
            AstNode::If { condition, then_branch, else_ifs, else_branch, .. } => {
                self.evaluate_if_expression(condition, then_branch, else_ifs, else_branch)
            },

            // Block expressions
            AstNode::Block { statements, .. } => {
                self.evaluate_block(statements)
            },

            // Variable declarations
            AstNode::VariableDeclaration { name, value, .. } => {
                self.evaluate_variable_declaration(name, value)
            },

            // Assignment
            AstNode::Assignment { target, operator, value, .. } => {
                self.evaluate_assignment(target, operator, value)
            },

            // Expression statements
            AstNode::ExpressionStatement { expression, .. } => {
                self.evaluate(expression)
            },

            // Program
            AstNode::Program { items, .. } => {
                self.evaluate_program(items)
            },

            _ => Err(RuntimeError::new(
                format!("Evaluation not implemented for {:?}", node),
                ErrorType::RuntimeError,
            )),
        }
    }

    fn evaluate_literal(&self, literal: &LiteralValue) -> Result<Value, RuntimeError> {
        match literal {
            LiteralValue::Null => Ok(Value::Null),
            LiteralValue::Boolean(b) => Ok(Value::Bool(*b)),
            LiteralValue::Integer(i) => Ok(Value::Int(*i)),
            LiteralValue::Float(f) => Ok(Value::Float(*f)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
        }
    }

    fn evaluate_binary_op(
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
            },
            BinaryOperator::Or => {
                let left_val = self.evaluate(left)?;
                if left_val.is_truthy()? {
                    return Ok(left_val);
                }
                self.evaluate(right)
            },
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

    fn add_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(RuntimeError::new(
                format!("Cannot add {} and {}", left.type_name(), right.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(RuntimeError::new(
                format!("Cannot subtract {} and {}", left.type_name(), right.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),
            _ => Err(RuntimeError::new(
                format!("Cannot multiply {} and {}", left.type_name(), right.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
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
            },
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / *b as f64)),
            _ => Err(RuntimeError::new(
                format!("Cannot divide {} and {}", left.type_name(), right.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
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
            },
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 % b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a % *b as f64)),
            _ => Err(RuntimeError::new(
                format!("Cannot modulo {} and {}", left.type_name(), right.type_name()),
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
            },
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powf(*b as f64))),
            _ => Err(RuntimeError::new(
                format!("Cannot exponentiate {} and {}", left.type_name(), right.type_name()),
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
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(op(0.0, if a < b { -1.0 } else if a > b { 1.0 } else { 0.0 }))),
            _ => Err(RuntimeError::new(
                format!("Cannot compare {} and {}", left.type_name(), right.type_name()),
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
            },
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
            },
            _ => Err(RuntimeError::new(
                "Right shift requires integers".to_string(),
                ErrorType::TypeError,
            )),
        }
    }

    fn evaluate_unary_op(
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
            },
            UnaryOperator::BitwiseNot => match value {
                Value::Int(i) => Ok(Value::Int(!i)),
                _ => Err(RuntimeError::new(
                    "Bitwise NOT requires integer".to_string(),
                    ErrorType::TypeError,
                )),
            },
        }
    }

    fn evaluate_function_call(
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

    fn evaluate_property_access(
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

    fn evaluate_index_access(
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

    fn evaluate_list_literal(&mut self, elements: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut list = Vec::new();
        for element in elements {
            list.push(self.evaluate(element)?);
        }
        Ok(Value::List(list))
    }

    fn evaluate_dict_literal(&mut self, entries: &[(AstNode, AstNode)]) -> Result<Value, RuntimeError> {
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

    fn evaluate_if_expression(
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

    fn evaluate_block(&mut self, statements: &[AstNode]) -> Result<Value, RuntimeError> {
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

    fn evaluate_variable_declaration(
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

    fn evaluate_assignment(
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

    fn evaluate_program(&mut self, items: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut result = Value::Null;
        for item in items {
            result = self.evaluate(item)?;
        }
        Ok(result)
    }
}