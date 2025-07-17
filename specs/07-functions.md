# 7. Functions

Functions are reusable units of code that encapsulate behavior and can accept parameters and return values. RustLeaf treats functions as first-class values that can be assigned, passed, and returned. This chapter defines function declaration, parameters, return values, closures, and execution semantics.

### 7.1. Function Declarations

Functions are declared using the `fn` keyword followed by a name, parameter list, and body.

**Syntax:**
```
FunctionDeclaration = "fn" Identifier "(" ParameterList? ")" Block
ParameterList = Parameter ("," Parameter)*
Parameter = Identifier ("=" Literal)?
          | "*" Identifier
          | "**" Identifier
```

**Declaration Rules:**
- Function names follow identifier rules (see Section 2.8)
- Functions must be declared before use (no hoisting)
- Redeclaring a function in the same scope is an error
- Functions create a new scope for parameters and local variables
- Nested function declarations are allowed and create closures

**Function Types:**
1. **Module-level functions** - Declared at module scope
2. **Nested functions** - Declared inside other functions (closures)
3. **Methods** - Declared inside classes with implicit `self`
4. **Anonymous functions** - Function expressions without names

**Examples:**
```
// Simple function
fn greet(name) {
    print("Hello, ${name}!")
}

// Function with default parameter
fn connect(host, port = 80) {
    print("Connecting to ${host}:${port}")
}

// Function with rest parameters
fn sum(*numbers) {
    var total = 0
    for n in numbers {
        total += n
    }
    total
}

// Function with keyword parameters
fn configure(**options) {
    for key, value in options.items() {
        print("${key} = ${value}")
    }
}

// Nested function (closure)
fn make_counter() {
    var count = 0
    fn increment() {
        count += 1
        count
    }
    increment
}

// Redeclaration error
fn foo() { }
fn foo() { }  // Error: foo already declared
```

### 7.2. Function Parameters

Functions can accept positional parameters, parameters with defaults, rest parameters (`*args`), and keyword parameters (`**kwargs`).

**Parameter Types:**

1. **Required Parameters**: Must be provided by caller
   ```
   fn add(a, b) { a + b }
   add(2, 3)  // OK
   add(2)     // Error: missing parameter b
   ```

2. **Default Parameters**: Have default values if not provided
   ```
   fn greet(name, greeting = "Hello") {
       print("${greeting}, ${name}!")
   }
   greet("Alice")              // "Hello, Alice!"
   greet("Bob", "Hi")          // "Hi, Bob!"
   ```

   **Default Parameter Rules:**
   - Default values must be literals only (no expressions, variables, or function calls)
   - Default values are evaluated fresh on each function call
   - Collection literals (`[]`, `{}`) create new instances on each call
   - For complex defaults, use `null` and compute the value in the function body
   
   **Valid Default Literals:**
   - Numbers: `42`, `3.14`, `-1`
   - Strings: `"hello"`, `''`
   - Booleans: `true`, `false`
   - Null: `null`
   - Empty collections: `[]`, `{}`
   
   **Examples:**
   ```
   fn connect(host, port = 80, secure = false) {
       // Valid: literal defaults
   }
   
   fn process(items = [], options = {}) {
       // Valid: each call gets fresh empty collections
       items.append("processed")
       options.set("processed", true)
   }
   
   // For complex defaults, use null pattern
   fn log_message(msg, timestamp = null) {
       if timestamp == null {
           timestamp = get_current_time()
       }
       print("[${timestamp}] ${msg}")
   }
   
   // Invalid examples
   fn bad_defaults(x, y = x + 1) {
       // Error: expressions not allowed in defaults
   }
   
   fn also_bad(data, processor = get_processor()) {
       // Error: function calls not allowed in defaults
   }
   ```

3. **Rest Parameters** (`*args`): Collect remaining positional arguments
   ```
   fn printf(format, *args) {
       // args is a list of remaining arguments
       print(format_string(format, args))
   }
   printf("Name: %s, Age: %d", "Alice", 30)
   ```

4. **Keyword Parameters** (`**kwargs`): Collect keyword arguments
   ```
   fn create_user(name, **attributes) {
       // attributes is a dict of keyword arguments
       var user = {name: name}
       user.update(attributes)
       user
   }
   var user = create_user("Alice", age=30, email="alice@example.com")
   ```

