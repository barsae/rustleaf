// Integration tests using RustLeaf macro

use rustleaf_macros::rustleaf_tests;

// Assertions
#[rustleaf_tests("tests/integration/assert")]
mod assert_tests {}

// Classes and OOP features
#[rustleaf_tests("tests/integration/classes")]
mod class_tests {}

// Control flow (if, loops, etc)
#[rustleaf_tests("tests/integration/control_flow")]
mod control_flow_tests {}

// Core language features
#[rustleaf_tests("tests/integration/core")]
mod core_tests {}

// Dictionaries
#[rustleaf_tests("tests/integration/dict")]
mod dict_tests {}

// Functions
#[rustleaf_tests("tests/integration/functions")]
mod function_tests {}

// Import system
#[rustleaf_tests("tests/integration/import")]
mod import_tests {}

// Lambda expressions
#[rustleaf_tests("tests/integration/lambdas")]
mod lambda_tests {}

// Lists
#[rustleaf_tests("tests/integration/list")]
mod list_tests {}

// Macros
#[rustleaf_tests("tests/integration/macros")]
mod macro_tests {}

// Operators
#[rustleaf_tests("tests/integration/operators")]
mod operator_tests {}

// Parser tests
#[rustleaf_tests("tests/integration/parse")]
mod parse_tests {}

// Pattern matching
#[rustleaf_tests("tests/integration/patterns")]
mod pattern_tests {}

// Ranges
#[rustleaf_tests("tests/integration/range")]
mod range_tests {}

// String features
#[rustleaf_tests("tests/integration/strings")]
mod string_tests {}
