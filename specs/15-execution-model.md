# 15. Execution Model

RustLeaf follows a predictable execution model with well-defined evaluation order, error handling, and context management. This chapter specifies how programs execute, expressions evaluate, and errors propagate through the system.

### 15.1. Program Execution

RustLeaf programs execute from top-level code without requiring a special entry point function.

**Top-level Execution:**
Programs start executing from the first statement in the main file.

```
// This executes immediately when the program runs
print("Program starting...")

var config = load_config()
print("Config loaded: ${config}")

// Functions are defined but not executed
fn main_logic() {
    print("Running main logic")
}

// This executes immediately
main_logic()

print("Program complete")
```

**Module Execution:**
Modules execute their top-level code when first imported.

```
// File: utils.rustleaf
print("Loading utils module...")

var UTILS_VERSION = "1.0"

fn helper() {
    "helper function"
}

print("Utils module loaded")

// File: main.rustleaf
print("Starting main")
use utils  // Triggers execution of utils.rustleaf
           // Output: "Loading utils module..."
           //         "Utils module loaded"
print("Utils imported")
```

**Execution Phases:**
1. **Parse phase**: Source code is parsed into AST
2. **Import phase**: Module dependencies are loaded and executed
3. **Execution phase**: Top-level statements execute in order
4. **Cleanup phase**: Resources are cleaned up when program exits

### 15.2. Expression Evaluation Order

Expressions are evaluated in a predictable left-to-right order.

**Left-to-Right Evaluation:**
All expressions evaluate their operands from left to right.

```
// Function calls evaluated left to right
result = first() + second() + third()
// Execution order: first(), then second(), then third()

// Property access evaluated left to right
value = obj.get_property().process().finalize()
// Execution order: obj.get_property(), then .process(), then .finalize()

// Array access evaluated left to right
item = get_array()[get_index()]
// Execution order: get_array(), then get_index()
```

**Function Argument Evaluation:**
Arguments are evaluated left to right before the function is called.

```
fn log_and_return(name, value) {
    print("Evaluating: ${name}")
    value
}

// Arguments evaluated left to right
result = combine(
    log_and_return("first", 1),   // Executes first
    log_and_return("second", 2),  // Executes second
    log_and_return("third", 3)    // Executes third
)
// Output: "Evaluating: first"
//         "Evaluating: second" 
//         "Evaluating: third"
// Then combine() is called with (1, 2, 3)
```

**Assignment Evaluation Order:**
In assignments, the right-hand side is evaluated before the left-hand side.

```
// RHS evaluated first, then LHS
get_object().field = compute_value()
// Execution order: compute_value(), then get_object(), then assignment

// Complex assignment
items[get_index()] = process_data()
// Execution order: process_data(), then get_index(), then items lookup, then assignment
```

**Short-Circuit Evaluation:**
Logical operators `and` and `or` use short-circuit evaluation.

```
// Short-circuit AND
if expensive_check() and quick_check() {
    // quick_check() only runs if expensive_check() returns true
}

// Short-circuit OR
result = cached_value() or compute_expensive()
// compute_expensive() only runs if cached_value() returns null or false

// Practical example
if user != null and user.is_admin() {
    // user.is_admin() only called if user is not null
    admin_action()
}
```

**Method Chaining Evaluation:**
Method chains evaluate left to right.

```
result = data
    .filter(|x| x > 0)     // First
    .map(|x| x * 2)        // Second  
    .reduce(|a, b| a + b)  // Third

// Equivalent to:
var temp1 = data.filter(|x| x > 0)
var temp2 = temp1.map(|x| x * 2)
var result = temp2.reduce(|a, b| a + b)
```

### 15.3. Function Call Semantics

Function calls follow consistent semantics for parameter passing, execution context, and return values.

**Parameter Passing:**
Parameters are passed according to their type's semantics (value or reference).

```
fn modify_data(num, list, obj) {
    num = 999        // Only affects local copy
    list.append(4)   // Modifies shared list
    obj.field = "new" // Modifies shared object
}

var number = 42
var array = [1, 2, 3]
var object = {field: "old"}

modify_data(number, array, object)

print(number)      // 42 (unchanged - copy semantics)
print(array)       // [1, 2, 3, 4] (modified - reference semantics)
print(object.field) // "new" (modified - reference semantics)
```

**Variable Arguments:**
Functions with *args and **kwargs handle variable arguments.

```
fn variadic_function(required, optional = "default", *args, **kwargs) {
    print("Required: ${required}")
    print("Optional: ${optional}")
    print("Args: ${args}")
    print("Kwargs: ${kwargs}")
}

variadic_function("hello", "world", 1, 2, 3, name: "test", flag: true)
// Output: Required: hello
//         Optional: world
//         Args: [1, 2, 3]
//         Kwargs: {name: "test", flag: true}
```

**Return Value Semantics:**
Functions return the last expression value, or unit if no explicit return.