**Parameter Rules:**
- Parameters are evaluated left-to-right at call time
- Default values are literals only and evaluated fresh on each call
- Parameter order: required, defaults, *args, **kwargs
- Only one `*args` and one `**kwargs` allowed
- Parameters after `*args` are keyword-only
- Default parameters can be skipped by position if followed by keyword arguments

**Parameter Simplicity:**
Parameters are simple identifiers only. For complex data processing, use destructuring in the function body:
```
fn process_point(point) {
    var {x, y} = point
    print("Point at (${x}, ${y})")
}

fn head_tail(list) {
    var [first, *rest] = list
    print("First: ${first}, Rest: ${rest}")
}

process_point({x: 10, y: 20})
head_tail([1, 2, 3, 4])
```

**Examples:**
```
// Mixed parameter types
fn complex_function(required, default = 10, *args, **kwargs) {
    print("Required: ${required}")
    print("Default: ${default}")
    print("Args: ${args}")
    print("Kwargs: ${kwargs}")
}

complex_function(1)                    // required=1, default=10, args=[], kwargs={}
complex_function(1, 2)                 // required=1, default=2, args=[], kwargs={}
complex_function(1, 2, 3, 4)          // required=1, default=2, args=[3,4], kwargs={}
complex_function(1, 2, 3, x=10, y=20) // required=1, default=2, args=[3], kwargs={x:10,y:20}

// Skipping defaults with keyword arguments
fn config(name, debug = false, verbose = false, output = "stdout") {
    print("${name}: debug=${debug}, verbose=${verbose}, output=${output}")
}

config("app")                           // name="app", debug=false, verbose=false, output="stdout"
config("app", true)                     // name="app", debug=true, verbose=false, output="stdout"
config("app", output="file.log")        // name="app", debug=false, verbose=false, output="file.log"
config("app", verbose=true, debug=true) // name="app", debug=true, verbose=true, output="stdout"

// Keyword-only parameters (after *args)
fn format_message(*parts, sep=" ", end="\n") {
    var message = parts.join(sep) + end
    message
}

format_message("Hello", "World")           // "Hello World\n"
format_message("A", "B", "C", sep="-")    // "A-B-C\n"
```

### 7.3. Function Body

The function body is a block that contains the function's implementation.

**Body Semantics:**
- The body is a block expression (see Section 5.10)
- Creates a new scope for local variables
- Has access to parameters and captured variables
- The last expression is the implicit return value
- Empty bodies return `null`

**Local Variables:**
- Must be declared with `var` before use
- Shadow parameters and outer scope variables
- Exist until end of function

**Examples:**
```
// Implicit return
fn double(x) {
    x * 2  // Last expression is returned
}

// Multiple statements
fn calculate(a, b) {
    var sum = a + b
    var product = a * b
    {sum: sum, product: product}  // Return object
}

// Empty body returns null
fn do_nothing() { }
var result = do_nothing()  // result is null

// Conditional return
fn abs(x) {
    if x < 0 {
        -x
    } else {
        x
    }
}
```

### 7.4. Return Values

Functions return values either implicitly (last expression) or explicitly (return statement).

**Return Semantics:**
- Last expression in function body is the return value
- `return` statement exits function immediately with a value
- `return` without expression returns `null`
- All code paths must be reachable or have returns

**Return Statement Syntax:**
```
ReturnStatement = "return" Expression?
```

**Examples:**
```
// Implicit return
fn add(a, b) {
    a + b  // Returned implicitly
}

// Explicit return for early exit
fn find_first(list, predicate) {
    for item in list {
        if predicate(item) {
            return item  // Early return
        }
    }
    null  // Not found
}

// Return without value
fn process_or_exit(data) {
    if not data {
        return  // Returns null
    }
    process(data)
}

// Multiple return paths
fn sign(x) {
    if x > 0 {
        return 1
    } else if x < 0 {
        return -1
    }
    0  // Implicit return for x == 0
}

// Return from nested function
fn outer() {
    fn inner() {
        return 42  // Returns from inner, not outer
    }
    var x = inner()  // x is 42
    x + 8  // Returns 50 from outer
}
```

