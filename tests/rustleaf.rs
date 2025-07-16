use rustleaf_macros::rustleaf_tests;

#[rustleaf_tests("./tests/rustleaf")]
mod generated_tests {}

// Keep the failing test separate since it needs should_panic
#[test]
#[should_panic(expected = "Assertion failed")]
fn rustleaf_assert_fail() {
    let source = std::fs::read_to_string("./tests/rustleaf/assert_fail.rustleaf.skip")
        .unwrap_or_else(|_| panic!("Failed to read test file"));
    
    let tokens = rustleaf::Lexer::new(&source).unwrap();
    
    let mut parser = rustleaf::Parser::new(tokens);
    let ast = parser.parse().unwrap();
    
    let mut evaluator = rustleaf::Evaluator::new();
    evaluator.evaluate(&ast).unwrap();
}