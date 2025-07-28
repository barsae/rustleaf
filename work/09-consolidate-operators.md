# Consolidate operator implementations

Reduce 400+ lines of repetitive operator code in `src/core/builtin_ops.rs`.

Read all relevant code before working.

## Tasks

1. Create operator traits in `src/core/operators.rs`:
   ```rust
   trait NumericOp {
       fn apply_int(a: i64, b: i64) -> Result<Value>;
       fn apply_float(a: f64, b: f64) -> Result<Value>;
       fn apply_mixed(int: i64, float: f64, int_first: bool) -> Result<Value>;
   }

   trait ComparisonOp {
       fn compare_int(a: i64, b: i64) -> bool;
       fn compare_float(a: f64, b: f64) -> bool;
       fn compare_string(a: &str, b: &str) -> bool;
   }
   ```

2. Implement generic numeric operator handler:
   ```rust
   fn handle_numeric_op<Op: NumericOp>(self_value: &Value, other: &Value) -> Result<Value> {
       match (self_value, other) {
           (Value::Int(a), Value::Int(b)) => Op::apply_int(*a, *b),
           (Value::Float(a), Value::Float(b)) => Op::apply_float(*a, *b),
           (Value::Int(a), Value::Float(b)) => Op::apply_mixed(*a, *b, true),
           (Value::Float(a), Value::Int(b)) => Op::apply_mixed(*b, *a, false),
           _ => Err(anyhow!("Type mismatch"))
       }
   }
   ```

3. Replace repetitive operator implementations with trait impls

4. Extract common patterns for:
   - Division by zero checking
   - Bitwise operations (integer only)
   - Container operations

5. Run `just test` after each operator refactoring

## Success Criteria
- Operator code reduced by 50%+
- No functionality changes
- DRY principle applied
- All tests pass