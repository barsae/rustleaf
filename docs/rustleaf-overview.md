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
- `open(path, mode)` - Open file (has .close() method for with statement)
- `create(path)` - Create new file (has .close() method for with statement)

### Module System
```
// File: library/math.rustleaf
pub class Math {
    pub static fn add(a, b) { a + b }
    pub static fn multiply(a, b) { a * b }
    pub static var pi = 3.14159;
}

pub fn sqrt(x) {
    x ** 0.5
}

pub fn square(x) {
    x * x
}

// Private helper - not accessible from other modules
fn validate_positive(x) {
    if x < 0 {
        raise("Value must be positive")
    }
}

// File: main.rustleaf
// Import everything
use library::math::*;
print(Math.add(2, 3))  // 5
print(sqrt(16))  // 4

// Import specific items
use library::math::{Math, sqrt};
print(Math.multiply(4, 5))  // 20
print(sqrt(25))  // 5

// Import with path
use library::math::Math;
print(Math.pi)  // 3.14159

// Nested paths
// File: graphics/shapes/circle.rustleaf
pub class Circle {
    pub var radius;
    
    pub static fn new(r) {
        var c = Circle()
        c.radius = r
        c
    }
    
    pub fn area() {
        3.14159 * self.radius * self.radius
    }
    
    // Private method
    fn validate() {
        if self.radius < 0 {
            raise("Radius cannot be negative")
        }
    }
}

// File: main.rustleaf
use graphics::shapes::circle::Circle;
var c = Circle.new(5)
print(c.area())  // 78.53975

// Multiple imports
use std::io::{read_file, write_file};
use std::collections::{List, Dict};
```

### Visibility Rules
```
// File: example.rustleaf

// Public items - accessible from other modules
pub var API_VERSION = "1.0";
pub fn get_version() { API_VERSION }

pub class Database {
    pub var host;      // Public field
    var connection;    // Private field
    
    pub static fn connect(host) {
        var db = Database()
        db.host = host
        db.connection = establish_connection(host)  // Private function
        db
    }
    
    pub fn query(sql) {
        // Public method
        self.validate_query(sql)
        // ... execute query
    }
    
    fn validate_query(sql) {
        // Private method - only accessible within this module
    }
}

// Private items - only accessible within this module
var cache = {};
fn establish_connection(host) {
    // Private helper function
}

// File: main.rustleaf
use example::{Database, get_version, API_VERSION};

var db = Database.connect("localhost")
db.query("SELECT * FROM users")  // OK - public method
// db.validate_query("...")      // Error - private method
// db.connection                 // Error - private field
print(db.host)                   // OK - public field
print(get_version())             // OK - public function
```

### Attributes

Attributes provide a way to attach metadata to functions, classes, and other items. They can be used for compile-time transformations.

```
// Basic attribute syntax
#[test]
fn test_addition() {
    assert(2 + 2 == 4)
}

#[deprecated("Use new_api instead")]
pub fn old_api() {
    // ...
}

// Multiple attributes
#[async]
#[timeout(5000)]
fn fetch_data(url) {
    // ...
}

// Parameterized attributes
#[route("/api/users")]
#[method("GET")]
fn handle_users_request(req) {
    // ...
}
```

### Custom Attributes

You can define custom attributes that transform the parse tree:

```
// File: attributes/memo.rustleaf

// The #[attribute] marker makes this function an attribute processor
#[attribute]
pub fn memo(ast_node) {
    // ast_node is the parse tree node this attribute is attached to
    if ast_node.type != "function" {
        raise("@memo can only be applied to functions")
    }
    
    // Generate cache storage
    var cache_var = "__memo_cache_" + ast_node.name
    
    // Create a modified copy of the function
    var new_node = ast_node.clone()
    
    // Wrap the original function body
    var original_body = new_node.body
    
    // Create new function body that checks cache
    new_node.body = parse("""
        {
            var key = stringify_args(args)
            if ${cache_var}.has(key) {
                ${cache_var}.get(key)
            } else {
                var result = ${original_body}
                ${cache_var}.set(key, result)
                result
            }
        }
    """)
    
    // Return a block containing cache init and the function
    parse("""
        {
            var ${cache_var} = {};
            ${new_node}
        }
    """)
}

// File: main.rustleaf
use attributes::memo::memo;

#[memo]
fn fibonacci(n) {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

print(fibonacci(40))  // Fast due to memoization
```

### Attribute Processing

```
// Attributes receive the AST node and return a new AST node
// The returned node replaces the original in the parse tree

#[attribute]
pub fn log_calls(ast_node) {
    if ast_node.type != "function" {
        raise("@log_calls can only be applied to functions")
    }
    
    var fn_name = ast_node.name
    var new_node = ast_node.clone()
    
    // Wrap the function body with logging
    var original_body = new_node.body
    new_node.body = parse("""
        {
            print("Calling ${fn_name} with args:", args)
            var result = ${original_body}
            print("${fn_name} returned:", result)
            result
        }
    """)
    
    // Return the modified function
    new_node
}

// Usage
#[log_calls]
fn add(a, b) {
    a + b
}

add(2, 3)
// Output:
// Calling add with args: [2, 3]
// add returned: 5
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
// File: config.rustleaf
pub class WindowConfig {
    pub var width = 800;
    pub var height = 600;
    pub var title = "My App";
}

pub class ThemeConfig {
    pub var background = "#1e1e1e";
    pub var foreground = "#d4d4d4";
    pub var accent = "#569cd6";
}

pub class Config {
    pub var window = WindowConfig();
    pub var theme = ThemeConfig();
}

// Create default config
var config = Config()

// Can override defaults if needed
var custom_config = Config()
custom_config.window.width = 1024
custom_config.theme.accent = "#ff0000"

// File: main.rustleaf
use config::{Config, WindowConfig, ThemeConfig};
var app_config = Config()
app_config.window.title = "My Editor"
```

### Plugin Example
```
// File: plugins/command_palette.rustleaf
pub class CommandPalettePlugin {
    pub var enabled = true;
    var shortcuts = [];  // Private list of shortcuts
    
    pub fn on_startup(editor) {
        if self.enabled {
            editor.set_status("Command palette plugin loaded")
            self.load_shortcuts()
        }
    }
    
    pub fn on_keypress(editor, key) {
        if self.enabled and key == "ctrl+shift+p" {
            editor.show_command_palette()
        }
    }
    
    // Private helper method
    fn load_shortcuts() {
        // Load user-defined shortcuts
        self.shortcuts = ["copy", "paste", "cut"]
    }
}

// File: main.rustleaf
use plugins::command_palette::CommandPalettePlugin;

var plugin = CommandPalettePlugin()
editor.register_plugin(plugin)
```