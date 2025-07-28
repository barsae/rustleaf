# Refactor compile_expression method

Break down the massive `compile_expression()` method in `src/eval/compiler.rs` (283 lines) into smaller, manageable pieces.

Read all relevant code before working.

## Planning
Consider `https://docs.rs/enum_dispatch/latest/enum_dispatch/`

## Tasks

1. Create expression compiler trait:
   ```rust
   trait ExpressionCompiler {
       fn compile(&self, expr: Expression, compiler: &mut Compiler) -> Result<Eval>;
   }
   ```

2. Implement compilers for each expression type:
   - `LiteralCompiler`
   - `BinaryOpCompiler`
   - `UnaryOpCompiler`
   - `CallCompiler`
   - `IndexCompiler`
   - `ControlFlowCompiler`
   - etc.

3. Create a registry to map expression types to compilers:
   ```rust
   struct CompilerRegistry {
       compilers: HashMap<ExpressionType, Box<dyn ExpressionCompiler>>
   }
   ```

4. Replace the giant match statement with registry lookups

5. Extract helper methods for common patterns

6. Run `just test` after each extraction

## Success Criteria
- `compile_expression()` reduced to under 50 lines
- Each compiler handles one expression type
- All tests pass
- Cyclomatic complexity reduced from ~50 to <10