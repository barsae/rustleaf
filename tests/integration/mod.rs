// Integration tests using RustLeaf macro

use rustleaf_macros::rustleaf_tests;

// Advanced features (classes, imports, macros, etc)
#[rustleaf_tests("tests/integration/advanced")]
mod advanced_tests {}

// Assertions
#[rustleaf_tests("tests/integration/assert")]
mod assert_tests {}

// Collections (lists, dicts, ranges)
#[rustleaf_tests("tests/integration/collections")]
mod collection_tests {}

// Control flow (if, loops, etc)
#[rustleaf_tests("tests/integration/control_flow")]
mod control_flow_tests {}

// Core language features
#[rustleaf_tests("tests/integration/core")]
mod core_tests {}

// Functions and lambdas
#[rustleaf_tests("tests/integration/functions")]
mod function_tests {}

// Operators
#[rustleaf_tests("tests/integration/operators")]
mod operator_tests {}

// Pattern matching
#[rustleaf_tests("tests/integration/patterns")]
mod pattern_tests {}
