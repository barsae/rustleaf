// Integration tests using RustLeaf macro

use rustleaf_macros::rustleaf_tests;

// If expression tests
#[rustleaf_tests("tests/integration/if")]
mod if_tests {}

// Logical expression tests
#[rustleaf_tests("tests/integration/logical")]
mod logical_tests {}

// Math expression tests
#[rustleaf_tests("tests/integration/math")]
mod math_tests {}

// Comparison expression tests
#[rustleaf_tests("tests/integration/comparison")]
mod comparison_tests {}

// Block expression tests
#[rustleaf_tests("tests/integration/block")]
mod block_tests {}

// Variable assignment tests
#[rustleaf_tests("tests/integration/variables")]
mod variable_tests {}

// Try/catch statement tests
#[rustleaf_tests("tests/integration/try_catch")]
mod try_catch_tests {}

// Dictionary tests
#[rustleaf_tests("tests/integration/dicts")]
mod dict_tests {}

// List tests
#[rustleaf_tests("tests/integration/lists")]
mod list_tests {}

// String tests
#[rustleaf_tests("tests/integration/strings")]
mod string_tests {}

// Null value tests
#[rustleaf_tests("tests/integration/null")]
mod null_tests {}

// Operator tests
#[rustleaf_tests("tests/integration/operators")]
mod operator_tests {}

// Assert function tests
#[rustleaf_tests("tests/integration/assert")]
mod assert_tests {}

// Function declaration and invocation tests
#[rustleaf_tests("tests/integration/functions")]
mod function_tests {}
