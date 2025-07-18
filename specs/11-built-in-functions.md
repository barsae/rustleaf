# 11. Built-in Functions

RustLeaf provides a comprehensive set of built-in functions available globally without import. These functions cover type introspection, collection operations, string manipulation, error handling, and utility operations. This chapter defines all built-in functions, their signatures, behavior, and usage patterns.

### 11.1. Type Functions

Functions for type introspection and conversion.

**type(value) → string**
Returns the type name of a value as a string.

```
type(42)           // "int"
type(3.14)         // "float"  
type("hello")      // "string"
type(true)         // "bool"
type(null)         // "null"
type([1, 2, 3])    // "list"
type({a: 1})       // "dict"
type(fn() {})      // "function"
```

**str(value) → string**
Converts any value to its string representation.

```
str(42)            // "42"
str(3.14)          // "3.14"
str(true)          // "true"
str(null)          // "null"
str([1, 2, 3])     // "[1, 2, 3]"
str({a: 1, b: 2})  // "{a: 1, b: 2}"

// Uses op_str() method if available
class Point {
    var x, y;
    fn op_str() { "(${self.x}, ${self.y})" }
}
str(Point.new(1, 2))  // "(1, 2)"
```

**int(value) → int**
Converts a value to an integer.

```
int("42")          // 42
int(3.14)          // 3 (truncated)
int(true)          // 1
int(false)         // 0
int(null)          // Error: Cannot convert null to int

// String parsing
int("123")         // 123
int("0xFF")        // 255 (hex)
int("0b1010")      // 10 (binary)
int("0o77")        // 63 (octal)
int("invalid")     // Error: Invalid integer format
```

**float(value) → float**
Converts a value to a floating-point number.

```
float("3.14")      // 3.14
float(42)          // 42.0
float(true)        // 1.0
float(false)       // 0.0
float(null)        // Error: Cannot convert null to float

// Scientific notation
float("1.23e4")    // 12300.0
float("5e-3")      // 0.005
```

**bool(value) → bool**
Converts a value to a boolean using truthiness rules.

```
bool(true)         // true
bool(false)        // false
bool(null)         // false
bool(0)            // Error: Only null and bool have truthiness
bool("")           // Error: Only null and bool have truthiness
bool([])           // Error: Only null and bool have truthiness

// Only null and bool values can be converted
// For other types, use explicit comparisons:
// if list.is_empty() { } instead of if bool(list) { }
```

### 11.2. Collection Functions

Functions for working with collections and iterables.

**range(start, end, step=1) → list**
Generates a list of integers from start (inclusive) to end (exclusive).

```
range(0, 5)        // [0, 1, 2, 3, 4]
range(1, 10, 2)    // [1, 3, 5, 7, 9]
range(10, 0, -1)   // [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
range(5, 5)        // [] (empty range)

// Single argument form (start from 0)
range(3)           // [0, 1, 2]
```

**enumerate(iterable, start=0) → list**
Returns a list of [index, value] pairs.

```
enumerate(["a", "b", "c"])           // [[0, "a"], [1, "b"], [2, "c"]]
enumerate(["x", "y"], 1)             // [[1, "x"], [2, "y"]]

// Usage in loops
for i, item in enumerate(["a", "b"]) {
    print("${i}: ${item}")
}
// Output: "0: a", "1: b"
```

**list(iterable) → list**
Converts an iterable to a list.

```
list("hello")      // ["h", "e", "l", "l", "o"]
list(range(3))     // [0, 1, 2]
list({a: 1, b: 2}.keys())  // ["a", "b"]

// Copy a list
var original = [1, 2, 3]
var copy = list(original)  // [1, 2, 3] (new list)
```

**dict(iterable) → dict**
Converts an iterable of key-value pairs to a dictionary.

```
dict([["a", 1], ["b", 2]])           // {a: 1, b: 2}
dict(enumerate(["x", "y"]))          // {0: "x", 1: "y"}

// Copy a dict
var original = {a: 1, b: 2}
var copy = dict(original.items())    // {a: 1, b: 2} (new dict)
```

### 11.3. String Functions

Functions for string operations and formatting.

**print(...values) → null**
Outputs values to console, separated by spaces.

```
print("Hello")                       // "Hello"
print("Result:", 42)                 // "Result: 42"  
print(1, 2, 3)                       // "1 2 3"
print()                              // "" (empty line)

// Automatically converts values to strings
print([1, 2], {a: 3})               // "[1, 2] {a: 3}"
```

### 11.4. Error Functions

Functions for error handling and debugging.

**raise(error) → never**
Raises an error with the given value.

