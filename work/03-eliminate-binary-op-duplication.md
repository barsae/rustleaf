# Eliminate binary operator code duplication

Remove repeated patterns in binary operator handling across parser and compiler.

Read all relevant code before working.

## Tasks

1. In `src/eval/compiler.rs`, create generic helper:
   ```rust
   fn compile_binary_op(&mut self, left: Expression, right: Expression, method_name: &str) -> Result<Eval> {
       let left_eval = Box::new(self.compile_expression(*left)?);
       let right_eval = Box::new(self.compile_expression(*right)?);
       Ok(Eval::MethodCallOp(EvalMethodCallOp {
           object: left_eval,
           method_name: method_name.to_string(),
           args: vec![right_eval],
       }))
   }
   ```

2. Replace all binary operator compilations (lines 79-149) with calls to the helper

3. Create operator method name mapping:
   ```rust
   impl BinaryOp {
       fn to_method_name(&self) -> &'static str {
           match self {
               BinaryOp::Add => "op_add",
               BinaryOp::Sub => "op_sub",
               // ... etc
           }
       }
   }
   ```

4. Apply similar pattern to assignment operators

5. Run `just test` after each change

## Success Criteria
- Binary operator compilation reduced from 70+ lines to ~10 lines
- No code duplication
- All tests pass
- Single source of truth for operator->method mapping