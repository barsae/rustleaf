# Replace string-based method names with enums

Convert string-based operator method names to type-safe enums.

Read all relevant code before working.

## Tasks

1. Create operator method enum in `src/core/operators.rs`:
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   pub enum OperatorMethod {
       Add,
       Sub,
       Mul,
       Div,
       Mod,
       Pow,
       Neg,
       // ... all operators
   }

   impl OperatorMethod {
       pub fn name(&self) -> &'static str {
           match self {
               Self::Add => "op_add",
               Self::Sub => "op_sub",
               // ... etc
           }
       }
   }
   ```

2. Update `EvalMethodCallOp` to use enum:
   ```rust
   pub struct EvalMethodCallOp {
       pub object: Box<Eval>,
       pub method: OperatorMethod,
       pub args: Vec<Box<Eval>>,
   }
   ```

3. Update compiler to use enums instead of strings

4. Update value attribute lookups to use enums

5. Add conversion methods where needed

6. Run `just test` after each module update

## Success Criteria
- No string literals for operator methods
- Compile-time verification of method names
- Easier refactoring
- All tests pass