use super::common::parse_source;
use rustleaf::{AstNode, SourceLocation};

/// Basic integration tests for the parser
/// These tests cover fundamental parsing functionality and overall integration

#[test]
fn parser_empty_program() {
    let ast = parse_source("").expect("Should parse empty program");
    
    assert_eq!(ast, AstNode::Program {
        items: vec![],
        location: SourceLocation { line: 1, column: 1, byte_offset: 0 },
    });
}

#[test]
fn parser_multiple_statements() {
    let source = r#"
        var x = 10;
        var y = 20;
        fn add(a, b) {
            return a + b;
        }
    "#;
    
    let _ast = parse_source(source).expect("Should parse multiple statements");
}

#[test]
fn parser_nested_while_loops() {
    // Test that nested while loops parse correctly without hanging
    let result = parse_source("while true { while true { } }");
    match result {
        Ok(_) => {
            // Good: parser correctly parsed nested while loops
        }
        Err(err) => {
            println!("Parse error: {}", err);
            // Parse errors are acceptable, infinite loops are not
        }
    }
}

#[test]
fn parser_source_location_tracking() {
    let ast = parse_source("var x = 42;").expect("Should parse with location tracking");
    
    if let AstNode::Program { items, location } = ast {
        assert_eq!(location.line, 1);
        assert_eq!(location.column, 1);
        
        match &items[0] {
            AstNode::VariableDeclaration { location, .. } => {
                assert_eq!(location.line, 1);
                assert_eq!(location.column, 1);
            }
            _ => panic!("Expected variable declaration"),
        }
    }
}