### 7.5. Closures

Functions can capture variables from enclosing scopes, creating closures.

**Capture Rules:**
- Variables are captured by reference
- Captured variables can be read and modified
- Each closure instance has its own captured environment
- Captured variables exist as long as the closure exists

**Closure Creation:**
- Nested function declarations create closures
- Anonymous functions can capture variables
- Closures are created when the function is defined, not called

**Examples:**
```
// Basic closure
fn make_adder(x) {
    fn add(y) {
        x + y  // Captures x by reference
    }
    add
}

var add5 = make_adder(5)
print(add5(3))  // 8

// Mutable capture
fn make_counter() {
    var count = 0
    fn increment() {
        count += 1  // Modifies captured variable
        count
    }
    increment
}

var c1 = make_counter()
var c2 = make_counter()
print(c1())  // 1
print(c1())  // 2
print(c2())  // 1 (independent counter)

// Multiple closures sharing state
fn make_account(initial) {
    var balance = initial
    
    fn deposit(amount) {
        balance += amount
        balance
    }
    
    fn withdraw(amount) {
        if amount <= balance {
            balance -= amount
            balance
        } else {
            raise("Insufficient funds")
        }
    }
    
    {deposit: deposit, withdraw: withdraw}
}

var account = make_account(100)
account.deposit(50)   // 150
account.withdraw(30)  // 120

// Capturing in loops
fn make_functions() {
    var funcs = []
    for i in range(3) {
        var j = i  // Create new variable for each iteration
        funcs.append(fn() { j })
    }
    funcs
}

var functions = make_functions()
for f in functions {
    print(f())  // 0, 1, 2
}
```

### 7.6. Anonymous Functions

Anonymous functions are function expressions that create function values without declaring a named function.

**Syntax:**
```
AnonymousFunction = "fn" "(" ParameterList? ")" Block
```

**Properties:**
- Same parameter rules as named functions
- Can capture variables (create closures)
- Always require braces for body
- Can be immediately invoked

**Examples:**
```
// Basic anonymous function
var double = fn(x) { x * 2 }
print(double(21))  // 42

// With default parameters
var greet = fn(name, greeting = "Hello") {
    "${greeting}, ${name}!"
}

// Defaults in anonymous functions follow same rules - literals only
var make_id = fn(prefix = "id", start = 0) {
    // All calls start with same literal default
    start += 1
    "${prefix}_${start}"
}

// With *args and **kwargs
var logger = fn(level, *messages, **metadata) {
    print("[${level}] ${messages.join(' ')}")
    for key, value in metadata.items() {
        print("  ${key}: ${value}")
    }
}

logger("INFO", "User", "logged", "in", user_id=123, ip="10.0.0.1")

// Higher-order functions
var numbers = [1, 2, 3, 4, 5]
var squares = numbers.map(fn(x) { x * x })
var evens = numbers.filter(fn(x) { x % 2 == 0 })

// Immediately invoked function expression (IIFE)
var result = fn(a, b) { a + b }(10, 20)  // 30

// Returning functions
fn make_multiplier(factor) {
    fn(x) { x * factor }
}

var triple = make_multiplier(3)
print(triple(10))  // 30
```

### 7.7. Function Scope

Functions create their own scope and follow specific rules for variable resolution.

**Scope Rules:**
1. Parameters are in function scope
2. Local variables must be declared with `var`
3. Functions can access outer scope variables
4. Inner declarations shadow outer ones
5. No hoisting - functions must be declared before use

**Name Resolution Order:**
1. Local variables and parameters
2. Captured variables (for closures)
3. Enclosing function scopes (for nested functions)
4. Module scope
5. Built-in functions

**Examples:**
```
var module_var = "module"

fn outer(param) {
    var outer_var = "outer"
    
    fn middle() {
        var middle_var = "middle"
        
        fn inner() {
            var inner_var = "inner"
            
            // Can access all scopes
            print(inner_var)   // "inner"
            print(middle_var)  // "middle"
            print(outer_var)   // "outer"
            print(param)       // parameter value
            print(module_var)  // "module"
        }
        
        inner()
    }
    
    middle()
}

// Shadowing
var x = 1

fn shadow_example(x) {  // Parameter shadows module x
    var x = 3          // Local shadows parameter
    print(x)           // 3
    
    fn nested() {
        var x = 4      // Shadows outer x
        print(x)       // 4
    }
    
    nested()
    print(x)           // 3 (back to local)
}

shadow_example(2)
print(x)               // 1 (module x unchanged)

// No hoisting
// bad()               // Error: bad not defined
fn good() { print("OK") }
good()                 // OK

fn bad() { print("Too late") }
```

