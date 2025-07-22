# Add In and Is Operators as Binary Expressions

## Task
Implement `in` and `is` operators as binary expressions in the RustLeaf parser. The lexer already supports these tokens but they're not implemented in binary expression parsing.

## Context
You are working on a RustLeaf language parser implemented in Rust. The lexer already supports:
- `TokenType::In` for membership testing
- `TokenType::Is` for identity comparison

**IMPORTANT**: Read the specifications in `./specs/*.md` first - these are the definitive authority on RustLeaf language behavior and all implementation must conform exactly to the specification.

## Current State
- Lexer tokenizes `in` as `TokenType::In`
- Lexer tokenizes `is` as `TokenType::Is`
- These operators are NOT in the binary expression constructor mapping
- No precedence defined for these operators

## Requirements
1. **Study Existing Implementation**:
   - Review `src/parser/binary_ops.rs` for binary operator patterns
   - Check `src/core/ast.rs` for `Expression::Binary` and `BinaryOp` definitions
   - Look at comparison operator precedence (likely similar)

2. **Add In/Is Operators**:
   - Add `In` and `Is` to `get_binary_expression_constructor()`
   - Define appropriate precedence (typically same as comparison operators)
   - Map tokens to appropriate `BinaryOp` enum variants

3. **Update AST if Needed**:
   - Check if `BinaryOp` enum needs new `In` and `Is` variants
   - Ensure these expressions are properly represented

4. **Testing**:
   - Create test files in `tests/integration/basic/` for:
     - `in_operator_parse.rustleaf`: `item in collection`
     - `is_operator_parse.rustleaf`: `a is b`
     - `is_not_parse.rustleaf`: `a is not b` (if supported)
   - Run `just test` to ensure all tests pass

## Files to Modify
- `src/parser/binary_ops.rs` - Add in/is operator mappings
- `src/core/ast.rs` - Add binary operation variants if needed
- `tests/integration/basic/*.rustleaf` - Add test files

## Success Criteria
- `item in collection` parses as binary expression
- `a is b` parses as binary expression
- Operators have appropriate precedence (similar to comparisons)
- All existing tests continue to pass
- New in/is test cases pass

## Notes
- `in` and `is` typically have same precedence as comparison operators
- Study existing comparison operator implementation
- Consider if `is not` should be handled as a special case
- Follow existing patterns for binary expression parsing