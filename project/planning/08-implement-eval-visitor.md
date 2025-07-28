# Implement visitor pattern for eval

Create a visitor pattern to reduce complexity in the evaluator.

Read all relevant code before working.

## Tasks

1. Create visitor trait in `src/eval/visitor.rs`:
   ```rust
   pub trait EvalVisitor {
       type Result;

       fn visit_literal(&mut self, lit: &EvalLiteral) -> Self::Result;
       fn visit_variable(&mut self, var: &EvalVariable) -> Self::Result;
       fn visit_list(&mut self, list: &EvalList) -> Self::Result;
       fn visit_dict(&mut self, dict: &EvalDict) -> Self::Result;
       fn visit_if(&mut self, if_expr: &EvalIf) -> Self::Result;
       fn visit_call(&mut self, call: &EvalCall) -> Self::Result;
       // ... methods for each eval type
   }
   ```

2. Add accept method to Eval trait:
   ```rust
   pub trait Eval {
       fn accept<V: EvalVisitor>(&self, visitor: &mut V) -> V::Result;
   }
   ```

3. Implement accept for each eval struct

4. Create `EvaluatorVisitor` that implements `EvalVisitor<Result = EvalResult>`

5. Refactor `Evaluator::eval()` to use visitor pattern

6. Extract common patterns into visitor helper methods

7. Run `just test` after implementing each visitor method

## Success Criteria
- Evaluator logic separated from traversal
- Easier to add new eval types
- Reduced coupling
- All tests pass