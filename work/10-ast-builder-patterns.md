# Add builder patterns for complex AST nodes

Implement builder patterns for complex AST construction.

Read all relevant code before working.

## Tasks

1. Create builders for complex expressions:
   ```rust
   pub struct CallExpressionBuilder {
       function: Option<Box<Expression>>,
       args: Vec<Expression>,
       kwargs: IndexMap<String, Expression>,
   }

   impl CallExpressionBuilder {
       pub fn new() -> Self
       pub fn function(mut self, func: Expression) -> Self
       pub fn arg(mut self, arg: Expression) -> Self
       pub fn kwarg(mut self, name: String, value: Expression) -> Self
       pub fn build(self) -> Result<Expression>
   }
   ```

2. Add builders for:
   - `FunctionDeclaration`
   - `ClassDeclaration`
   - `MatchExpression`
   - `ImportStatement`

3. Update parser to use builders where appropriate

4. Add convenience methods:
   ```rust
   impl Expression {
       pub fn call() -> CallExpressionBuilder
       pub fn list() -> ListExpressionBuilder
       pub fn dict() -> DictExpressionBuilder
   }
   ```

5. Run `just test` after each builder implementation

## Success Criteria
- Cleaner AST construction code
- Impossible to create invalid AST nodes
- Better API ergonomics
- All tests pass