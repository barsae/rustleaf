use super::common::parse_source;
use rustleaf::{AstNode, BinaryOperator};

/// Tests for statement parsing (if, while, for, expression statements, etc.)

#[test]
fn parser_if_statement() {
    let ast = parse_source("if x > 0 { print(x); }").expect("Should parse if statement");
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::If { condition, then_branch, else_ifs, else_branch, .. } => {
                // Check condition is a binary operation
                match condition.as_ref() {
                    AstNode::BinaryOp { operator: BinaryOperator::Greater, .. } => {},
                    _ => panic!("Expected greater than comparison"),
                }
                
                // Check then branch is a block
                match then_branch.as_ref() {
                    AstNode::Block { statements, .. } => {
                        assert_eq!(statements.len(), 1);
                    }
                    _ => panic!("Expected block in then branch"),
                }
                
                assert!(else_ifs.is_empty(), "Expected no else-if clauses");
                assert!(else_branch.is_none(), "Expected no else clause");
            }
            _ => panic!("Expected if statement"),
        }
    }
}

#[test]
fn parser_simple_if_statement() {
    let ast = parse_source("if true { }").expect("Should parse if statement");
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::If { condition, .. } => {
                match condition.as_ref() {
                    AstNode::Literal(rustleaf::LiteralValue::Boolean(true), _) => {},
                    _ => panic!("Expected boolean literal true"),
                }
            }
            _ => panic!("Expected if statement"),
        }
    }
}

#[test]
fn parser_while_loop() {
    let ast = parse_source("while i < 10 { i = i + 1; }").expect("Should parse while loop");
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::WhileStatement { condition, body, .. } => {
                match condition.as_ref() {
                    AstNode::BinaryOp { operator: BinaryOperator::Less, .. } => {},
                    _ => panic!("Expected less than comparison"),
                }
                
                match body.as_ref() {
                    AstNode::Block { statements, .. } => {
                        assert_eq!(statements.len(), 1);
                    }
                    _ => panic!("Expected block as while body"),
                }
            }
            _ => panic!("Expected while statement"),
        }
    }
}

#[test]
fn parser_for_loop() {
    let ast = parse_source("for item in list { process(item); }").expect("Should parse for loop");
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::ForStatement { variable, index_variable, iterable, body, .. } => {
                assert_eq!(variable, "item");
                assert!(index_variable.is_none(), "Expected no index variable");
                
                match iterable.as_ref() {
                    AstNode::Identifier(name, _) => assert_eq!(name, "list"),
                    _ => panic!("Expected identifier for iterable"),
                }
                
                match body.as_ref() {
                    AstNode::Block { statements, .. } => {
                        assert_eq!(statements.len(), 1);
                    }
                    _ => panic!("Expected block as for body"),
                }
            }
            _ => panic!("Expected for statement"),
        }
    }
}

#[test]
fn parser_for_loop_with_index() {
    let ast = parse_source("for item, idx in list { }").expect("Should parse for loop with index");
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::ForStatement { variable, index_variable, .. } => {
                assert_eq!(variable, "item");
                assert_eq!(index_variable.as_ref().unwrap(), "idx");
            }
            _ => panic!("Expected for statement"),
        }
    }
}

#[test]
fn parser_expression_statement() {
    let ast = parse_source("foo();").expect("Should parse expression statement");
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::ExpressionStatement { expression, .. } => {
                match expression.as_ref() {
                    AstNode::FunctionCall { function, arguments, .. } => {
                        match function.as_ref() {
                            AstNode::Identifier(name, _) => assert_eq!(name, "foo"),
                            _ => panic!("Expected identifier for function"),
                        }
                        assert!(arguments.is_empty(), "Expected no arguments");
                    }
                    _ => panic!("Expected function call"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }
}