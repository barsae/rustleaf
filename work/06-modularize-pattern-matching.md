# Modularize pattern matching

Extract pattern parsing and matching logic into dedicated modules.

Read all relevant code before working.

## Tasks

1. Create pattern module structure:
   ```
   src/patterns/
   ├── mod.rs
   ├── parser.rs    # Pattern parsing logic
   ├── matcher.rs   # Pattern matching logic
   └── types.rs     # Pattern types and traits
   ```

2. Extract pattern parsing from `src/parser/statement.rs`:
   - Move `parse_pattern()` method
   - Move `parse_list_pattern()` method
   - Move `parse_dict_pattern()` method

3. Extract pattern matching from `src/eval/evaluator.rs`:
   - Move `match_pattern()` method
   - Move `match_pattern_matches()` method
   - Unify duplicated logic

4. Create pattern trait:
   ```rust
   trait Pattern {
       fn parse(parser: &mut Parser) -> Result<Self>;
       fn matches(&self, value: &Value, evaluator: &mut Evaluator) -> Result<bool>;
       fn bind(&self, value: &Value, scope: &mut Scope) -> Result<()>;
   }
   ```

5. Update imports in parser and evaluator

6. Run `just test` after each extraction

## Success Criteria
- Pattern logic centralized
- No code duplication
- Easier to add new pattern types
- All tests pass