```
fn explicit_return() {
    if condition {
        return "early"  // Explicit return
    }
    "normal"           // Implicit return
}

fn no_return() {
    print("side effect")
    // Implicitly returns unit
}

var result1 = explicit_return()  // "early" or "normal"
var result2 = no_return()        // unit
```

**Closure Capture:**
Closures capture variables by reference from their lexical scope.

```
fn create_counter(start) {
    var count = start
    
    fn increment() {
        count = count + 1  // Captures 'count' by reference
        count
    }
    
    increment  // Return the closure
}

var counter1 = create_counter(0)
var counter2 = create_counter(10)

print(counter1())  // 1
print(counter1())  // 2
print(counter2())  // 11
print(counter1())  // 3 (independent state)
```

**Recursion Limits:**
Recursion is limited to prevent stack overflow.

```
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)  // Recursive call
    }
}

factorial(1000)   // Works fine
factorial(1001)   // Error: Maximum recursion depth (1000) exceeded
```

### 15.4. Error Handling

Errors propagate up the call stack until caught or the program terminates.

**Error Propagation:**
Uncaught errors unwind the stack and terminate the program.

```
fn level3() {
    raise("Error in level3")
}

fn level2() {
    level3()  // Error propagates through here
}

fn level1() {
    level2()  // Error propagates through here
}

// Error will terminate program with stack trace:
// Error: Error in level3
//   at level3() (main.rustleaf:2:5)
//   at level2() (main.rustleaf:6:5)  
//   at level1() (main.rustleaf:10:5)
//   at main.rustleaf:13:1
level1()
```

**Error Catching:**
Try-catch blocks catch errors and prevent propagation.

```
fn risky_operation() {
    if random() < 0.5 {
        raise("Random failure")
    }
    "success"
}

try {
    var result = risky_operation()
    print("Success: ${result}")
} catch e {
    print("Caught error: ${e}")
    // Error is handled, execution continues
}

print("Program continues...")
```

**Errors in Expressions:**
Errors can occur in any expression and propagate immediately.

```
// Error during function call
var result = compute() + risky_function() + finalize()
// If risky_function() raises, compute() result is discarded
// and finalize() never executes

// Error during property access
var value = obj.safe_property + obj.risky_property + obj.final_property  
// If obj.risky_property raises, obj.final_property never accessed
```

### 15.5. Resource Cleanup

Resources are cleaned up deterministically using structured patterns.

**With Statement Cleanup:**
Resources are cleaned up in reverse order of acquisition.

```
with file1 = open("first.txt"), file2 = open("second.txt") {
    process_files(file1, file2)
}
// Cleanup order: file2.close(), then file1.close()
```

**Cleanup During Errors:**
Resources are cleaned up even when errors occur.

```
with file = open("data.txt") {
    try {
        dangerous_processing(file)
    } catch e {
        print("Error during processing: ${e}")
        raise(e)  // Re-raise
    }
}  // file.close() still called despite the error
```

**Nested Resource Management:**
Nested with statements clean up in proper order.

```
with outer = acquire_outer() {
    with inner = acquire_inner() {
        if error_condition {
            raise("Something went wrong")
        }
        use_resources(outer, inner)
    }  // inner.close() called first
}  // outer.close() called second

// Cleanup occurs even if error is raised:
// 1. inner.close() 
// 2. outer.close()
// 3. Error propagates
```

**Manual Cleanup Timing:**
Without structured cleanup, resources must be managed manually.

```
var resource = acquire_resource()
try {
    use_resource(resource)
} catch e {
    print("Error: ${e}")
    // Must manually clean up before re-raising
    resource.close()
    raise(e)
} 
// Must manually clean up in success case too
resource.close()
```

**Execution Context Hierarchy:**
RustLeaf maintains nested execution contexts for proper scoping.

```
// Global context
var global_var = "global"

fn outer_function() {
    // Function context (nested in global)
    var function_var = "function"
    
    fn inner_function() {
        // Inner function context (nested in outer function)
        var inner_var = "inner"
        
        // All variables accessible due to lexical scoping
        print(global_var)    // "global"
        print(function_var)  // "function"  
        print(inner_var)     // "inner"
    }
    
    inner_function()
    // inner_var not accessible here
}

outer_function()
// function_var and inner_var not accessible here
```

**Variable Lifetime and Initialization:**
Variables are not hoisted and must be declared before use.

```
// Error: Cannot use variable before declaration
print(x)  // Error: Undefined variable 'x'
var x = 42

// Block scoping
{
    var block_var = "block"
    print(block_var)  // OK
}
print(block_var)  // Error: Undefined variable 'block_var'

// Class field initialization
class Example {
    var field1 = "initialized";  // Initialized at object creation
    var field2;                  // Initialized to null at object creation
    
    static fn new() {
        var obj = Example()  // field1="initialized", field2=null
        obj.field2 = "set"   // Modified after creation
        obj
    }
}
```