```
raise("Something went wrong")
raise(404)
raise({type: "ValueError", message: "Invalid input"})

// Custom error objects
class CustomError {
    var message;
    static fn new(msg) {
        var e = CustomError()
        e.message = msg
        e
    }
    fn op_str() { "CustomError: ${self.message}" }
}

raise(CustomError.new("Operation failed"))
```

**assert(condition, message="Assertion failed") → unit**
Raises an error if condition is false.

```
assert(x > 0)                        // Error if x <= 0
assert(x > 0, "x must be positive")  // Custom message

// Only works with bool values (strict truthiness)
assert(true)                         // OK
assert(false)                        // Error: Assertion failed
assert(null)                         // Error: Assertion failed
// assert(x)                         // Error: assert requires bool
```

### 11.5. Utility Functions

General utility functions for common operations.

**abs(number) → number**
Returns the absolute value of a number.

```
abs(-5)            // 5
abs(3.14)          // 3.14
abs(0)             // 0
abs(-0.5)          // 0.5
```

**min(...values) → value**
Returns the smallest value from the arguments.

```
min(1, 2, 3)       // 1
min(3.14, 2.71)    // 2.71
min(-5, 0, 10)     // -5

// Works with any comparable types
min("apple", "banana", "cherry")     // "apple"
```

**max(...values) → value**
Returns the largest value from the arguments.

```
max(1, 2, 3)       // 3
max(3.14, 2.71)    // 3.14
max(-5, 0, 10)     // 10

// Works with any comparable types  
max("apple", "banana", "cherry")     // "cherry"
```

**callable(value) → bool**
Tests whether a value can be called as a function.

```
callable(print)              // true
callable(fn() {})            // true
callable("hello")            // false
callable(42)                 // false

// Test before calling
var handler = get_handler()
if callable(handler) {
    handler()
} else {
    print("Handler is not callable")
}
```

**is_unit(value) → bool**
Tests whether a value is the unit type.

```
is_unit(unit_value)          // true (if value is unit)
is_unit(null)                // false
is_unit(42)                  // false
is_unit("hello")             // false

// Check function returns
fn void_function() {
    print("side effect")
}

if is_unit(void_function()) {
    print("Function returned unit")
}

// Check iterator completion
var iter = some_iterator.op_iter()
var next_value = iter.op_next()
if is_unit(next_value) {
    print("Iterator is exhausted")
}
```

### 11.6. Global Availability

All built-in functions are available in the global namespace without import.

**Namespace Rules:**
- Built-ins cannot be shadowed by user definitions in global scope
- Built-ins can be shadowed in local scopes
- Built-ins are always accessible via their original names

**Shadowing Examples:**
```
// Global scope - cannot shadow built-ins
var print = "not a function"  // Error: Cannot redefine built-in 'print'

// Local scope - can shadow
fn test() {
    var print = "local variable"  // OK - local scope
    print  // Returns "local variable"
}

fn demo() {
    var abs = fn(x) { x * x }     // OK - local scope  
    abs(-3)                       // 9 (uses local function)
    
    // Access original built-in explicitly if needed
    // (implementation-specific mechanism)
}
```

**Error Handling:**
All built-in functions provide clear error messages:
```
int("invalid")
// Error: Cannot convert "invalid" to integer
//   at int() (built-in)
//   at main.rustleaf:5:10

type()
// Error: type() requires exactly 1 argument, got 0
//   at type() (built-in)  
//   at main.rustleaf:3:5
```

**Performance Notes:**
- Built-in functions are implemented natively for optimal performance
- Type checking is performed at runtime with helpful error messages
- Built-ins integrate seamlessly with RustLeaf's operator overloading

**Complete Built-in Function List:**

**Type Functions:**
- `type(value)` - Get type name
- `str(value)` - Convert to string  
- `int(value)` - Convert to integer
- `float(value)` - Convert to float
- `bool(value)` - Convert to boolean

**Collection Functions:**
- `range(start, end, step=1)` - Generate integer sequence
- `enumerate(iterable, start=0)` - Create indexed pairs
- `list(iterable)` - Convert to list
- `dict(iterable)` - Convert to dictionary

**String Functions:**
- `print(...values)` - Output to console

**Error Functions:**
- `raise(error)` - Raise an error
- `assert(condition, message="Assertion failed")` - Assert condition

**Utility Functions:**
- `abs(number)` - Absolute value
- `min(...values)` - Minimum value
- `max(...values)` - Maximum value  
- `callable(value)` - Test if callable
- `is_unit(value)` - Test if value is unit type

