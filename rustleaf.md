# RustLeaf AI Agent Programming Guide

This is a concise reference for AI agents writing RustLeaf code. RustLeaf is a dynamically-typed, expression-oriented scripting language with Rust-inspired syntax.

## Core Syntax Rules

### Variables & Functions
```rustleaf
var name = "Alice";           // Variable declaration
var age = 30;                 // Integer
var height = 5.8;             // Float
var active = true;            // Boolean
var data = null;              // Null value

fn greet(name) {              // Function definition
    "Hello, ${name}!"         // String interpolation (last expr = return value)
}

fn add(x, y) { x + y }        // Single expression function
```

### Control Flow
```rustleaf
// If expression (returns value)
var status = if score >= 90 {
    "excellent"
} else if score >= 70 {
    "good"
} else {
    "needs work"
};

// For loops
for x in [1, 2, 3] {
    print(x);
}

// While loops
while condition {
    // statements
}

// Match expressions
var result = match value {
    case 0 { "zero" }
    case 1..10 { "single digit" }
    case _ { "other" }
};
```

### Data Structures
```rustleaf
// Lists
var numbers = [1, 2, 3, 4, 5];
var first = numbers[0];       // Index access
var last = numbers[-1];       // Negative indexing

// Dictionaries (objects)
var person = {
    name: "Bob",
    age: 25,
    "full name": "Bob Smith"  // String keys for spaces
};
var name = person.name;       // Property access
var age = person["age"];      // Index access
```

### Classes & Objects
```rustleaf
class Greeter {
    var name;                 // Field declaration

    fn greet() {              // Method
        "Hello, ${self.name}"
    }
}

var greeter = Greeter();      // Constructor call
greeter.name = "World";       // Field assignment
var msg = greeter.greet();    // Method call
```

## Key Language Features

### Pipe Operator for Chaining
```rustleaf
// Transform data in readable pipelines
var result = range(1, 10)
    | filter(|x| x % 2 == 0)  // Keep even numbers
    | map(|x| x * x)          // Square them
    | sum();                  // Sum the results
```

### Lambda Functions
```rustleaf
var double = |x| x * 2;       // Single parameter
var add = |a, b| a + b;       // Multiple parameters
var greet = || "Hello!";      // No parameters

// With block body
var complex = |n| {
    var temp = n * n;
    temp + 1
};
```

### Expression-Oriented Design
Most constructs are expressions that return values:
```rustleaf
// Blocks return their last expression
var result = {
    var temp = calculate();
    temp * 2                  // This is the block's value
};

// If expressions return values
var max = if a > b { a } else { b };

// Loops can return values via break
var found = while true {
    var item = get_next();
    if item.matches() {
        break item;           // Return the found item
    }
};
```

## Common Patterns

### Error Handling
```rustleaf
// Try expressions for error handling
var result = try {
    risky_operation()
} catch e {
    print("Error: ${e.message}");
    null
};

// Pattern matching on errors
var value = try {
    parse_int(input)
} catch {type: "ValueError"} {
    0  // Default for parse errors
};
```

### Data Processing Pipelines
```rustleaf
// Clean and transform data
var processed = raw_data
    | filter(|item| item.valid)
    | map(|item| {
        name: item.name.trim(),
        score: item.score * 100
    })
    | sort_by(|item| item.score);
```

### Functional Programming
```rustleaf
// Higher-order functions
fn apply_twice(func, value) {
    func(func(value))
}

var result = apply_twice(|x| x * 2, 5);  // 20

// Closures capture outer variables
fn make_counter() {
    var count = 0;
    |increment| {
        count += increment;
        count
    }
}

var counter = make_counter();
print(counter(5));  // 5
print(counter(3));  // 8
```

### Iterator-Style Operations
```rustleaf
// Common data processing patterns
var evens = numbers.filter(|n| n % 2 == 0);
var doubled = numbers.map(|n| n * 2);
var sum = numbers.reduce(|a, b| a + b, 0);
var any_large = numbers.any(|n| n > 100);
var all_positive = numbers.all(|n| n > 0);
```

## Best Practices for AI Agents

### 1. Use Expression-Oriented Style
```rustleaf
// Prefer this:
var result = if condition { value_a } else { value_b };

// Over this:
var result;
if condition {
    result = value_a;
} else {
    result = value_b;
}
```

### 2. Leverage Pipe Operator for Readability
```rustleaf
// Instead of nested calls:
var result = process(transform(filter(data, pred), func), opts);

// Use pipes for clarity:
var result = data
    | filter(pred)
    | transform(func)
    | process(opts);
```

### 3. Use Pattern Matching Effectively
```rustleaf
// Pattern match for complex data
match response {
    case {type: "success", data: data} {
        process_data(data)
    }
    case {type: "error", code: code, message: msg} {
        handle_error(code, msg)
    }
    case _ {
        handle_unknown(response)
    }
}
```

### 4. Utilize Lambda Functions for Inline Operations
```rustleaf
// Short lambdas for simple operations
var results = items
    | filter(|x| x.active)
    | map(|x| x.name.upper())
    | sort();

// Block lambdas for complex logic
var processed = data.map(|item| {
    var cleaned = item.name.trim();
    var normalized = cleaned.lower();
    {
        id: item.id,
        name: normalized,
        score: item.score * multiplier
    }
});
```

### 5. Handle Errors Appropriately
```rustleaf
// Use try expressions for recoverable errors
var config = try {
    load_config_file()
} catch e {
    print("Using default config: ${e.message}");
    default_config()
};

// Let errors propagate when appropriate
fn critical_operation() {
    var data = load_critical_data();  // Let this fail if it must
    process(data)
}
```

## Testing Patterns

Use `assert()` for testing:
```rustleaf
// Simple assertions
assert(add(2, 3) == 5);
assert("hello".upper() == "HELLO");

// Test with expected values from examples
var result = solve_euler_001(10);
assert(result == 23);  // Known expected value
```

## Common Gotchas

1. **Statements vs Expressions**: Assignment is a statement, not an expression
   ```rustleaf
   // This is wrong:
   // var x = (y = 5);  // Error: assignment is not an expression
   
   // This is correct:
   y = 5;
   var x = y;
   ```

2. **Logical Operators**: Use `and`/`or`/`not` keywords, not symbols
   ```rustleaf
   if active and not disabled { ... }  // Correct
   // if active && !disabled { ... }   // Wrong - use keywords
   ```

3. **String Interpolation**: Use `${}` syntax
   ```rustleaf
   var msg = "Hello, ${name}!";        // Correct
   // var msg = f"Hello, {name}!";     // Wrong - not Python
   ```

4. **Function Return**: Last expression is the return value
   ```rustleaf
   fn calculate(x) {
       var temp = x * 2;
       temp + 1                        // This is returned
   }
   ```

This guide covers the essential patterns for writing effective RustLeaf code. Focus on the expression-oriented nature and functional programming capabilities for clean, readable code.