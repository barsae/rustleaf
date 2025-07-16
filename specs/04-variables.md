# 4. Variables

Variables in RustLeaf are dynamically typed containers that can hold values of any type. This chapter defines variable declaration, initialization, scope rules, and assignment semantics.

### 4.1. Variable Declarations

Variables must be explicitly declared using the `var` keyword before they can be used.

**Declaration Syntax:**
```
VarDeclaration = "var" Pattern ("=" Expression)?
Pattern = Identifier | ListPattern | DictPattern
```

**Declaration Rules:**
- All variables must be declared with `var` before use
- Using an undeclared variable is a runtime error
- One variable declaration per `var` statement
- Redeclaring a variable in the same scope is an error
- Variable names follow identifier rules (see Section 2.8)

**Examples:**
```
var x           // Declares x, initialized to null
var y = 42      // Declares y, initialized to 42
var name = "Alice"

// Error: Cannot redeclare in same scope
var x = 10      // Error: x already declared

// Error: Cannot use without declaration
z = 100         // Error: z is not declared
```

**Destructuring Declarations:**
Variables can be declared using destructuring patterns:

```
// List destructuring
var [a, b, c] = [1, 2, 3]
print(a)  // 1
print(b)  // 2
print(c)  // 3

// With rest pattern
var [first, *rest] = [1, 2, 3, 4, 5]
print(first)  // 1
print(rest)   // [2, 3, 4, 5]

// Dict destructuring
var {x, y} = {x: 10, y: 20, z: 30}
print(x)  // 10
print(y)  // 20
// z is not extracted

// Nested destructuring
var [[a, b], {x, y}] = [[1, 2], {x: 3, y: 4}]

// With aliases
var {name: userName, age: userAge} = {name: "Alice", age: 30}
print(userName)  // "Alice"
print(userAge)   // 30
```

**Pattern Matching Rules:**
- List patterns must match the exact length unless using rest pattern (*)
- Dict patterns extract only specified keys
- Patterns can be nested arbitrarily
- Non-matching patterns raise runtime errors

### 4.2. Variable Initialization

Variables can be initialized at declaration or default to `null`.

**Initialization Rules:**
- Variables declared without an initializer are set to `null`
- Initializers can be any expression
- Initializer expressions are evaluated at declaration time
- Variables are initialized before they enter scope

**Examples:**
```
var x               // x is null
var y = null        // Explicit null (same as above)
var z = 2 + 3       // z is 5

// Complex initializers
var list = [1, 2, 3].map(fn(x) { x * 2 })
var config = {
    host: "localhost",
    port: 8080,
    debug: true
}

// Initialization order matters
var a = 10
var b = a + 5       // b is 15

// Function calls in initializers
fn getValue() { 42 }
var v = getValue()  // v is 42
```

**Destructuring Initialization:**
When using destructuring patterns, all variables in the pattern are initialized atomically:

```
// All or nothing - if pattern fails, no variables are created
var [x, y] = [1]    // Error: Pattern mismatch

// Safe extraction with defaults
fn safe_extract(data) {
    try {
        var {name, age} = data
        print("Name: ${name}, Age: ${age}")
    } catch e {
        print("Invalid data format")
    }
}
```

### 4.3. Scope and Lifetime

RustLeaf uses lexical block scoping for variables.

**Scope Rules:**
- Variables are scoped to the block where they are declared
- Inner scopes can access outer scope variables
- Inner scopes can shadow outer scope variables
- Variables exist from declaration until end of their scope

**Block Scope:**
```
{
    var x = 10
    print(x)        // 10
}
// print(x)         // Error: x not in scope

if true {
    var y = 20
    print(y)        // 20
}
// print(y)         // Error: y not in scope
```

**Function Scope:**
- Parameters and local variables are scoped to the function body
- Functions can access module-level variables

```
var module_var = "global"

fn example(param) {
    var local = "local"
    print(param)        // Parameter in scope
    print(local)        // Local variable in scope
    print(module_var)   // Module variable accessible
}

// print(param)         // Error: param not in scope
// print(local)         // Error: local not in scope
```

**Shadowing:**
Inner scopes can declare variables with the same name as outer scopes:

```
var x = 10

{
    var x = 20      // Shadows outer x
    print(x)        // 20
    
    {
        var x = 30  // Shadows both outer x's
        print(x)    // 30
    }
    
    print(x)        // 20 (back to this scope's x)
}

print(x)            // 10 (original x)
```

