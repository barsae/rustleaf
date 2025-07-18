# 6. Statements

Statements are instructions that perform actions but do not produce values. This chapter defines the various statement types in RustLeaf, their syntax, and execution semantics. Unlike expressions, statements are executed for their side effects.

### 6.1. Statement Evaluation

Statements are executed sequentially within their containing block or module.

**Statement Execution Rules:**
- Statements execute in the order they appear
- Statements must be separated by semicolons, except for the last statement in a block
- The last statement in a block may omit its semicolon if it's an expression whose value should be returned
- Module-level statements execute when the module is loaded
- Certain statements (break, continue, return) alter control flow

**Statement Categories:**
1. **Expression statements** - Expressions executed for side effects
2. **Variable declarations** - Introduce new variables
3. **Control flow statements** - Alter execution flow
4. **Import statements** - Import modules and symbols
5. **Empty statements** - No operation

**Semicolon Rules:**
```
// Semicolons separate statements
fn example() {
    print("first");   // Semicolon required
    print("second");  // Semicolon required
    print("third")    // No semicolon - this is the return value
}

// Error: missing semicolon
fn bad_example() {
    print("first")    // Parse error: missing semicolon
    print("second")   // Parser stops at previous error
}

// Block with multiple statements
{
    var x = 10;       // Semicolon required
    x = x + 1;        // Semicolon required
    x * 2             // No semicolon - block returns x * 2
}
```

### 6.2. Expression Statements

Any expression can be used as a statement by following it with a semicolon.

**Syntax:**
```
ExpressionStatement = Expression ";"
```

**Expression Statement Rules:**
- The expression is evaluated for its side effects
- The resulting value is discarded
- Commonly used for function calls, assignments, and method calls
- Pure expressions without side effects are allowed but not useful

**Examples:**
```
// Function call statements
print("Hello, World!");
list.append(42);
file.close();

// Assignment statements  
x = 10;
obj.field = "value";
arr[0] = 100;

// Compound assignment
count += 1;
total *= 2;

// Method calls
list.sort();
dict.clear();

// Pure expressions (legal but not useful)
42;              // Evaluates to 42, then discards it
x + y;           // Computes sum, then discards it
true and false;  // Evaluates to false, then discards it
```

### 6.3. Variable Declaration Statements

Variables are declared using the `var` keyword, optionally with initialization.

**Syntax:**
```
VarStatement = "var" Pattern ("=" Expression)? ";"
Pattern = Identifier 
        | ListPattern
        | DictPattern
```

**Declaration Rules:**
- Creates new variables in the current scope
- Uninitialized variables default to `null`
- Destructuring patterns can declare multiple variables
- Pattern must match the initializer structure

**Examples:**
```
// Simple declarations
var x;                    // x is null
var y = 42;              // y is 42
var name = "Alice";      // name is "Alice"

// Destructuring declarations
var [a, b, c] = [1, 2, 3];           // a=1, b=2, c=3
var [first, *rest] = [1, 2, 3, 4];   // first=1, rest=[2,3,4]
var {x, y} = {x: 10, y: 20, z: 30};  // x=10, y=20

// Nested destructuring
var [[a, b], {name}] = [[1, 2], {name: "Bob", age: 30}];

// Pattern mismatch error
var [x, y] = [1];        // Error: Pattern mismatch
var {foo} = {bar: 1};    // Error: Key 'foo' not found
```

### 6.4. Empty Statements

An empty statement performs no operation.

**Syntax:**
```
EmptyStatement = ";"
```

**Uses:**
- Generated code placeholders
- Syntactic requirements
- Explicit "do nothing"

**Examples:**
```
// Empty statement
;

// Empty block serves similar purpose
{ }

// In generated code
if condition {
    ; // Placeholder for future code
}
```

### 6.5. Block Statements

Block statements group multiple statements and create a new scope.

**Syntax:**
```
BlockStatement = "{" Statement* "}"
```

**Block Statement vs Block Expression:**
- Block statement: Used where a statement is expected, no value returned
- Block expression: Used where an expression is expected, returns last expression value

**Examples:**
```
// Block statement in if
if condition {
    var x = 10;
    print(x);
    // No value returned
}

// Block statement standalone
{
    var temp = calculate();
    process(temp);
    cleanup(temp);
}

// Block expression (note no semicolon after block)
var result = {
    var x = 10;
    var y = 20;
    x + y  // This value is returned
}
```

### 6.6. Control Flow Statements

Control flow statements alter the sequential execution of statements.

#### 6.6.1. While Statements

While statements repeatedly execute a block while a condition is true.

**Syntax:**
```
WhileStatement = "while" Expression Block
```

**Semantics:**
- Evaluates condition before each iteration
- Executes block if condition is truthy
- Continues until condition is falsy
- Body may contain break/continue statements
- While loops are statements, not expressions (cannot return values)

