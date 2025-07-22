# Add String Interpolation Parsing Support

## Task
Implement string interpolation parsing in the RustLeaf parser. The lexer already has `StringPart` token support but the parser doesn't handle `${expr}` syntax within strings.

## Context
You are working on a RustLeaf language parser implemented in Rust. The lexer already supports:
- `TokenType::StringPart` for interpolated string segments
- Basic string literal parsing exists

**IMPORTANT**: Read the specifications in `./specs/*.md` first - these are the definitive authority on RustLeaf language behavior and all implementation must conform exactly to the specification.

## Current State
- Lexer tokenizes string parts with `TokenType::StringPart`
- Parser handles basic string literals but not interpolation
- No AST representation for interpolated expressions within strings

## Requirements
1. **Study Existing Implementation**:
   - Review `src/lexer/core.rs` for string tokenization patterns
   - Check `src/parser/expression.rs` for string literal parsing
   - Look at `src/core/ast.rs` for string representation in AST

2. **Understand Interpolation Syntax**:
   - Determine exact syntax from specs (likely `"text ${expr} more text"`)
   - Check if lexer properly handles `${}` expressions
   - Understand how to parse expressions within string context

3. **Implement Interpolation Parsing**:
   - Extend string literal parsing to handle interpolated expressions
   - Parse embedded expressions between `${` and `}`
   - Create appropriate AST representation for interpolated strings

4. **Update AST if Needed**:
   - May need new `Expression` variant for interpolated strings
   - Consider `InterpolatedString(Vec<StringPart>)` where `StringPart` is enum of text/expression

5. **Testing**:
   - Create test files in `tests/integration/basic/` for:
     - `string_interpolation_simple_parse.rustleaf`: `"Hello ${name}"`
     - `string_interpolation_complex_parse.rustleaf`: `"Value: ${x + y}"`
     - `string_interpolation_multiple_parse.rustleaf`: `"${a} and ${b}"`
   - **IMPORTANT**: Use `just test` (not `cargo test`) to run tests - this regenerates test cases and ensures proc macros pick up new test files

## Files to Modify
- `src/core/ast.rs` - Add interpolated string AST representation
- `src/parser/expression.rs` - Extend string parsing for interpolation
- `src/lexer/core.rs` - Verify/fix string interpolation tokenization
- `tests/integration/basic/*.rustleaf` - Add test files

## Success Criteria
Use `just test` to run your new integration tests, review the output in the resulting `.parse` file for correctness. This is sufficient testing for now.
- `"Hello ${name}"` parses with embedded expression
- `"${expr}"` works with any expression type
- Multiple interpolations in same string work
- All existing tests continue to pass
- New interpolation test cases pass

## Notes
- This may require significant changes to both lexer and parser
- String interpolation is complex - expressions within strings need full parsing
- Consider lexer state management for entering/exiting string context
- Follow existing string literal patterns where possible
- May need recursive expression parsing within string context