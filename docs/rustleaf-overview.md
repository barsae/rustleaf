# RustLeaf Language Overview

## Overview
This project has a scripting language embedded in it.

## Core Design Principles
- **Dynamic typing** with runtime type checking
- **Brace-based** syntax (no significant whitespace)
- **Expression-oriented** with implicit returns (last expression is the value)
- **Ease of use** prioritized over performance
- **Deep editor integration** with REPL, inline evaluation, and hot reload
- **Extensible** through Rust-implemented types

## Syntax

### Expression-Oriented Design
Almost everything is an expression that produces a value:
- The last expression in a block is its value (no `return` needed)
- `if`, `match`, and `try` are expressions
- Loops can produce values with `break value`
- Blocks `{ ... }` are expressions

```
// Block expression
var x = {
    var temp = calculate()
    temp * 2  // This is the block's value
}

// Everything composes
var result = if condition { 
    try { risky() } catch e { 0 } 
} else { 
    42 
}
```

### Variables
Variables must be explicitly declared with `var`:
```
var x = 42
var name = "Hello"
var items = [1, 2, 3]
```

### Functions
Functions are declared with `fn`. The last expression is automatically returned:
```
fn greet(name) {
    "Hello, " + name  // No return needed
}

// Explicit return for early exit
fn check_positive(n) {
    if n < 0 {
        return null  // Early return
    }
    n * 2  // This is returned if n >= 0
}

// Block expressions
fn add(a, b) { a + b }
```

### Comments
RustLeaf uses Rust-style comments:
```
// This is a single-line comment
var x = 42  // Inline comment

/* This is a
   multi-line comment */
var y = 100
```

### String Interpolation
Strings support `${}` interpolation:
```
var name = "Alice"
var age = 30
var message = "Hello, ${name}! You are ${age} years old."
// Can include expressions
var debug = "Result: ${x + y * 2}"
```

### Control Flow

#### With Statement
The `with` statement ensures resources are properly cleaned up:
```
// Automatic resource cleanup
with file = open("data.txt") {
    var contents = file.read()
    process(contents)
}  // file.close() is automatically called

// Multiple resources
with db = connect_db(), file = open("output.txt") {
    var data = db.query("SELECT * FROM users")
    file.write(data)
}  // Both are closed in reverse order

// With expressions can return values
var data = with file = open("config.json") {
    file.read()  // This is returned
}

// Custom cleanup via close() method
class TempDir {
    var path;
    
    static fn new(path) {
        var t = TempDir()
        t.path = path
        t
    }
    
    fn close() {
        print("Cleaning up ${self.path}")
        remove_dir(self.path)
    }
}

with temp = TempDir.new("/tmp/work") {
    // Use temp directory
}  // temp.close() called automatically
```

#### If Expressions
If is an expression and returns a value:
```
var result = if x > 0 {
    "positive"
} else if x < 0 {
    "negative"
} else {
    "zero"
}

// Can be used inline
var abs = if n < 0 { -n } else { n }

// All branches must return compatible types
var value = if condition {
    42  // Returns number
} else {
    null  // Returns null
}
```

#### Loops
Loops can also produce values with `break`:
```
// While loops
var result = while true {
    if condition {
        break 42  // Break with value
    }
}

// For loops
for item in collection {
    print(item)
}

// For with index
for i, item in enumerate(collection) {
    print(i, item)
}

// Loop expressions
var found = for item in list {
    if item.id == target {
        break item  // Break with value
    }
}
```

### Pattern Matching
Match is also an expression:
```
var description = match value {
    case 0 {
        "zero"
    }
    case 1..10 {
        "single digit"
    }
    case [first, *rest] {
        "list starting with ${first}"
    }
    case {"type": "user", "name": n} {
        "user named ${n}"
    }
    case _ {
        "other"
    }
}

// Match with guards
var category = match n {
    case x if x < 0 {
        "negative"
    }
    case x if x > 100 {
        "large"
    }
    case _ {
        "normal"
    }
}
```

## Type System

### Built-in Types
- **Primitives**: `int`, `float`, `string`, `bool`, `null`
- **Collections**: `list`, `dict`
- **Functions**: First-class function values
- **Classes**: User-defined types with methods and properties
- **RustValue**: Custom types implemented in Rust

### Dynamic Typing
```
var x = 42        // x is an int
x = "hello"       // now x is a string
x = [1, 2, 3]     // now x is a list
```

### Type Checking
Runtime type checking with helpful error messages:
```
fn add(a, b) {
    if type(a) != "int" or type(b) != "int" {
        raise("add expects integers")
    }
    a + b
}
```

## Functional Features

### First-class Functions
```
fn make_adder(n) {
    fn adder(x) {
        x + n  // Implicit return
    }
    adder  // Return the function
}

var add5 = make_adder(5)
print(add5(3))  // 8
```

### Anonymous Functions (Lambdas)
```
var square = fn(x) { x * x }
var nums = [1, 2, 3, 4]
var squared = nums.map(fn(x) { x * x })
```

