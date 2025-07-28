
# RustLeaf

## Personality
Don't say "you're right" or statements like that to indicate that you understand. Reserve such statements for when you've actually performed an analysis.

Try your hardest to complete the work you are assigned.

## Specs

The specifications live at `./specs/*.md`. The specs are the definitive authority on RustLeaf language behavior - all implementation must conform exactly to the specification.

RULE **CRITICAL**: Using `Any` for downcasting is forbidden. Ask for help instead.

## Development and Testing

`just test` is your primary command for building, testing, linting, etc. It performs a check, a build, runs all integration tests, and clippy. It will make you treat warnings as errors, which you should fix.

Don't create one-off test files and scripts. Use the existing testing infrastructure and debug printing.

## Integration Tests

Integration tests are automatically discovered and generated using the `#[rustleaf_tests]` macro in `rustleaf-macros`.

### Running Tests

Use `just test` to run all integration tests, which builds the project and runs tests with clippy checks.

### Creating New Tests

To create a new integration test:

1. Create a `.md` file in the appropriate `tests/integration/` subdirectory
2. Add a Program section with a rustleaf code block:

```markdown
# Program 🟢
```rustleaf
// Your test code here
assert(1 + 1 == 2);
```
```

Running `just test` will automatically:
- Execute your code and capture results
- Update the file with Output, Result, Lex, Parse, and Eval sections
- Use colored circles: 🟢 for passing tests, 🔴 for failing tests

### Special Test Types

- **`_panic.md`**: Tests expected to panic (generates `#[should_panic]`)
- **`_ignore.md`**: Tests to skip by default (generates `#[ignore]`)

### Test Discovery

The `#[rustleaf_tests]` macro automatically discovers `.md` files and generates individual test functions. Add new test directories to `tests/integration/mod.rs`.