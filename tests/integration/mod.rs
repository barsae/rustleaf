// Integration tests using RustLeaf macro

use rustleaf_macros::rustleaf_tests;

// Expression evaluation tests
#[rustleaf_tests("tests/integration/expressions")]
mod expression_tests {}

// Statement execution tests
#[rustleaf_tests("tests/integration/statements")]
mod statement_tests {}

// Data type tests
#[rustleaf_tests("tests/integration/data_types")]
mod data_type_tests {}

// Operator tests
#[rustleaf_tests("tests/integration/operators")]
mod operator_tests {}

// Builtin function tests
#[rustleaf_tests("tests/integration/builtins")]
mod builtin_tests {}

// Function declaration and invocation tests
#[rustleaf_tests("tests/integration/functions")]
mod function_tests {}
