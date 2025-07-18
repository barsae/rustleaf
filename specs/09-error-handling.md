# 9. Error Handling

RustLeaf provides error handling through errors that can be raised and caught. Any value can be raised as an error, and errors propagate up the call stack until caught. This chapter defines error raising, catching, propagation, and cleanup semantics.

### 9.1. Error Types

Errors in RustLeaf are not a special type—any value can be raised as an error.

**Error Values:**
- Any value can be raised: strings, numbers, objects, etc.
- Most commonly strings or custom objects
- No required error base class or interface
- Error type identified using `type()` function

**Common Error Patterns:**
```
// String errors
raise("File not found")
raise("Invalid argument: expected positive number")

// Object errors
class NetworkError {
    var code;
    var message;
    
    static fn new(code, message) {
        var e = NetworkError()
        e.code = code
        e.message = message
        e
    }
    
    fn op_str() {
        "NetworkError(${self.code}): ${self.message}"
    }
}

raise(NetworkError.new(404, "Resource not found"))

// Even primitives can be errors
raise(42)
raise(null)
raise([1, 2, 3])
```

**Error Information:**
When an error is raised, the runtime captures:
- The error value
- Stack trace at point of raise
- Source location (file, line, column)

### 9.2. Raising Errors

Errors are raised using the `raise` function, which immediately transfers control to the nearest error handler.

**Syntax:**
```
raise(value)
```

**Raise Semantics:**
- Evaluates the argument to get error value
- Captures current stack trace
- Unwinds stack looking for try-catch block
- If no handler found, terminates program
- Never returns to caller

**Examples:**
```
fn divide(a, b) {
    if b == 0 {
        raise("Division by zero")
    }
    a / b
}

fn validate_age(age) {
    if type(age) != "int" {
        raise("Age must be an integer")
    }
    if age < 0 {
        raise("Age cannot be negative")
    }
    if age > 150 {
        raise("Age seems unrealistic")
    }
}

// Conditional raising
fn process_data(data) {
    if not data {
        raise("No data provided")
    }
    
    if type(data) != "list" {
        raise("Data must be a list")
    }
    
    // Process data...
}

// Raising custom errors
class ValidationError {
    var field;
    var value;
    var message;
    
    static fn new(field, value, message) {
        var e = ValidationError()
        e.field = field
        e.value = value
        e.message = message
        e
    }
    
    fn op_str() {
        "ValidationError in ${self.field}: ${self.message}"
    }
}

fn validate_email(email) {
    if not email.contains("@") {
        raise(ValidationError.new("email", email, "Missing @ symbol"))
    }
}
```

### 9.3. Try-Catch Blocks

Try-catch blocks handle errors by providing an alternative execution path when errors occur.

**Syntax:**
```
TryExpression = "try" Block "catch" Pattern Block
```

**Try-Catch Semantics:**
- Try block is executed
- If no error, catch block is skipped
- If error raised, control transfers to catch
- Catch pattern matches against error value
- Try-catch is an expression (returns a value)
- Value is from try block or catch block

**Pattern Matching:**
- Catch uses pattern matching on error
- Can destructure error objects
- Non-matching patterns re-raise error

**Examples:**
```
// Basic try-catch
var result = try {
    risky_operation()
} catch e {
    print("Error occurred: ${e}")
    null  // Default value on error
}

// Pattern matching errors
var data = try {
    parse_json(input)
} catch {type: "SyntaxError", line: l} {
    print("JSON syntax error on line ${l}")
    {}  // Empty dict as fallback
} catch e {
    print("Unexpected error: ${e}")
    raise(e)  // Re-raise
}

// Using error type
fn safe_divide(a, b) {
    try {
        divide(a, b)
    } catch e {
        if type(e) == "string" and e.contains("zero") {
            0  // Return 0 for division by zero
        } else {
            raise(e)  // Re-raise other errors
        }
    }
}

// Nested try-catch
try {
    var conn = connect_database()
    try {
        conn.execute(query)
    } catch e {
        print("Query failed: ${e}")
        rollback(conn)
    }
} catch e {
    print("Connection failed: ${e}")
}
```

### 9.4. Error Propagation

Errors propagate up the call stack until caught or the program terminates.

**Propagation Rules:**
1. Error raised in function
2. Function immediately returns (abnormally)
3. Caller's execution interrupted
4. Stack unwinds to nearest try-catch
5. If none found, program terminates

**Stack Traces:**
- Captured at raise point
- Include function names and line numbers
- Available in error handlers
- Printed on uncaught errors

**Examples:**
```
fn level3() {
    raise("Deep error")
}

fn level2() {
    print("Before level3")
    level3()  // Error propagates here
    print("Never reached")
}

fn level1() {
    try {
        level2()
    } catch e {
        print("Caught at level1: ${e}")
        // Stack trace available here
    }
}

// Selective catching
fn process_file(path) {
    var file = try {
        open(path)
    } catch e {
        // Only catch file errors
        if e.contains("File") or e.contains("Permission") {
            return null  // File not available
        } else {
            raise(e)  // Propagate other errors
        }
    }
    
    // Process file...
}

// Automatic propagation
fn caller() {
    risky_function()  // Error propagates automatically
}

// Re-raising with context
fn wrapper() {
    try {
        dangerous_operation()
    } catch e {
        // Add context and re-raise
        raise("Failed in wrapper: ${e}")
    }
}
```

