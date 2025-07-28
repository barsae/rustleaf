# Create error handling abstraction

Implement a proper error context system with file/line tracking and unified error patterns.

Read all relevant code before working.

## Tasks

1. Create error context module in `src/core/error.rs`:
   ```rust
   pub struct ErrorContext {
       file: Option<PathBuf>,
       line: usize,
       column: usize,
       source_line: Option<String>,
   }

   pub struct RustLeafError {
       kind: ErrorKind,
       message: String,
       context: Option<ErrorContext>,
       cause: Option<Box<dyn Error>>,
   }
   ```

2. Create error builder pattern:
   ```rust
   impl RustLeafError {
       pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self
       pub fn with_context(self, context: ErrorContext) -> Self
       pub fn with_cause(self, cause: Box<dyn Error>) -> Self
   }
   ```

3. Replace all `anyhow!` calls with structured errors

4. Add error context to parser and evaluator

5. Implement error formatting with source code snippets

6. Update all error returns to use new system

7. Run `just test` to ensure error handling still works

## Success Criteria
- All errors have proper context
- Error messages show source location
- Consistent error handling throughout codebase
- Better debugging experience