**Closure Capture:**
Functions capture variables from enclosing scopes by reference:

```
fn make_counter() {
    var count = 0
    
    fn increment() {
        count = count + 1   // Captures count by reference
        count
    }
    
    increment
}

var counter1 = make_counter()
var counter2 = make_counter()

print(counter1())   // 1
print(counter1())   // 2
print(counter2())   // 1 (independent count)

// Capturing in loops
var funcs = []
for i in range(0, 3) {
    var j = i       // Create new variable each iteration
    funcs.append(fn() { j })
}

for f in funcs {
    print(f())      // 0, 1, 2
}
```

**Module-Level Variables:**
Variables declared at the top level of a module have module scope:

```
// At module level
var MODULE_CONSTANT = 42
var shared_state = []

fn use_module_vars() {
    print(MODULE_CONSTANT)
    shared_state.append("used")
}

// Accessible from anywhere in the module
class MyClass {
    fn method() {
        print(MODULE_CONSTANT)
    }
}
```

### 4.4. Assignment

Assignment updates the value stored in a variable or a mutable location.

**Assignment Syntax:**
```
Assignment = LValue "=" Expression
           | LValue CompoundOp Expression

LValue = Identifier
       | Expression "." Identifier
       | Expression "[" Expression "]"

CompoundOp = "+=" | "-=" | "*=" | "/=" | "%="
```

**Assignment Semantics:**
- Assignment is a statement, not an expression
- Assignment does not return a value
- Cannot be used in conditional contexts
- Right-associative when chained (though chaining is rare without expression semantics)

**Simple Assignment:**
```
var x = 10
x = 20              // Update x to 20

var y
y = 30              // Initialize previously declared variable

// Error: Assignment is not an expression
// if x = 5 { }     // Error: Assignment in condition
// var z = (x = 10) // Error: Assignment doesn't return value
```

**Property Assignment:**
```
var obj = {name: "Alice", age: 30}
obj.name = "Bob"
obj.age = 31

var point = Point()
point.x = 10
point.y = 20

// Dynamic property access
var key = "name"
obj[key] = "Charlie"
```

**Index Assignment:**
```
var list = [1, 2, 3]
list[0] = 10        // [10, 2, 3]
list[1] = 20        // [10, 20, 3]

var dict = {a: 1, b: 2}
dict["a"] = 10
dict["c"] = 30      // Add new key

// Nested assignment
var nested = [[1, 2], [3, 4]]
nested[0][1] = 99   // [[1, 99], [3, 4]]
```

**Compound Assignment:**
Compound assignment operators combine an operation with assignment:

```
var x = 10
x += 5      // x = x + 5, now 15
x -= 3      // x = x - 3, now 12
x *= 2      // x = x * 2, now 24
x /= 4      // x = x / 4, now 6
x %= 5      // x = x % 5, now 1

// Works with properties and indices
var obj = {count: 0}
obj.count += 1

var list = [10, 20, 30]
list[0] *= 2        // [20, 20, 30]

// Type checking still applies
var s = "hello"
s += " world"       // "hello world"
// s += 42          // Error: Cannot add string and int
```

**Assignment Validation:**
- Left-hand side must be a valid assignment target
- Cannot assign to literals or expressions
- Cannot assign to method calls

```
// Valid assignments
x = 10
obj.field = 20
list[0] = 30
dict["key"] = 40

// Invalid assignments
// 42 = x           // Error: Cannot assign to literal
// x + y = 10       // Error: Cannot assign to expression
// obj.method() = 5 // Error: Cannot assign to method call
// null = 10        // Error: Cannot assign to null
```

**Destructuring Assignment:**
Assignment can use destructuring patterns to update multiple variables:

```
var a, b, c

// List destructuring assignment
[a, b, c] = [1, 2, 3]
print(a)    // 1
print(b)    // 2
print(c)    // 3

// Swapping variables
[a, b] = [b, a]

// Dict destructuring assignment
var x, y
{x, y} = {x: 10, y: 20, z: 30}

// Mixed with declarations
var p
[p, var q] = [100, 200]  // p assigned, q declared and initialized

// With rest pattern
var first, rest
[first, *rest] = [1, 2, 3, 4, 5]
print(first)    // 1
print(rest)     // [2, 3, 4, 5]
```

---