### Closures
Functions capture variables from enclosing scope:
```
fn counter() {
    var count = 0
    fn() {
        count = count + 1
        count  // Return new count
    }
}

var c = counter()
print(c())  // 1
print(c())  // 2
```

## Classes and Objects

### Class Declaration
```
class Person {
    // Member variables (default to null if not specified)
    var name;
    var age = 0;  // Default value
    
    // Static helper to construct with arguments
    static fn new(name, age) {
        var p = Person()  // Calls default constructor
        p.name = name
        p.age = age
        p  // Return the instance
    }
    
    // Methods
    fn greet() {
        print("Hello, I'm " + self.name)
    }
    
    fn birthday() {
        self.age = self.age + 1
        print("${self.name} is now ${self.age} years old!")
    }
}

// Create instance using static helper
var alice = Person.new("Alice", 30)
alice.greet()  // "Hello, I'm Alice"
alice.birthday()  // "Alice is now 31 years old!"

// Or create with defaults
var bob = Person()
bob.name = "Bob"  // Set properties after construction
bob.greet()  // "Hello, I'm Bob"
```

### Static Methods
```
class Math {
    static fn max(a, b) {
        if a > b { a } else { b }
    }
    
    static fn min(a, b) {
        if a < b { a } else { b }
    }
    
    static fn abs(n) {
        if n < 0 { -n } else { n }
    }
}

// Call static methods without instance
print(Math.max(10, 20))  // 20
print(Math.abs(-42))  // 42
```

### Instance Properties
```
class Point {
    var x = 0;  // Default to origin
    var y = 0;
    var label;  // Defaults to null
    
    static fn new(x, y) {
        var p = Point()
        p.x = x
        p.y = y
        p
    }
    
    fn distance() {
        (self.x ** 2 + self.y ** 2) ** 0.5
    }
}

var p = Point.new(3, 4)
print(p.x)  // 3
print(p.distance())  // 5.0

// Set optional property
p.label = "Point A"
print(p.label)  // "Point A"

// Create point at origin using defaults
var origin = Point()  // x=0, y=0, label=null
print(origin.x)  // 0
print(origin.distance())  // 0.0

// Example with various default values
class Settings {
    var enabled = true;
    var timeout = 30;
    var name;  // null by default
    var tags = [];  // Empty list
    
    static fn defaults() {
        Settings()  // Just return default instance
    }
}

var settings = Settings.defaults()
print(settings.enabled)  // true
print(settings.timeout)  // 30
print(settings.name)  // null
```

## Standard Library

### Built-in Functions
- `print(...)` - Output to console
- `type(value)` - Get type name as string
- `len(collection)` - Get length
- `range(start, end, step)` - Generate number sequence
- `enumerate(collection)` - Get (index, value) pairs
- `raise(message)` - Raise runtime error/exception
- `export(value)` or `export(dict)` - Export values from module
- `import(path)` - Import a module
- `require(path)` - Import and return exported value
- `open(path, mode)` - Open file (has .close() method for with statement)
- `create(path)` - Create new file (has .close() method for with statement)

### Import/Export System
```
// math_utils.rustleaf
class MathUtils {
    static fn add(a, b) { a + b }
    static fn multiply(a, b) { a * b }
}

export(MathUtils)

// main.rustleaf
var MathUtils = require("math_utils.rustleaf")
print(MathUtils.add(2, 3))  // 5
print(MathUtils.multiply(4, 5))  // 20

// Or export individual functions
// utils.rustleaf
fn calculate_area(width, height) {
    width * height
}

export(calculate_area)

// main.rustleaf
var calculate_area = require("utils.rustleaf")
print(calculate_area(10, 20))  // 200
```

### Operators
```
// Arithmetic
+   // Addition/concatenation
-   // Subtraction/negation
*   // Multiplication
/   // Division
%   // Modulo
**  // Exponentiation

// Comparison
==  // Equality
!=  // Inequality
<   // Less than
>   // Greater than
<=  // Less than or equal
>=  // Greater than or equal

// Logical
and // Logical AND
or  // Logical OR
not // Logical NOT

// Bitwise
&   // Bitwise AND
|   // Bitwise OR
^   // Bitwise XOR
~   // Bitwise NOT
<<  // Left shift
>>  // Right shift

// Assignment
=   // Assignment
+=  // Add and assign
-=  // Subtract and assign
*=  // Multiply and assign
/=  // Divide and assign
%=  // Modulo and assign

// Other
in  // Membership test
is  // Identity test
```

### String Methods
- `str.split(delimiter)`
- `str.trim()`
- `str.upper()`, `str.lower()`
- `str.replace(old, new)`
- `str.contains(substring)`

### List Methods
- `list.append(item)`
- `list.extend(other_list)`
- `list.pop(index)`
- `list.insert(index, item)`
- `list.map(fn)`
- `list.filter(fn)`
- `list.reduce(fn, initial)`

