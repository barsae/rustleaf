use rustleaf::{Lexer, Parser, Evaluator, Value};

#[test]
fn test_basic_arithmetic() {
    let source = "2 + 3 * 4;";
    
    let mut lexer = Lexer::new(source);
    let (tokens, _) = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&ast).unwrap();
    
    assert_eq!(result, Value::Int(14)); // 2 + (3 * 4)
}

#[test]
fn test_variable_assignment_and_access() {
    let source = r#"
        var x = 42;
        var y = x + 8;
        y;
    "#;
    
    let mut lexer = Lexer::new(source);
    let (tokens, _) = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&ast).unwrap();
    
    assert_eq!(result, Value::Int(50));
}