### 7.8. Recursion

Functions can call themselves recursively, subject to stack depth limits.

**Recursion Rules:**
- Direct recursion: function calls itself
- Indirect recursion: function A calls function B which calls function A
- Maximum recursion depth: 1000 calls
- Exceeding limit raises a runtime error
- No tail call optimization

**Stack Depth Limit:**
- The implementation must support at least 1000 recursive calls
- Exceeding the limit must raise a clear error with stack trace
- The limit prevents stack overflow and infinite recursion

**Examples:**
```
// Direct recursion
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

print(factorial(5))  // 120

// Indirect recursion
fn is_even(n) {
    if n == 0 {
        true
    } else {
        is_odd(n - 1)
    }
}

fn is_odd(n) {
    if n == 0 {
        false
    } else {
        is_even(n - 1)
    }
}

print(is_even(4))  // true
print(is_odd(4))   // false

// Tree traversal
fn sum_tree(node) {
    if not node {
        0
    } else {
        node.value + sum_tree(node.left) + sum_tree(node.right)
    }
}

// Stack limit error
fn infinite(n) {
    infinite(n + 1)  // No base case
}

try {
    infinite(0)
} catch e {
    print(e.message)  // "Maximum recursion depth (1000) exceeded"
}

// Fibonacci with memoization (to avoid deep recursion)
var fib_cache = {}

fn fib(n) {
    if n in fib_cache {
        fib_cache[n]
    } else if n <= 1 {
        n
    } else {
        var result = fib(n - 1) + fib(n - 2)
        fib_cache[n] = result
        result
    }
}
```

### Function Call Semantics

When a function is called, the following steps occur:

**Call Process:**
1. Evaluate the function expression
2. Evaluate all arguments left-to-right
3. Match arguments to parameters:
   - Fill required parameters
   - Apply defaults for missing optional parameters
   - Collect extra positional arguments into `*args`
   - Collect keyword arguments into `**kwargs`
4. Create new scope with parameters bound
5. Execute function body
6. Return the result value

**Argument Passing:**
- All values are passed by reference
- Arguments are evaluated before the call
- Spread operators expand collections:
  - `*list` expands to positional arguments
  - `**dict` expands to keyword arguments

**Examples:**
```
// Positional and keyword arguments
fn example(a, b, c = 3, *args, **kwargs) {
    print("a=${a}, b=${b}, c=${c}")
    print("args=${args}")
    print("kwargs=${kwargs}")
}

example(1, 2)                          // a=1, b=2, c=3, args=[], kwargs={}
example(1, 2, 4)                       // a=1, b=2, c=4, args=[], kwargs={}
example(1, 2, 4, 5, 6)                // a=1, b=2, c=4, args=[5,6], kwargs={}
example(1, 2, x=10, y=20)             // a=1, b=2, c=3, args=[], kwargs={x:10,y:20}
example(1, 2, 4, 5, x=10)            // a=1, b=2, c=4, args=[5], kwargs={x:10}

// Spread arguments
var args = [1, 2, 3]
var kwargs = {x: 10, y: 20}

example(*args)                        // a=1, b=2, c=3, args=[], kwargs={}
example(*args, **kwargs)             // a=1, b=2, c=3, args=[], kwargs={x:10,y:20}
example(0, *args, z=30, **kwargs)   // a=0, b=1, c=2, args=[3], kwargs={z:30,x:10,y:20}

// Method calls
class Calculator {
    var value = 0;
    
    fn add(x) {
        self.value += x
        self
    }
    
    fn multiply(x) {
        self.value *= x
        self
    }
}

var calc = Calculator()
calc.add(5).multiply(3).add(7)  // Chaining
print(calc.value)  // 22
```

---

