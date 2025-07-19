# RustLeaf Language Guide for AI Agents

*Version 1.0 - Optimized for AI Agent Learning*

## Overview

RustLeaf is a dynamically-typed, expression-oriented scripting language designed for editor automation and extension. It combines Rust-inspired syntax with dynamic semantics, providing powerful pattern matching, error handling, and integration with Rust code.

**Key Characteristics:**
- **Dynamic typing** with strong runtime type checking
- **Expression-oriented** - most constructs produce values
- **No implicit conversions** - all type conversions must be explicit
- **First-class functions** with lexical closures
- **Pattern matching** for destructuring and control flow
- **Integrated error handling** with try-catch expressions
- **Module system** with explicit visibility control
- **Rust integration** via RustValue trait

## Quick Reference

### Basic Syntax

```rustleaf
// Variables - must be declared with var
var x = 42;
var name = "Alice";
var numbers = [1, 2, 3];
var data = {key: "value", count: 10};

// Functions
fn add(a, b) {
    a + b  // Last expression - no semicolon
}

fn greet(name, greeting = "Hello") {
    print("${greeting}, ${name}!");  // Statement - needs semicolon
}

// Classes and objects
class Point {
    var x = 0;
    var y = 0;
    
    fn distance() {
        (self.x ** 2 + self.y ** 2) ** 0.5  // Last expression - no semicolon
    }
}

var p = Point();
p.x = 3;
p.y = 4;
print(p.distance());  // 5.0
```

### Type System

RustLeaf has these core types:

| Type | Example | Notes |
|------|---------|-------|
| `null` | `null` | Absence of value, falsy |
| `unit` | (no literal) | Returned by statements, use `is_unit()` to check |
| `bool` | `true`, `false` | Only types with truthiness |
| `int` | `42`, `0xFF`, `0b1010` | 64-bit signed integers |
| `float` | `3.14`, `1e10` | IEEE 754 double precision |
| `string` | `"hello"`, `r"raw\string"` | UTF-8 encoded, immutable |
| `list` | `[1, 2, 3]` | Ordered, mutable, heterogeneous |
| `dict` | `{key: value}` | Key-value mapping, preserves insertion order |
| `function` | `fn(x) { x * 2 }` | First-class functions and closures |
| `object` | `MyClass()` | Instance of user-defined class |
| `RustValue` | (from Rust) | Custom types implemented in Rust |

**Critical Rule:** Only `null` and `bool` have truthiness. Using other types in boolean contexts raises an error.

```rustleaf
// Valid
if true { }
if false { }
if null { }  // null is falsy

// Invalid - raises errors
if 0 { }        // Error: int has no truthiness
if "" { }       // Error: string has no truthiness
if [] { }       // Error: list has no truthiness

// Use explicit tests instead
if x == 0 { }
if s.is_empty() { }
if list.len() == 0 { }
```

## Variables and Scope

All variables must be declared with `var` before use:

```rustleaf
var x;           // Initialized to null
var y = 42;      // Initialized to 42

// Destructuring declarations
var [a, b, c] = [1, 2, 3];
var {name, age} = {name: "Bob", age: 30};
var [first, *rest] = [1, 2, 3, 4, 5];

// Scoping rules
{
    var inner = 10;  // Block scope
    print(inner);    // 10 - statement needs semicolon
}
// print(inner);     // Error: inner not in scope

// Assignment (statement, not expression)
x = 20;
obj.field = "value";
list[0] = 42;
count += 1;  // Compound assignment
```

## Expressions vs Statements

RustLeaf distinguishes between expressions (produce values) and statements (perform actions):

```rustleaf
// Expressions - produce values
var result = if x > 0 { "positive" } else { "non-positive" };
var doubled = numbers.map(fn(n) { n * 2 });
var found = match value { case 42 { "answer" } case _ { "other" } };

// Statements - perform actions, end with semicolon
print("Hello");
list.append(42);
x = 10;

// Function bodies - last expression is return value
fn calculate(x) {
    var temp = x * 2;    // Statement
    temp + 1             // Expression - returned (no semicolon)
}

// Empty body returns unit
fn side_effect() {
    print("doing something");  // Statement
}  // Returns unit
```

## Pattern Matching

Powerful pattern matching for destructuring and control flow:

