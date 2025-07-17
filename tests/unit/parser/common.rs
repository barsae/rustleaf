use rustleaf::{AstNode, Lexer, Parser};

/// Common test utilities for parser tests
pub fn parse_source(input: &str) -> Result<AstNode, String> {
    let tokens = match Lexer::new(input) {
        Ok(tokens) => tokens,
        Err(lexer_errors) => {
            return Err(format!("Lexer errors: {}", lexer_errors));
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => Ok(ast),
        Err(parse_errors) => Err(format!("Parse errors: {:?}", parse_errors)),
    }
}

/// Helper function for debug string testing
pub fn assert_debug_eq(ast: &AstNode, expected: &str) {
    let actual = format!("{:#?}", ast);
    if actual != expected {
        println!("=== ASSERTION FAILED ===");
        println!("Expected:");
        println!("{}", expected);
        println!("\nActual:");
        println!("{}", actual);
        println!("========================");
        panic!("Debug output does not match expected");
    }
}
