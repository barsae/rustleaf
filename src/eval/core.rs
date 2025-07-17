use super::environment::Environment;
use crate::lexer::LiteralValue;
use crate::parser::AstNode;
use crate::value::types::{RuntimeError, Value};

pub struct Evaluator {
    pub(crate) environment: Environment,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
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
            AstNode::BinaryOp {
                left,
                operator,
                right,
                ..
            } => self.evaluate_binary_op(left, operator, right),

            // Unary operations
            AstNode::UnaryOp {
                operator, operand, ..
            } => self.evaluate_unary_op(operator, operand),

            // Function calls
            AstNode::FunctionCall {
                function,
                arguments,
                ..
            } => self.evaluate_function_call(function, arguments),

            // Property access
            AstNode::PropertyAccess {
                object, property, ..
            } => self.evaluate_property_access(object, property),

            // Index access
            AstNode::IndexAccess { object, index, .. } => self.evaluate_index_access(object, index),

            // List literals
            AstNode::ListLiteral { elements, .. } => self.evaluate_list_literal(elements),

            // Dict literals
            AstNode::DictLiteral { entries, .. } => self.evaluate_dict_literal(entries),

            // If expressions
            AstNode::If {
                condition,
                then_branch,
                else_ifs,
                else_branch,
                ..
            } => self.evaluate_if_expression(condition, then_branch, else_ifs, else_branch),

            // Block expressions
            AstNode::Block { statements, .. } => self.evaluate_block(statements),

            // Variable declarations
            AstNode::VariableDeclaration { name, value, .. } => {
                self.evaluate_variable_declaration(name, value)
            }

            // Assignment
            AstNode::Assignment {
                target,
                operator,
                value,
                ..
            } => self.evaluate_assignment(target, operator, value),

            // Expression statements
            AstNode::ExpressionStatement { expression, .. } => self.evaluate(expression),

            // Program
            AstNode::Program { items, .. } => self.evaluate_program(items),

            // Expression forms
            AstNode::Match { .. } => {
                todo!("Match expressions not implemented yet")
            }
            AstNode::Try { .. } => {
                todo!("Try expressions not implemented yet")
            }
            AstNode::AnonymousFunction { .. } => {
                todo!("Anonymous functions not implemented yet")
            }

            // Statement forms
            AstNode::FunctionDeclaration { .. } => {
                todo!("Function declarations not implemented yet")
            }
            AstNode::ClassDeclaration { .. } => {
                todo!("Class declarations not implemented yet")
            }
            AstNode::ImportStatement { .. } => {
                todo!("Import statements not implemented yet")
            }
            AstNode::WhileStatement { .. } => {
                todo!("While statements not implemented yet")
            }
            AstNode::ForStatement { .. } => {
                todo!("For statements not implemented yet")
            }
            AstNode::MatchStatement { .. } => {
                todo!("Match statements not implemented yet")
            }
            AstNode::TryStatement { .. } => {
                todo!("Try statements not implemented yet")
            }
            AstNode::WithStatement { .. } => {
                todo!("With statements not implemented yet")
            }
            AstNode::BreakStatement { .. } => {
                todo!("Break statements not implemented yet")
            }
            AstNode::ContinueStatement { .. } => {
                todo!("Continue statements not implemented yet")
            }
            AstNode::ReturnStatement { .. } => {
                todo!("Return statements not implemented yet")
            }
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
}