```rustleaf
// Match expressions
var description = match value {
    case 0 { "zero" }
    case 1..=10 { "small positive" }
    case n if n < 0 { "negative: ${n}" }
    case [first, *rest] { "list starting with ${first}" }
    case {type: "user", name: n} { "user named ${n}" }
    case _ { "something else" }
}

// Destructuring in variable declarations
var [x, y] = get_coordinates();
var {name, age} = user_data;

// Pattern matching in function parameters (use destructuring in body)
fn process_point(point) {
    var {x, y} = point;
    print("Point at (${x}, ${y})");
}

// For loops with patterns
for [key, value] in data.items() {
    print("${key}: ${value}");
}
```

## Error Handling

Any value can be an error. Errors propagate until caught:

```rustleaf
// Raising errors
raise("Something went wrong");
raise({type: "ValueError", message: "Invalid input", code: 400});
assert(x > 0, "x must be positive");

// Try-catch expressions
var result = try {
    risky_operation();
    "success"
} catch e {
    print("Error: ${e}");
    "failed"
}

// Pattern matching errors
var data = try {
    parse_json(input)
} catch {type: "SyntaxError"} {
    {}  // Return empty dict for syntax errors
} catch e {
    raise(e);  // Re-raise other errors
}

// Resource management with with statements
with file = open("data.txt") {
    var content = file.read();
    process(content);
}  // file.close() called automatically
```

## Functions and Closures

Functions are first-class values with powerful parameter handling:

```rustleaf
// Basic function
fn add(a, b) { a + b }

// Default parameters (literals only)
fn connect(host, port = 80, secure = false) {
    print("Connecting to ${host}:${port} (secure: ${secure})");
}

// Rest and keyword parameters
fn printf(format, *args, **kwargs) {
    // args is a list, kwargs is a dict
    var formatted = format_string(format, args);
    print(formatted);
}

printf("User %s has %d points", "Alice", 100, debug=true);

// Anonymous functions (lambdas)
var double = fn(x) { x * 2 };
var mapped = numbers.map(fn(n) { n * n });

// Closures capture by reference
fn make_counter() {
    var count = 0;
    fn() {
        count += 1;  // Modifies captured variable
        count
    }
}

var counter = make_counter();
print(counter());  // 1
print(counter());  // 2

// Function calls with spread
var args = [1, 2, 3];
var kwargs = {debug: true, timeout: 30};
some_function(*args, **kwargs);
```

## Built-in Functions and Methods

### Core Functions
```rustleaf
// Type operations
type(value)         // Returns type name as string
str(value)          // Convert to string
int(value)          // Convert to integer
float(value)        // Convert to float
bool(value)         // Convert to boolean (limited types)

// Collections
range(start, end)   // Create range iterator
list(iterable)      // Convert to list
dict(pairs)         // Convert to dict
enumerate(iterable) // Add indices

// Utilities
print(value)        // Output to console
len(collection)     // Get length
abs(number)         // Absolute value
min(a, b, ...)      // Minimum value
max(a, b, ...)      // Maximum value
callable(value)     // Check if callable
is_unit(value)      // Check if unit type
```

### String Methods
```rustleaf
var s = "  Hello, World!  "
s.len()                    // 17
s.trim()                   // "Hello, World!"
s.upper()                  // "  HELLO, WORLD!  "
s.lower()                  // "  hello, world!  "
s.split(", ")              // ["  Hello", "World!  "]
s.replace("Hello", "Hi")   // "  Hi, World!  "
s.contains("World")        // true
s.starts_with("  Hello")   // true
s.ends_with("!  ")         // true
s.is_empty()               // false
```

### List Methods
```rustleaf
var list = [3, 1, 4, 1, 5];
list.append(9);             // Mutates: [3, 1, 4, 1, 5, 9]
list.extend([2, 6]);        // Mutates: [3, 1, 4, 1, 5, 9, 2, 6]
list.insert(0, 0);          // Mutates: [0, 3, 1, 4, 1, 5, 9, 2, 6]
var removed = list.pop();   // Returns 6, list is [0, 3, 1, 4, 1, 5, 9, 2]
list.remove(1);             // Removes first 1: [0, 3, 4, 1, 5, 9, 2]
list.sort();                // Mutates: [0, 1, 2, 3, 4, 5, 9]
list.reverse();             // Mutates: [9, 5, 4, 3, 2, 1, 0]

// Functional methods (return new lists)
var doubled = list.map(fn(x) { x * 2 });
var evens = list.filter(fn(x) { x % 2 == 0 });
var sum = list.reduce(fn(a, b) { a + b }, 0);
```