**Examples:**
```
// Basic while loop
var i = 0;
while i < 10 {
    print(i);
    i += 1;
}

// While with break
while true {
    var input = read_line();
    if input == "quit" {
        break;
    }
    process(input);
}

// While with continue  
var i = 0;
while i < 100 {
    i += 1;
    if i % 2 == 0 {
        continue;  // Skip even numbers
    }
    print(i);  // Only prints odd numbers
}
```

#### 6.6.2. For Statements

For statements iterate over sequences using the iterator protocol.

**Syntax:**
```
ForStatement = "for" Pattern "in" Expression Block
Pattern = Identifier | ListPattern | DictPattern
```

**Iterator Protocol:**
For statements work with any object that implements the iterator protocol. See Section 12.5 for complete iterator protocol specification, including requirements for `op_iter()` and `op_next()` methods. Built-in types (list, dict, string) implement this protocol.

**Examples:**
```
// Simple iteration
for x in [1, 2, 3, 4, 5] {
    print(x);
}

// String iteration (iterates over characters)
for ch in "hello" {
    print(ch);  // Prints: h, e, l, l, o
}

// Dict iteration (yields [key, value] pairs)
for key, value in {a: 1, b: 2, c: 3} {
    print("${key} = ${value}");
}

// Destructuring in for loop
var points = [[0, 0], [1, 1], [2, 4]];
for [x, y] in points {
    print("Point at (${x}, ${y})");
}

// Using custom iterator
class Range {
    var start;
    var end;
    
    static fn new(start, end) {
        var r = Range();
        r.start = start;
        r.end = end;
        r
    }
    
    fn op_iter() {
        var iter = RangeIterator();
        iter.current = self.start;
        iter.end = self.end;
        iter
    }
}

class RangeIterator {
    var current;
    var end;
    
    fn op_next() {
        if self.current >= self.end {
            return;  // Returns unit - iteration complete
        }
        var value = self.current;
        self.current += 1;
        value
    }
}

for i in Range.new(0, 5) {
    print(i);  // Prints: 0, 1, 2, 3, 4
}
```

**Unit Type:**
The unit type represents "no meaningful value" and is used consistently throughout RustLeaf for:

**Unit Value Properties:**
- Cannot be written as a literal in source code
- Has type name `"unit"` when checked with `type()`
- Can be checked with `is_unit(value)` built-in function
- All unit values are equal to each other

**When Unit is Returned:**
- Functions without explicit return statements
- `return;` statements (return without a value)
- Expression statements (the discarded value is not unit, but statements themselves produce unit)
- `op_next()` when iteration is complete
- Any control flow that doesn't explicitly produce a value

#### 6.6.3. Try-Catch Statements

Try-catch statements handle errors that may occur during execution.

**Syntax:**
```
TryStatement = "try" Block "catch" Pattern Block
Pattern = Identifier | DictPattern
```

**Error Handling:**
- Try block executes normally until an error occurs
- On error, control transfers to catch block
- Catch pattern binds the error value
- No finally clause (use `with` statement for cleanup)
- Try-catch is also an expression form (see Section 5.9)

**Examples:**
```
// Basic error handling
try {
    var result = risky_operation();
    process(result);
} catch e {
    print("Error occurred: ${e}");
}

// Pattern matching on error
try {
    validate_input(data);
} catch {type, message} {
    print("${type} error: ${message}");
}

// Re-raising errors
try {
    database_operation();
} catch e {
    log_error(e);
    raise(e);  // Re-raise the same error
}

// Nested try-catch
try {
    try {
        inner_operation();
    } catch e {
        print("Inner error: ${e}");
        raise("Failed in inner block");
    }
} catch e {
    print("Outer error: ${e}");
}
```

#### 6.6.4. With Statements

With statements ensure resources are properly cleaned up after use.

**Syntax:**
```
WithStatement = "with" ResourceList Block
ResourceList = Resource ("," Resource)*
Resource = Identifier "=" Expression
```

**Resource Management:**
- Each expression is evaluated and bound to its identifier
- Block executes with resources available
- After block (or on error), resources are cleaned up
- Cleanup calls `close()` method on each resource
- Cleanup happens in reverse order of acquisition
- If resource lacks `close()` method, no cleanup occurs

**Examples:**
```
// Single resource
with file = open("data.txt") {
    var contents = file.read();
    process(contents);
}  // file.close() called automatically

// Multiple resources
with input = open("input.txt"), output = create("output.txt") {
    output.write(input.read());
}  // output.close() then input.close() called

// With error handling
try {
    with conn = connect_database() {
        conn.query("SELECT * FROM users");
    }  // conn.close() called even if query fails
} catch e {
    print("Database error: ${e}");
}

// Custom resource with close method
class TempDirectory {
    var path;
    
    static fn new(path) {
        var td = TempDirectory();
        td.path = path;
        create_directory(td.path);
        td
    }
    
    fn close() {
        remove_directory(self.path);
    }
}

with temp = TempDirectory.new("/tmp/work") {
    // Use temporary directory
}  // Directory automatically removed
```

