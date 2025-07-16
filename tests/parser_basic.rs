use rustleaf::{Lexer, Parser, AstNode, LiteralValue, BinaryOperator, Visibility};
use std::time::{Duration, Instant};
use std::panic;
use std::thread;

fn parse_source(input: &str) -> Result<AstNode, String> {
    parse_source_with_timeout(input, Duration::from_secs(5))
}

fn parse_source_with_timeout(input: &str, timeout: Duration) -> Result<AstNode, String> {
    let start = Instant::now();
    
    let mut lexer = Lexer::new(input);
    let (tokens, lexer_errors) = lexer.tokenize();
    
    if !lexer_errors.is_empty() {
        return Err(format!("Lexer errors: {:?}", lexer_errors));
    }
    
    if start.elapsed() > timeout {
        return Err("Timeout during lexing".to_string());
    }
    
    let mut parser = Parser::new(tokens);
    
    // Use a separate thread with timeout for parsing
    let (tx, rx) = std::sync::mpsc::channel();
    let _input_clone = input.to_string();
    
    thread::spawn(move || {
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            parser.parse()
        }));
        
        match result {
            Ok(parse_result) => {
                match parse_result {
                    Ok(ast) => tx.send(Ok(ast)).unwrap_or(()),
                    Err(parse_errors) => tx.send(Err(format!("Parse errors: {:?}", parse_errors))).unwrap_or(()),
                }
            }
            Err(_) => {
                tx.send(Err("Parser panicked".to_string())).unwrap_or(())
            }
        }
    });
    
    match rx.recv_timeout(timeout) {
        Ok(result) => result,
        Err(_) => Err(format!("Parser timeout after {:?} for input: {:?}", timeout, input)),
    }
}

fn assert_program_with_items(ast: &AstNode, expected_item_count: usize) {
    match ast {
        AstNode::Program { items, .. } => {
            assert_eq!(items.len(), expected_item_count, "Expected {} items in program", expected_item_count);
        }
        _ => panic!("Expected Program node, got {:?}", ast),
    }
}

#[test]
fn test_empty_program() {
    let ast = parse_source("").expect("Should parse empty program");
    assert_program_with_items(&ast, 0);
}

#[test]
fn test_simple_variable_declaration() {
    let ast = parse_source("var x = 42;").expect("Should parse variable declaration");
    assert_program_with_items(&ast, 1);
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::VariableDeclaration { name, value, .. } => {
                assert_eq!(name, "x");
                if let Some(val) = value {
                    match val.as_ref() {
                        AstNode::Literal(LiteralValue::Integer(42), _) => {},
                        _ => panic!("Expected integer literal 42"),
                    }
                } else {
                    panic!("Expected variable to have a value");
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }
}

#[test]
fn test_simple_function_declaration() {
    let ast = parse_source("fn hello() { }").expect("Should parse function declaration");
    assert_program_with_items(&ast, 1);
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::FunctionDeclaration { visibility, name, parameters, .. } => {
                assert_eq!(*visibility, Visibility::Private);
                assert_eq!(name, "hello");
                assert_eq!(parameters.len(), 0);
            }
            _ => panic!("Expected function declaration"),
        }
    }
}

#[test]
fn test_simple_arithmetic() {
    let ast = parse_source("var result = 2 + 3;").expect("Should parse arithmetic");
    assert_program_with_items(&ast, 1);
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::VariableDeclaration { value: Some(val), .. } => {
                match val.as_ref() {
                    AstNode::BinaryOp { operator: BinaryOperator::Add, .. } => {},
                    _ => panic!("Expected addition expression"),
                }
            }
            _ => panic!("Expected variable declaration with value"),
        }
    }
}

#[test]
fn test_simple_if_statement() {
    let ast = parse_source("if true { }").expect("Should parse if statement");
    assert_program_with_items(&ast, 1);
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::If { condition, .. } => {
                match condition.as_ref() {
                    AstNode::Literal(LiteralValue::Boolean(true), _) => {},
                    _ => panic!("Expected boolean literal true"),
                }
            }
            _ => panic!("Expected if statement"),
        }
    }
}

#[test]
fn test_import_statement() {
    let ast = parse_source("use std;").expect("Should parse simple import");
    assert_program_with_items(&ast, 1);
    
    if let AstNode::Program { items, .. } = ast {
        match &items[0] {
            AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path, &vec!["std".to_string()]);
                assert!(clause.is_none(), "Expected no import clause for simple import");
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn test_expression_statement() {
    let ast = parse_source("foo();").expect("Should parse expression statement");
    assert_program_with_items(&ast, 1);
    
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