### Dict Methods
```rustleaf
var d = {a: 1, b: 2};
d.set("c", 3);              // Same as d["c"] = 3
var value = d.get("a");     // 1
var missing = d.get("z", 0); // 0 (default)
var removed = d.pop("b");   // Returns 2, removes key
d.update({d: 4, e: 5});     // Merge other dict
d.clear();                  // Remove all entries

var keys = d.keys();        // List of keys
var values = d.values();    // List of values  
var items = d.items();      // List of [key, value] pairs
var exists = d.has("a");    // Check if key exists
```

## Iteration and Iterator Protocol

RustLeaf uses a consistent iterator protocol:

```rustleaf
// For loops work with any iterator
for item in [1, 2, 3] { print(item); }
for char in "hello" { print(char); }
for key, value in {a: 1, b: 2} { print("${key}: ${value}"); }

// Custom iterator implementation
class Range {
    var start; var end;
    
    fn op_iter() {
        var iter = RangeIterator();
        iter.current = self.start;
        iter.end = self.end;
        iter
    }
}

class RangeIterator {
    var current; var end;
    
    fn op_next() {
        if self.current >= self.end {
            return  // Returns unit - iteration complete
        }
        var value = self.current;
        self.current += 1;
        value
    }
}

// Usage
for i in Range.new(0, 5) {
    print(i);  // 0, 1, 2, 3, 4
}

// Manual iteration
var iter = range(0, 3).op_iter();
while true {
    var value = iter.op_next();
    if is_unit(value) {
        break;  // Iterator exhausted
    }
    print(value);
}
```

## Modules and Imports

File-based module system with explicit visibility:

```rustleaf
// File: utils/math.rustleaf
pub fn add(a, b) { a + b }
pub fn multiply(a, b) { a * b }
fn helper() { }  // Private - not accessible outside

// File: main.rustleaf
use utils::math::{add, multiply}
use utils::math::add as sum  // Rename import
use std::collections::*      // Import all public items

var result = add(2, 3);
```

**Module Rules:**
- One module per `.rustleaf` file
- Private by default, use `pub` for public visibility
- No module caching - fresh imports each run
- Circular dependencies detected at runtime

## Loop Expressions

Loops can return values through break statements:

```rustleaf
// While expression
var found = while i < list.len() {
    if list[i] == target {
        break list[i];  // Return the found value
    }
    i += 1;
}
// found is either the target value or unit

// For expression  
var first_match = for item in collection {
    if item.matches(criteria) {
        break item;
    }
}

// Loop expression (infinite loop)
var exit_code = loop {
    var input = read_line();
    if input == "quit" {
        break 0;  // Normal exit
    }
    if input == "error" {
        break 1;  // Error exit
    }
    process(input);
}

// Check results with is_unit()
if is_unit(found) {
    print("Not found");
} else {
    print("Found: ${found}");
}
```

## Classes and Objects

Simple object-oriented programming with single inheritance:

```rustleaf
class Person {
    var name = null;
    var age = 0;
    
    // Constructor pattern (static method)
    static fn new(name, age) {
        var p = Person();
        p.name = name;
        p.age = age;
        p
    }
    
    fn greet() {
        print("Hi, I'm ${self.name}");
    }
    
    fn birthday() {
        self.age += 1;
        print("${self.name} is now ${self.age}");
    }
}

var person = Person.new("Alice", 30);
person.greet();     // "Hi, I'm Alice"
person.birthday();  // "Alice is now 31"

// Operator overloading
class Vector {
    var x; var y;
    
    fn op_add(other) {
        var v = Vector();
        v.x = self.x + other.x;
        v.y = self.y + other.y;
        v
    }
    
    fn op_str() {
        "Vector(${self.x}, ${self.y})"
    }
}

var v1 = Vector.new(1, 2);
var v2 = Vector.new(3, 4);
var v3 = v1 + v2;  // Uses op_add
print(v3);         // Uses op_str
```

## String Interpolation

Powerful string interpolation with `${}` syntax:

