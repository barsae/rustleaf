# Add Macro Parsing Support

## Task
Implement complete macro parsing support for the RustLeaf language parser. Currently macro annotations are parsed but there's no macro expansion or processing system.

## Context
You are working on a RustLeaf language parser implemented in Rust. The lexer already supports:
- `TokenType::Hash` (#) for macro syntax  
- `TokenType::Macro` for the macro keyword
- Basic macro annotation parsing exists in AST

**IMPORTANT**: Read the specifications in `./specs/*.md` first - these are the definitive authority on RustLeaf language behavior and all implementation must conform exactly to the specification.

## Current State
- Lexer tokenizes `#` and `macro` keywords
- AST has `Statement::MacroAnnotation` variant
- No macro expansion or processing during parsing

## Requirements
1. **Understand Current Implementation**:
   - Review `src/core/ast.rs` for macro-related AST nodes
   - Check `src/parser/statement.rs` for existing macro annotation parsing
   - Look at lexer support in `src/lexer/token.rs` and `src/lexer/core.rs`

2. **Implement Macro Parsing**:
   - Parse macro definitions: `macro name(params) { body }`
   - Parse macro invocations: `#macro_name` or `#macro_name(args)`
   - Handle macro expansion during parsing phase
   - Support macro hygiene (avoid name collisions)

3. **Integration**:
   - Update parser to process macros before other parsing
   - Add macro storage/resolution system
   - Handle macro arguments and parameter substitution

4. **Testing**:
   - Create `.rustleaf` test files in `tests/integration/basic/` for:
     - Simple macro definitions
     - Macro invocations
     - Macro expansion with arguments
   - Run `just test` to ensure all tests pass

## Files to Modify
- `src/core/ast.rs` - Add macro AST nodes if needed
- `src/parser/core.rs` - Add macro processing pipeline
- `src/parser/statement.rs` - Extend macro parsing
- `src/parser/expression.rs` - Handle macro expressions
- `tests/integration/basic/*.rustleaf` - Add test files

## Success Criteria
- Macros can be defined with `macro name(params) { body }`
- Macro invocations `#name` and `#name(args)` are parsed and expanded
- All existing tests continue to pass
- New macro test cases pass
- Code follows existing parser patterns and conventions

## Notes
- Study existing parser patterns for consistency
- Use `with_checkpoint` for backtracking if needed
- Macro expansion should happen before normal parsing
- Follow Rust naming conventions and error handling patterns