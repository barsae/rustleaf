// Integration tests using RustLeaf macro

use rustleaf_macros::rustleaf_tests;

#[rustleaf_tests("tests/integration/assert")]
mod assert_tests {}

#[rustleaf_tests("tests/integration/basic")]
mod basic_tests {}