### Dict Methods
- `dict.keys()`
- `dict.values()`
- `dict.items()`
- `dict.get(key, default)`
- `dict.pop(key, default)`

## Editor Integration

### REPL Commands
The editor provides an integrated REPL with special commands:
- `:type <expr>` - Show type of expression
- `:time <expr>` - Time execution
- `:clear` - Clear REPL history
- `:reload` - Reload current file

### Inline Evaluation
Code blocks can be evaluated inline with results shown in the editor:
```
var x = 10
var y = 20
x + y  // => 30
```

### Hot Reload
Changes to functions and variables are automatically reloaded in running scripts without losing state.

## Error Handling

### Runtime Errors
```
fn divide(a, b) {
    if b == 0 {
        raise("Division by zero")
    }
    a / b
}
```

### Try/Catch/Finally
Try/catch is also an expression:
```
fn safe_divide(a, b) {
    try {
        a / b  // Last expression is returned
    } catch e {
        print("Error:", e.message)
        null  // Return null on error
    }
}

// Using with for automatic cleanup
fn read_file(path) {
    try {
        with file = open(path) {
            file.read()  // This is returned if successful
        }  // file.close() called automatically
    } catch e {
        raise("Failed to read file: ${e.message}")
    }
}

// With and try/finally both available
fn copy_file(src, dst) {
    with input = open(src), output = create(dst) {
        output.write(input.read())
    }  // Both files closed automatically
}

// Try as expression
var result = try {
    risky_operation()
} catch e {
    default_value
}
```

### Error Objects
```
// Throwing custom errors
fn validate_age(age) {
    if age < 0 {
        raise({
            type: "ValidationError",
            message: "Age cannot be negative",
            value: age
        })
    }
}

// Catching and inspecting
try {
    validate_age(-5)
} catch e {
    print("Error type:", e.type)
    print("Message:", e.message)
    print("Value:", e.value)
}
```

### Error Context
Errors include:
- Line and column information
- Stack trace
- Variable values in scope

## Extensibility

### Rust Integration
The language uses a `Value` enum that can hold either primitive values or boxed `RustValue` trait objects:

```rust
// Rust side - Value enum
enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function(...),
    Object(...),
    Rust(Box<dyn RustValue>),
}

// Custom types implement RustValue trait
trait RustValue {
    fn get_field(&self, name: &str) -> Option<Value>;
    fn set_field(&mut self, name: &str, value: Value) -> Result<(), String>;
    fn call_method(&self, name: &str, args: Vec<Value>) -> Result<Value, String>;
    fn type_name(&self) -> &str;
}
```

### Example Rust Extension
```rust
// Implement a custom Point type
struct Point { x: f64, y: f64 }

impl RustValue for Point {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "x" => Some(Value::Float(self.x)),
            "y" => Some(Value::Float(self.y)),
            _ => None,
        }
    }
    
    fn call_method(&self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "distance" => {
                // Calculate distance from origin
                let dist = (self.x * self.x + self.y * self.y).sqrt();
                Ok(Value::Float(dist))
            }
            _ => Err(format!("Unknown method: {}", name))
        }
    }
    
    fn type_name(&self) -> &str {
        "Point"
    }
}

// Register constructor function
fn point_constructor(x: Value, y: Value) -> Value {
    Value::Rust(Box::new(Point {
        x: x.as_float().unwrap_or(0.0),
        y: y.as_float().unwrap_or(0.0),
    }))
}
```

### Using Rust Types in Scripts
```
// Create instances of Rust types
var p = Point(3.0, 4.0)
print(p.x)  // 3.0
print(p.y)  // 4.0
print(p.distance())  // 5.0

// Type checking
print(type(p))  // "Point"

// Rust values behave like regular values
var points = [Point(0, 0), Point(1, 1)]
for p in points {
    print(p.distance())
}
```

## Examples

### Hello World
```
print("Hello, World!")
```

### Fibonacci
```
fn fib(n) {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

for i in range(0, 10) {
    print(fib(i))
}
```

### Configuration File
```
class WindowConfig {
    var width = 800;
    var height = 600;
    var title = "My App";
}

class ThemeConfig {
    var background = "#1e1e1e";
    var foreground = "#d4d4d4";
    var accent = "#569cd6";
}

class Config {
    var window = WindowConfig();
    var theme = ThemeConfig();
}

// Export for use
var config = Config()
export(config)

// Can override defaults if needed
var custom_config = Config()
custom_config.window.width = 1024
custom_config.theme.accent = "#ff0000"
```

### Plugin Example
```
// plugin.rustleaf
class Plugin {
    fn on_startup(editor) {
        editor.set_status("Plugin loaded")
    }
    
    fn on_keypress(editor, key) {
        if key == "ctrl+shift+p" {
            editor.show_command_palette()
        }
    }
}

// Export plugin instance
var plugin = Plugin()
export(plugin)
```