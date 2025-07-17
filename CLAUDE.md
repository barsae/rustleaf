
# RustLeaf

## Specs

The specifications live at `./specs/*.md`.

## Pre-Commit Verification

Before committing changes, always run these verification steps:

1. **Compile Check**: `RUSTFLAGS="-D warnings" cargo check`
   - Ensures the code compiles without errors or warnings

2. **Run Tests**: `RUSTFLAGS="-D warnings" cargo test`
   - Verifies all tests pass without warnings

3. **Linting**: `cargo clippy -- -D warnings`
   - Checks for common mistakes and style issues
   - **Policy**: All warnings must be fixed before committing

## Post-Commit Formatting

After committing, run formatting as a separate commit:

1. **Format Code**: `cargo fmt`
2. **Check if formatting changed anything**: `git diff`
3. **If there are changes**: `git add -A && git commit -m "Apply cargo fmt"`
