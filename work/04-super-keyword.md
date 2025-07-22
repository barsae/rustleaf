# Add Super Keyword Parsing as Primary Expression

## Task
Implement `super` keyword parsing as a primary expression in the RustLeaf parser. The lexer already supports this token but it's not parsed as an expression.

## Context
You are working on a RustLeaf language parser implemented in Rust. The lexer already supports:
- `TokenType::Super` for parent class access

**IMPORTANT**: Read the specifications in `./specs/*.md` first - these are the definitive authority on RustLeaf language behavior and all implementation must conform exactly to the specification.

## Current State
- Lexer tokenizes `super` as `TokenType::Super`
- Parser does NOT handle `super` in `parse_primary()` method
- No AST representation for super expressions

## Requirements
1. **Study Existing Implementation**:
   - Review `src/parser/expression.rs` `parse_primary()` method
   - Check `src/core/ast.rs` for `Expression` enum variants
   - Look at how similar keywords are handled (like identifiers)

2. **Add Super Expression**:
   - Add `Super` variant to `Expression` enum in AST
   - Add `super` handling in `parse_primary()` method
   - Follow same pattern as other primary expressions

3. **Update Parser**:
   - In `parse_primary()`, add check for `TokenType::Super`
   - Return `Expression::Super` when super token is found
   - Ensure it works with property access (e.g., `super.method()`)

4. **Testing**:
   - Create test files in `tests/integration/basic/` for:
     - `super_expression_parse.rustleaf`: `super;`
     - `super_property_parse.rustleaf`: `super.method;`
     - `super_method_call_parse.rustleaf`: `super.method();`
   - Run `just test` to ensure all tests pass

## Files to Modify
- `src/core/ast.rs` - Add `Super` variant to `Expression` enum
- `src/parser/expression.rs` - Add super parsing to `parse_primary()`
- `tests/integration/basic/*.rustleaf` - Add test files

## Success Criteria
- `super` parses as primary expression
- `super.method` parses as property access on super
- `super.method()` parses as method call on super
- All existing tests continue to pass
- New super test cases pass

## Notes
- Super should be handled similarly to identifiers in primary expressions
- Follow existing patterns in `parse_primary()` method
- Super typically works with property access and method calls
- Consider if super needs special semantic handling beyond parsing