# Add Range Operators as Binary Operators

## Task
Implement range operators (`..` and `..=`) as binary expressions in the RustLeaf parser. The lexer already supports these tokens but they're not implemented in binary expression parsing.

## Context
You are working on a RustLeaf language parser implemented in Rust. The lexer already supports:
- `TokenType::DotDot` (..) for exclusive range
- `TokenType::DotDotEqual` (..=) for inclusive range

**IMPORTANT**: Read the specifications in `./specs/*.md` first - these are the definitive authority on RustLeaf language behavior and all implementation must conform exactly to the specification.

## Current State
- Lexer tokenizes `..` as `TokenType::DotDot`
- Lexer tokenizes `..=` as `TokenType::DotDotEqual`  
- Range operators are NOT in the binary expression constructor mapping
- No precedence defined for range operators

## Requirements
1. **Study Existing Implementation**:
   - Review `src/parser/binary_ops.rs` for binary operator patterns
   - Check `src/core/ast.rs` for `Expression::Binary` variants
   - Look at precedence handling in existing operators

2. **Add Range Operators**:
   - Add `DotDot` and `DotDotEqual` to `get_binary_expression_constructor()`
   - Define appropriate precedence for range operators (typically low precedence)
   - Map tokens to `BinaryOp::Range` or similar enum variants

3. **Update AST if Needed**:
   - Check if `BinaryOp` enum needs new variants for ranges
   - Ensure range expressions are properly represented

4. **Testing**:
   - Create test files in `tests/integration/basic/` for:
     - `range_exclusive_parse.rustleaf`: `1..10`
     - `range_inclusive_parse.rustleaf`: `1..=10`
     - `range_expression_parse.rustleaf`: `(start..end)`
   - Run `just test` to ensure all tests pass

## Files to Modify
- `src/parser/binary_ops.rs` - Add range operator mappings
- `src/core/ast.rs` - Add range binary operation variants if needed
- `tests/integration/basic/*.rustleaf` - Add test files

## Success Criteria
- `1..10` parses as binary expression with exclusive range
- `1..=10` parses as binary expression with inclusive range
- Range operators have appropriate precedence
- All existing tests continue to pass
- New range test cases pass

## Notes
- Range operators typically have lower precedence than arithmetic
- Study existing binary operator implementation patterns
- Follow Rust naming conventions for enum variants
- Consider if ranges need special handling beyond normal binary expressions