### 9.5. Error Objects

While any value can be an error, objects provide structured error information.

**Error Object Conventions:**
- No required structure
- Common pattern: include message, code, details
- Use `op_str()` for display formatting
- Type name identifies error category

**Examples:**
```
// Simple error class
class FileError {
    var path;
    var operation;
    var reason;
    
    static fn new(path, operation, reason) {
        var e = FileError()
        e.path = path
        e.operation = operation
        e.reason = reason
        e
    }
    
    fn op_str() {
        "FileError: ${self.operation} failed on '${self.path}': ${self.reason}"
    }
}

// Usage
fn read_config(path) {
    if not exists(path) {
        raise(FileError.new(path, "read", "file not found"))
    }
    
    var content = try {
        read_file(path)
    } catch e {
        raise(FileError.new(path, "read", str(e)))
    }
    
    parse_config(content)
}

// Error with error code
class HttpError {
    var status;
    var message;
    var url;
    
    static fn new(status, message, url) {
        var e = HttpError()
        e.status = status
        e.message = message
        e.url = url
        e
    }
    
    fn op_str() {
        "HttpError ${self.status}: ${self.message} (${self.url})"
    }
}

// Catching specific error types
try {
    fetch_data(url)
} catch e {
    if type(e) == "HttpError" {
        if e.status == 404 {
            return cached_data()  // Use cache for 404
        } else if e.status >= 500 {
            retry_later()  // Server error
        }
    }
    raise(e)  // Re-raise others
}

// Chained errors
class ChainedError {
    var message;
    var cause;
    
    static fn new(message, cause) {
        var e = ChainedError()
        e.message = message
        e.cause = cause
        e
    }
    
    fn op_str() {
        if self.cause {
            "${self.message}\nCaused by: ${self.cause}"
        } else {
            self.message
        }
    }
}

fn high_level_operation() {
    try {
        low_level_operation()
    } catch e {
        raise(ChainedError.new("High-level operation failed", e))
    }
}
```

### Assert Function

The `assert` function provides runtime assertions for debugging and validation.

**Syntax:**
```
assert(condition, message?)
```

**Assert Semantics:**
- Evaluates condition
- If truthy, returns unit
- If falsy, raises error with message
- Message is optional (default describes assertion)

**Examples:**
```
// Basic assertions
assert(x > 0)
assert(type(value) == "int")
assert(list.length > 0, "List cannot be empty")

// In functions
fn sqrt(x) {
    assert(x >= 0, "Cannot take square root of negative number")
    x ** 0.5
}

// Validating invariants
class BankAccount {
    var balance = 0;
    
    fn deposit(amount) {
        assert(amount > 0, "Deposit amount must be positive")
        self.balance += amount
        assert(self.balance >= 0, "Balance invariant violated")
    }
    
    fn withdraw(amount) {
        assert(amount > 0, "Withdrawal amount must be positive")
        assert(amount <= self.balance, "Insufficient funds")
        self.balance -= amount
    }
}

// Complex conditions
fn process_data(data) {
    assert(data, "Data cannot be null")
    assert(type(data) == "list", "Data must be a list")
    assert(data.length > 0, "Data cannot be empty")
    assert(data.all(fn(x) { type(x) == "int" }), "All elements must be integers")
    
    // Process validated data...
}

// Development assertions
fn optimize_path(points) {
    var original_count = points.length
    
    // ... optimization logic ...
    
    assert(points.length <= original_count, "Optimization should not add points")
}
```

### Pattern Match Failures

When pattern matching fails in irrefutable contexts, a runtime error is raised.

**Failure Contexts:**
- Variable declarations with patterns
- Destructuring assignments
- Function parameters with patterns
- For loop bindings

**Error Messages:**
Include helpful information:
- Expected pattern structure
- Actual value structure
- Source location

**Examples:**
```
// Declaration failures
var [x, y] = [1]              // Error: List pattern expected 2 elements, got 1
var {name, age} = {name: "Alice"}  // Error: Dict pattern missing required key 'age'

// Assignment failures
var a, b
[a, b] = [1, 2, 3]           // Error: List pattern expected 2 elements, got 3

// Parameter failures
fn process_pair([x, y]) {
    x + y
}
process_pair([1])            // Error: List pattern expected 2 elements, got 1

// For loop failures
for [x, y] in [[1, 2], [3]] {  // Error on second iteration
    print(x + y)
}

// Safe pattern matching
fn safe_destructure(data) {
    try {
        var [x, y, z] = data
        process_triple(x, y, z)
    } catch e {
        print("Invalid data format: ${e}")
        null
    }
}

// Validating before destructuring
fn extract_point(data) {
    if type(data) == "list" and data.length == 2 {
        var [x, y] = data  // Safe - we checked
        Point.new(x, y)
    } else {
        raise("Expected [x, y] coordinate pair")
    }
}
```

