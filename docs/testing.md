# Testing Guide

## Integration Tests

Integration tests are automatically discovered and generated using the `#[rustleaf_tests]` macro in `rustleaf-macros`.

### Test File Naming Conventions

Integration test files use the `.rustleaf` extension and support special suffixes:

- **`_panic.rustleaf`**: Tests that should panic with "Assertion failed"
  - Generates `#[should_panic(expected = "Assertion failed")]` attribute
  - Use for testing assertion failures or expected runtime errors

- **`_ignore.rustleaf`**: Tests that should be ignored by default
  - Generates `#[ignore]` attribute
  - Run with `cargo test -- --ignored`
  - Use for slow tests, known failing tests, or platform-specific tests

- **`.rustleaf`**: Standard tests that should pass
  - No special attributes
  - Run with normal `cargo test`

### Examples

```
tests/integration/basic/
├── arithmetic.rustleaf          # Standard test
├── division_by_zero_panic.rustleaf  # Should panic
└── slow_computation_ignore.rustleaf # Ignored by default
```

### Test Discovery

The macro automatically:
1. Scans the specified directory for `.rustleaf` files
2. Generates test function names by converting path separators to underscores
3. Applies appropriate test attributes based on filename suffixes
4. Sets up proper module resolution relative to the test file location

### Usage in Test Modules

```rust
#[rustleaf_tests("tests/integration/basic")]
mod basic_tests {}
```

This generates individual test functions for each `.rustleaf` file in the specified directory.