```rustleaf
var name = "Alice"
var age = 30
var score = 95.5

// Basic interpolation
var msg = "Hello, ${name}!"

// Expression interpolation
var status = "User ${name} (age ${age}) scored ${score}%"
var calc = "Result: ${(score / 100) * age}"

// Nested interpolation
var complex = "Status: ${if score > 90 { "excellent" } else { "good" }}"

// Raw strings don't interpolate
var raw = r"This ${name} is literal"  // "This ${name} is literal"

// Escaping
var escaped = "The syntax is \${variable}"  // "The syntax is ${variable}"
```

## Common Patterns

### Safe Dict Access
```rustleaf
// Instead of dict[key] which may error
var value = dict.get(key, default_value);
```

### Optional Chaining Pattern
```rustleaf
// Since no built-in optional chaining, use try-catch
var result = try {
    obj.field.subfield.value
} catch e {
    null  // Safe default
}
```

### Functional Programming
```rustleaf
// Chain functional operations
var result = data
    .filter(fn(item) { item.active })
    .map(fn(item) { item.transform() })
    .reduce(fn(acc, item) { acc + item.value }, 0);

// Higher-order functions
fn compose(f, g) {
    fn(x) { f(g(x)) }
}

var add_one = fn(x) { x + 1 };
var double = fn(x) { x * 2 };
var add_then_double = compose(double, add_one);
```

### Resource Management
```rustleaf
// Always use with for resources
with file = open("data.txt"), 
     conn = connect_database() {
    var data = parse(file.read());
    conn.save(data);
}  // Both file and conn closed automatically

// Multiple cleanup in reverse order
with outer = acquire_outer(),
     inner = acquire_inner() {
    // Use resources
}  // inner.close() called first, then outer.close()
```

## Error Patterns

### Validation Functions
```rustleaf
fn validate_user(data) {
    if not data {
        raise({type: "ValidationError", message: "Data required"});
    }
    if not data.has("name") {
        raise({type: "ValidationError", message: "Name required"});
    }
    if not data.has("email") {
        raise({type: "ValidationError", message: "Email required"});
    }
    return true;
}

// Usage
try {
    validate_user(user_data);
    process_user(user_data);
} catch {type: "ValidationError", message: msg} {
    print("Validation failed: ${msg}");
} catch e {
    print("Unexpected error: ${e}");
}
```

### Retry Patterns
```rustleaf
fn retry(operation, max_attempts = 3) {
    var attempts = 0;
    loop {
        try {
            break operation();  // Success - return result
        } catch e {
            attempts += 1;
            if attempts >= max_attempts {
                raise(e);  // Give up
            }
            print("Attempt ${attempts} failed, retrying...");
        }
    }
}

var result = retry(fn() { unstable_network_call() });
```

## Performance and Best Practices

### Memory Management
- Primitives (`int`, `float`, `string`, `bool`, `null`) are copied by value
- Collections and objects are shared by reference
- Automatic garbage collection handles cleanup
- Use `with` statements for deterministic resource cleanup

### Function Design
- Prefer expressions over statements when possible
- Use pattern matching instead of complex conditionals
- Keep functions small and focused
- Use closures for stateful operations

### Error Handling
- Use structured error objects with `type` field
- Catch specific error types when possible
- Always clean up resources in error cases
- Provide helpful error messages

### Type Safety
- Use explicit type conversions (`str()`, `int()`, etc.)
- Check types with `type()` function when needed
- Use `is_unit()` to check for unit values
- Remember only `bool` and `null` have truthiness

## Integration with Rust

RustLeaf integrates seamlessly with Rust through the RustValue trait:

```rust
// Rust side
struct Point {
    x: f64,
    y: f64,
}

impl RustValue for Point {
    fn type_name(&self) -> &str { "Point" }
    
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "x" => Some(Value::Float(self.x)),
            "y" => Some(Value::Float(self.y)),
            _ => None,
        }
    }
    
    fn call_method(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "distance" => Ok(Value::Float((self.x * self.x + self.y * self.y).sqrt())),
            _ => Err(format!("Unknown method: {}", name)),
        }
    }
}
```

```rustleaf
// RustLeaf side
var point = Point(3.0, 4.0);  // Created from Rust
print(point.x);               // 3.0
print(point.distance());      // 5.0
print(type(point));           // "Point"
```

This guide covers the essential features of RustLeaf for AI agents. The language combines familiar syntax with powerful features like pattern matching, first-class functions, and seamless Rust integration.