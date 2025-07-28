# Split eval_structs.rs into modular files

Split the massive `src/eval/eval_structs.rs` file (1259 lines) into smaller, focused modules.

Read all relevant code before working.

## Tasks

1. Create directory structure:
   ```
   src/eval/structs/
   ├── mod.rs
   ├── literals.rs      # EvalLiteral, EvalVariable
   ├── control_flow.rs  # EvalIf, EvalLoop, EvalWhile, EvalFor, EvalBreak, EvalContinue, EvalReturn
   ├── operators.rs     # EvalLogicalAnd, EvalLogicalOr, EvalNot, EvalMethodCallOp
   ├── collections.rs   # EvalList, EvalDict, EvalIndex, EvalSlice
   └── functions.rs     # EvalCall, EvalFunction, EvalLambda, EvalClass
   ```

2. Move each struct to its appropriate file with its impl block

3. Update imports in `src/eval/structs/mod.rs` to re-export all structs

4. Update `src/eval/mod.rs` to import from the new structure

5. Run `just test` after each file move to ensure nothing breaks

6. Delete the original `eval_structs.rs` file once all structs are moved

## Success Criteria
- All tests pass
- No functionality changes
- Each new file is under 300 lines
- Clear module organization