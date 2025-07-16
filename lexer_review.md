# RustLeaf Lexer Implementation Fixes

## Task Overview
Fix the discrepancies between the current RustLeaf lexer implementation (`src/lexer.rs`) and the RustLeaf language specification (`docs/rustleaf-spec.md`).

## Issues to Address

### 1. Missing Keyword: `not`
**Problem**: The `not` keyword is defined in the spec but missing from the lexer implementation.

**Required Changes**:
- Add `TokenType::Not` to the enum in `src/lexer.rs:8`
- Add `"not"` keyword mapping in the keywords HashMap in `src/lexer.rs:159`
- Add test case for the `not` keyword in `src/lexer_tests.rs`

**Verification**: Ensure `not` is correctly tokenized as a keyword, not an identifier.

### 2. Float Literal Parsing Bug
**Problem**: The implementation incorrectly handles float literals ending with a dot (e.g., `42.`).

**Current Issue**: In `src/lexer.rs:750`, the condition for detecting floats requires digits after the decimal point, which conflicts with the spec that allows `42.` as a valid float.

**Required Changes**:
- Fix the float detection logic to properly handle `42.` pattern
- Ensure scientific notation detection works correctly for both `42.e10` and `42e10`
- Update the `scan_number` method to correctly identify when a dot should be treated as part of a float vs. a separate dot token

**Test Cases to Add**:
```rust
// Should all parse as float literals
"42."       // -> 42.0
"42.e10"    // -> 42e10
"0."        // -> 0.0
```

### 3. Enhanced Error Handling (Optional - Lower Priority)
**Spec Requirements**:
- File size warnings for files >10MB
- Error handling for files >100MB
- Explicit UTF-8 validation errors

**Implementation Notes**:
- These are lower priority since they don't affect basic lexer functionality
- Consider adding file size checking in the `Lexer::new()` method
- UTF-8 validation may be handled sufficiently by Rust's string handling

## Implementation Guidelines

### Testing Requirements
- All changes must pass existing tests
- Add specific test cases for the `not` keyword
- Add comprehensive float literal tests covering edge cases
- Ensure the test in `src/lexer_tests.rs:383` (exhaustive keyword test) passes with `not` included

### Code Style
- Follow existing code patterns and naming conventions
- Maintain the same error handling approach
- Keep the same token structure and API

### Files to Modify
1. `src/lexer.rs` - Main lexer implementation
2. `src/lexer_tests.rs` - Test cases
3. Potentially `src/lib.rs` if exports need updating

## Verification Steps
1. Run `cargo test` to ensure all tests pass
2. Verify the exhaustive keyword test includes `not`
3. Test float literal parsing with edge cases
4. Ensure existing functionality remains unchanged

## Success Criteria
- [ ] `not` keyword correctly tokenized
- [ ] Float literals ending with dot parse correctly  
- [ ] All existing tests continue to pass
- [ ] New test cases demonstrate fixed functionality
- [ ] No regressions in lexer performance or accuracy

## Context
The lexer is well-implemented overall with proper handling of:
- All other keywords and operators
- String literals (regular, triple-quoted, raw)
- Numeric literals (all bases with underscores)
- Comments (single-line, multi-line, nested, doc)
- Position tracking and error reporting

These fixes will bring the implementation into full compliance with the RustLeaf specification.