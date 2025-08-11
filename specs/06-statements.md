# 6. Statements

Statements are syntactic constructs that either execute expressions for their side effects or perform special operations that cannot be expressions. This chapter defines the various statement types in RustLeaf, their syntax, and execution semantics.

### 6.1. Statement Grammar

The statement grammar follows a unified structure where most constructs are expressions, but certain operations are statement-only.

**Statement Syntax:**
```
Statement = Expression ";" Statement?
          | Assignment ";" Statement?
          | Declaration ";" Statement?
          | ControlFlow ";" Statement?
          | Import ";" Statement?
          | Expression  // final expression value (no semicolon)

Assignment = LValue AssignOp Expression
Declaration = "var" Pattern ("=" Expression)?
ControlFlow = ReturnStmt | BreakStmt | ContinueStmt
Import = "use" ModulePath ImportList
```

**Statement Execution Rules:**
- Statements execute in the order they appear
- Statements are separated by semicolons, except for the final expression in a block
- The final expression in a block (without semicolon) becomes the block's value
- Module-level statements execute when the module is loaded
- Control flow statements (break, continue, return) alter execution flow

**Statement Categories:**
1. **Expression statements** - Expressions executed for side effects (followed by semicolon)
2. **Assignment** - Update variables or object properties (statement-only, not expressions)
3. **Declaration** - Introduce new variables (statement-only)
4. **Control flow** - Alter execution flow (statement-only)
5. **Import** - Import modules and symbols (statement-only)

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
- Commonly used for function calls and method calls
- Pure expressions without side effects are allowed but not useful
- Assignment is **not** an expression and therefore cannot appear in expression statements

**Examples:**
```
// Function call statements
print("Hello, World!");
list.append(42);
file.close();

// Method calls
list.sort();
dict.clear();

// Pure expressions (legal but not useful)
42;              // Evaluates to 42, then discards it
x + y;           // Computes sum, then discards it
true and false;  // Evaluates to false, then discards it
```

### 6.3. Declaration Statements

Variables are declared using the `var` keyword, optionally with initialization.

**Syntax:**
```
Declaration = "var" Pattern ("=" Expression)?
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

### 6.4. Assignment Statements

Assignment statements update the value stored in a variable or mutable location.

**Syntax:**
```
Assignment = LValue AssignOp Expression
LValue = Identifier
       | Expression "." Identifier  
       | Expression "[" Expression "]"
AssignOp = "=" | "+=" | "-=" | "*=" | "/=" | "%="
```

**Assignment Rules:**
- Assignment is a statement, not an expression
- Assignment does not return a value
- Left-hand side must be a valid assignment target
- Right-hand side is evaluated before the assignment
- Cannot be used in conditional contexts or embedded in expressions

**Examples:**
```
// Simple assignment
x = 10;
y = "hello";

// Property assignment
obj.field = "value";
point.x = 100;

// Index assignment  
arr[0] = 42;
dict["key"] = "value";

// Compound assignment
count += 1;
total *= 2;
name += " Smith";

// Nested assignment targets
matrix[0][1] = 99;
obj.nested.field = "deep";

// Invalid: assignment is not an expression
// var result = (x = 5);    // Error
// if x = 10 { }            // Error
```

### 6.5. Control Flow Statements

Control flow statements alter the sequential execution of statements.

**Syntax:**
```
ControlFlow = ReturnStmt | BreakStmt | ContinueStmt

ReturnStmt = "return" Expression?
BreakStmt = "break" Expression?
ContinueStmt = "continue"
```

**Control Flow Rules:**
- These constructs are statements only, not expressions
- They immediately alter execution flow
- Cannot be used in expression contexts
- Must appear in appropriate scopes (return in functions, break/continue in loops)

### 6.6. Loop Statements

Loop constructs (`while`, `for`, `loop`) use identical syntax in both statement and expression contexts. When used as statements, any return value is discarded. When used as expressions, they can return values via `break` statements.

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
- When used as a statement (not assigned to variable), the return value is discarded
- See Section 5.14.1 for while expressions that return values

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

**Semantics:**
- Iterates over the collection using iterator protocol
- Pattern binds values on each iteration
- When used as a statement (not assigned to variable), the return value is discarded
- See Section 5.14.2 for for expressions that return values

**Iterator Protocol:**
For statements work with any object that implements the iterator protocol. See Section 12.5 for complete iterator protocol specification, including requirements for `op_iter()` and `op_next()` methods. Iterator completion is detected using `is_unit()` since unit values cannot be used in boolean contexts. Built-in types (list, dict, string) implement this protocol.

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
- Functions that end with a statement rather than an expression
- `return;` statements (return without a value)
- `op_next()` when iteration is complete
- Any control flow that doesn't explicitly produce a value

**Statement vs Expression Semantics:**
- Statements are executed for their side effects and don't produce values
- Expression statements execute an expression for side effects but discard the result
- Functions return the value of their last expression, or unit if they end with a statement

### 6.7. Try-Catch Statements

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
- Try-catch uses identical syntax in both statement and expression forms (see Section 5.8)
- Parser distinguishes based on context (whether result is used)

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

### 6.8. With Statements

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

**Note:** Break, continue, and return statements are now covered in Section 6.5 (Control Flow Statements) since they follow the unified grammar structure.

### 6.9. Import Statements

Import statements bring symbols from other modules into scope.

**Syntax:**
```
Import = "use" ModulePath ImportList
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

