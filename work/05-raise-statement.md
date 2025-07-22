# Add Raise Statement Parsing

## Task
Implement `raise` statement parsing in the RustLeaf parser. The lexer already supports the raise token but there's no statement parsing for raise expressions.

## Context
You are working on a RustLeaf language parser implemented in Rust. The lexer already supports:
- `TokenType::Raise` for exception raising

**IMPORTANT**: Read the specifications in `./specs/*.md` first - these are the definitive authority on RustLeaf language behavior and all implementation must conform exactly to the specification.

## Current State
- Lexer tokenizes `raise` as `TokenType::Raise`
- Parser does NOT handle raise statements in `parse_statement()`
- No AST representation for raise statements

## Requirements
1. **Study Existing Implementation**:
   - Review `src/parser/statement.rs` for statement parsing patterns
   - Check `src/core/ast.rs` for `Statement` enum variants
   - Look at how similar control flow statements are handled (return, break)

2. **Add Raise Statement**:
   - Add `Raise` variant to `Statement` enum in AST
   - Add raise statement parsing in `parse_statement()` method
   - Handle `raise expression;` syntax

3. **Update Parser**:
   - In `parse_statement()`, add try-parse for raise statements
   - Parse optional expression after `raise` keyword
   - Expect semicolon termination
   - Follow existing statement parsing patterns

4. **Testing**:
   - Create test files in `tests/integration/basic/` for:
     - `raise_statement_parse.rustleaf`: `raise error;`
     - `raise_expression_parse.rustleaf`: `raise Exception("message");`
     - `raise_empty_parse.rustleaf`: `raise;` (if supported)
   - **IMPORTANT**: Use `just test` (not `cargo test`) to run tests - this regenerates test cases and ensures proc macros pick up new test files

## Files to Modify
- `src/core/ast.rs` - Add `Raise` variant to `Statement` enum
- `src/parser/statement.rs` - Add raise statement parsing
- `tests/integration/basic/*.rustleaf` - Add test files

## Success Criteria
Use `just test` to run your new integration tests, review the output in the resulting `.parse` file for correctness. This is sufficient testing for now.
- `raise expression;` parses as raise statement
- Raise statements work with any expression type
- Optional expression handling (if bare `raise;` is supported)
- All existing tests continue to pass
- New raise test cases pass

## Notes
- Follow existing control flow statement patterns (return, break, continue)
- Consider if bare `raise;` (re-raise) is supported by the language
- Use try-parse pattern like other statements
- Handle semicolon termination consistently