#### 6.6.5. Break Statements

Break statements exit from the nearest enclosing loop.

**Syntax:**
```
BreakStatement = "break" ";"
```

**Break Rules:**
- Must appear within a while or for loop
- Immediately exits the loop
- Execution continues after the loop
- Cannot break from nested loops to outer loops
- Break in statement loops does not return a value

**Examples:**
```
// Break from while
while true {
    var cmd = read_command();
    if cmd == "exit" {
        break;
    }
    execute(cmd);
}

// Break from for
for item in large_list {
    if found_target(item) {
        break;
    }
}

// Break in nested loops (only breaks inner)
for i in range(0, 10) {
    for j in range(0, 10) {
        if matrix[i][j] == target {
            break;  // Only exits inner loop
        }
    }
}

// Error: break outside loop
if condition {
    break;  // Error: break not in loop
}
```

#### 6.6.6. Continue Statements

Continue statements skip to the next iteration of the nearest enclosing loop.

**Syntax:**
```
ContinueStatement = "continue" ";"
```

**Continue Rules:**
- Must appear within a while or for loop
- Skips remaining statements in loop body
- For while: re-evaluates condition
- For for: advances to next iteration
- Cannot continue outer loops from nested loops

**Examples:**
```
// Skip even numbers
for i in range(0, 10) {
    if i % 2 == 0 {
        continue;
    }
    print(i);  // Only prints odd numbers
}

// Continue in while
var i = 0;
while i < 100 {
    i += 1;
    if not is_valid(i) {
        continue;
    }
    process(i);
}

// Error: continue outside loop
fn example() {
    continue;  // Error: continue not in loop
}
```

#### 6.6.7. Return Statements

Return statements exit from the current function with an optional value.

**Syntax:**
```
ReturnStatement = "return" Expression? ";"
```

**Return Rules:**
- Must appear within a function body
- Immediately exits the function
- Without expression, returns unit
- With expression, returns expression value
- Cannot be used at module level

**Unit Return Behavior:**
All functions that don't explicitly return a value will return unit:

```
// Explicit unit return
fn example1() {
    return;  // Explicitly returns unit
}

// Implicit unit return - empty function body
fn example2() {
    // Function body is empty, implicitly returns unit
}

// Implicit unit return - statements only
fn example3() {
    print("hello");  // Statement executed for side effect
    // No explicit return, so implicitly returns unit
}

// Implicit unit return - control flow without value
fn example4(condition) {
    if condition {
        print("true case");
        // No return here
    } else {
        print("false case"); 
        // No return here either
    }
    // Function implicitly returns unit
}

// Check for unit return
var result = example1();
if is_unit(result) {
    print("Function returned unit");  // This will always print
}
```

**Examples:**
```
// Return with value
fn add(a, b) {
    return a + b;
}

// Early return
fn validate(data) {
    if data == null {
        return false;
    }
    if data.length == 0 {
        return false;
    }
    return true;
}

// Return without value
fn print_message(msg) {
    if msg == "" {
        return;  // Returns unit
    }
    print(msg);
    // Implicit return unit
}

// Error: return outside function
var x = 10;
if x > 5 {
    return x;  // Error: return not in function
}
```

### 6.7. Import Statements

Import statements bring symbols from other modules into scope.

**Syntax:**
```
ImportStatement = "use" ModulePath ImportList ";"
ModulePath = Identifier ("::" Identifier)*
ImportList = "::" "*"
           | "::" "{" ImportItem ("," ImportItem)* "}"
           | "::" Identifier

ImportItem = Identifier ("as" Identifier)?
```

**Import Rules:**
- Modules correspond to `.rustleaf` files
- Path separator is `::`
- Can import all public symbols with `*`
- Can import specific symbols with `{...}`
- Can rename imports with `as`
- Imported symbols must be marked `pub` in source module

**Module Resolution:**
- Paths resolve relative to current module's directory
- Use `super::` to access parent directory
- See Chapter 10 for complete module resolution specification

**Examples:**
```
// Import everything from a module
use std::collections::*;

// Import specific items
use std::io::{read_file, write_file};

// Import with renaming
use std::math::{sqrt as square_root, pi};

// Import a single item
use graphics::Color;

// Nested module paths
use game::entities::player::Player;

// Multiple imports from same module
use std::string::{
    split,
    join,
    trim as strip_whitespace
};

// Error: importing private symbols
// In other.rustleaf: fn helper() { }  // Not pub
use other::helper;  // Error: helper is not public
```

**Module Example:**
```
// File: utils/math.rustleaf
pub fn add(a, b) {
    a + b
}

pub fn multiply(a, b) {
    a * b
}

fn internal_helper() {  // Not accessible outside module
    // ...
}

// File: main.rustleaf
use utils::math::{add, multiply};

var sum = add(2, 3);
var product = multiply(4, 5);
```

---

