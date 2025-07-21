
# RustLeaf

## Specs

The specifications live at `./specs/*.md`. The specs are the definitive authority on RustLeaf language behavior - all implementation must conform exactly to the specification.

## Development and Testing

`just test` is your primary command for building, testing, linting, etc. It performs a check, a build, runs all integration tests, and clippy. It will make you treat warnings as errors, which you should fix.

Don't create one-off test files and scripts. Use the existing testing infrastructure and debug printing.

## Integration Tests

Integration tests are automatically discovered and generated using the `#[rustleaf_tests]` macro in `rustleaf-macros`.

### Test File Naming Conventions

Integration test files use the `.rustleaf` extension and support special suffixes:

- **`_panic.rustleaf`**: Tests that should panic with "Assertion failed"
  - Generates `#[should_panic]` attribute
  - Use for testing assertion failures or expected runtime errors

- **`_ignore.rustleaf`**: Tests that should be ignored by default
  - Generates `#[ignore]` attribute
  - Run with `cargo test -- --ignored`
  - Use for slow tests, known failing tests, or platform-specific tests

- **`.rustleaf`**: Standard tests that should pass
  - No special attributes
  - Run with normal `just test`

### Test Discovery

The macro automatically:
1. Scans the specified directory for `.rustleaf` files
2. Generates test function names by converting path separators to underscores
3. Applies appropriate test attributes based on filename suffixes

### Usage in Test Modules

```rust
#[rustleaf_tests("tests/integration/basic")]
mod basic_tests {}
```

This generates individual test functions for each `.rustleaf` file in the specified directory.


## Docs

There is documentation about development processes in `./docs`:
 - `docs/just.md` is instructional material on writing justfiles. It is *not* documentation about *our* justfile.
 - `docs/testing.md` documents integration test naming conventions and the `_panic`/`_ignore` suffixes.
 - `docs/development.md` contains general development guidelines.