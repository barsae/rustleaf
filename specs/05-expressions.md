# 5. Expressions

Expressions are constructs that evaluate to a value. RustLeaf is an expression-oriented language where most constructs produce values. This chapter defines the syntax, semantics, and evaluation rules for all expression forms.

**Expressions vs Statements:**
- **Expressions** evaluate to values and can be used anywhere a value is needed
- **Statements** either execute expressions for side effects or perform operations that cannot be expressions
- Assignment, variable declaration, and control flow (return/break/continue) are statements, not expressions
- This prevents common pitfalls like `if (x = 5)` while maintaining expression-oriented design

**What are Expressions:**
- Literals, identifiers, function calls, method calls
- Arithmetic, comparison, logical, and bitwise operations
- Conditional expressions (`if`), match expressions, try expressions
- Block expressions, anonymous functions, object/list/dict literals
- Loop expressions (`while`, `for`, `loop`) that can return values via `break`

**What are Statements (not expressions):**
- Assignment (`x = 5`, `x += 1`)
- Variable declaration (`var x = 5`)
- Control flow (`return`, `break`, `continue`)
- Import statements (`use module::item`)

### 5.1. Expression Evaluation

Expressions are evaluated to produce values according to specific rules that ensure predictable behavior.

**Evaluation Order:**
- Expressions are evaluated left-to-right
- Subexpressions are fully evaluated before the containing expression
- Function arguments are evaluated left-to-right before the function call
- Side effects occur in the order of evaluation

**Short-Circuit Evaluation:**
The logical operators `and` and `or` use short-circuit evaluation:
- `and`: If the left operand is falsy, the right operand is not evaluated
- `or`: If the left operand is truthy, the right operand is not evaluated

**Type Checking:**
- Type errors are raised when operations receive incompatible types
- Type checking occurs at the point of operation, not earlier

**Examples:**
```
// Left-to-right evaluation
var x = 1;
// var result = (x = 2) + (x = 3) + x;  // Error: assignment is not an expression

// Function argument evaluation
fn side_effect(n) {
    print("evaluating ${n}");
    n
}
var sum = add(side_effect(1), side_effect(2));
// Output: "evaluating 1", "evaluating 2"

// Short-circuit evaluation
var obj = null;
if obj != null and obj.name == "test" {  // Safe: obj.name not evaluated
    print("found")
}

fn expensive() {
    print("expensive called");
    true
}
var result = true or expensive();  // expensive() not called
```

### 5.2. Primary Expressions

Primary expressions are the simplest expression forms that serve as building blocks for more complex expressions.

**Primary Expression Types:**
1. **Literals** - Numeric, string, boolean, null (see Chapter 2)
2. **Identifiers** - Variable and function names
3. **Parenthesized Expressions** - Expressions in parentheses
4. **`self`** - Reference to current object in methods

**Identifier Resolution:**
- Identifiers are resolved in the current scope
- If not found locally, outer scopes are searched
- Unresolved identifiers raise runtime errors

**Parentheses:**
- Parentheses group expressions to override precedence
- No limit on nesting depth
- The value is the value of the contained expression

**Examples:**
```
// Literals
42
"hello"
true
null

// Identifiers
x
calculate
MyClass

// Parenthesized expressions
(2 + 3) * 4      // 20
((x + y) * z)    // Nested parentheses

// self in methods
class Counter {
    var count = 0;
    
    fn increment() {
        self.count = self.count + 1
    }
}
```

### 5.3. Postfix Expressions

Postfix expressions apply operations after their operand.

**Postfix Operations:**
1. **Property Access**: `expr.identifier`
2. **Index Access**: `expr[index]`
3. **Function Call**: `expr(args...)`
4. **Method Call**: `expr.method(args...)`

#### 5.3.1. Property Access

Access named fields or methods of an object.

**Syntax:**
```
PropertyAccess = Expression "." Identifier
```

**Semantics:**
- Evaluates the expression to get an object
- Looks up the identifier as a field or method
- Returns the field value or bound method
- Accessing properties on `null` raises an error

**Dynamic Property Access:**
Use indexing with string keys for dynamic access:
```
obj["property"]  // Same as obj.property
var key = "name";
obj[key]         // Dynamic property access
```

**Examples:**
```
var point = {x: 10, y: 20};
print(point.x);   // 10

var user = User();
user.name = "Alice";
user.greet();     // Method call

// Error cases
var n = null;
// n.anything    // Error: Cannot access property of null

// Dynamic access
var prop = "x";
print(point[prop]);  // 10
```

#### 5.3.2. Index Access

Access elements of lists, dicts, or custom indexable objects.

**Syntax:**
```
IndexAccess = Expression "[" Expression "]"
```

**Semantics:**
- Evaluates both expressions
- For lists: index must be an integer
- For dicts: index can be any valid key type
- For objects: calls `op_index` if defined
- Out-of-bounds access raises an error

**Examples:**
```
var list = [10, 20, 30];
print(list[0]);    // 10
print(list[-1]);   // 30 (negative indexing)

var dict = {a: 1, b: 2};
print(dict["a"]);  // 1

// Chained indexing
var matrix = [[1, 2], [3, 4]];
print(matrix[0][1]);  // 2

// Custom indexing
class Matrix {
    var data;
    
    fn op_index(row, col) {
        self.data[row * self.width + col]
    }
}
```

#### 5.3.3. Function Call

Call a function with arguments.

**Syntax:**
```
FunctionCall = Expression "(" ArgumentList? ")"
ArgumentList = Expression ("," Expression)*
```

**Semantics:**
- Evaluates the function expression
- Evaluates all arguments left-to-right
- Calls the function with the arguments
- Returns the function's return value

**Examples:**
```
print("Hello");
add(2, 3);
get_function()();  // Call returned function

// With spread (if supported)
var args = [1, 2, 3];
sum(*args);  // Spread arguments
```

#### 5.3.4. Method Call

Call a method on an object.

**Syntax:**
```
MethodCall = Expression "." Identifier "(" ArgumentList? ")"
```

**Semantics:**
- Evaluates the object expression
- Looks up the method by name
- Binds `self` to the object
- Calls the method with arguments
- Returns the method's return value

**Method Chaining:**
Methods that return values can be chained:
```
var result = text.replace("old", "new")
                 .upper()
                 .trim()
                 .split(" ")
```

**Examples:**
```
var list = [3, 1, 4, 1, 5];
list.sort();       // Modifies list
var doubled = list.map(|x| x * 2);

// Chaining
var words = "  hello world  "
    .trim()
    .upper()
    .split(" ");  // ["HELLO", "WORLD"]
```

### 5.4. Unary Expressions

Unary expressions apply an operator to a single operand.

**Unary Operators:**
```
-   // Arithmetic negation
not // Logical negation  
~   // Bitwise NOT
```

**Syntax:**
```
UnaryExpression = UnaryOperator Expression
UnaryOperator = "-" | "not" | "~"
```

**Operator Semantics:**
- `-`: Negates numbers (int or float)
- `not`: Inverts truthiness (requires bool or null)
- `~`: Bitwise complement (int only)

**Examples:**
```
var x = 42;
print(-x);         // -42
print(not true);   // false
print(~0b1010);    // -11 (bitwise NOT)

// Type checking
// -"hello"       // Error: Cannot negate string
// not 42         // Error: int has no truthiness
// ~3.14          // Error: Bitwise NOT requires int
```

### 5.5. Binary Expressions

Binary expressions combine two operands with an operator.

#### 5.5.1. Arithmetic Operators

Operate on numeric values.

**Operators:**
```
+   // Addition
-   // Subtraction
*   // Multiplication
/   // Division
%   // Modulo
**  // Exponentiation
```

**Type Rules:**
- Both operands must be numeric (int or float)
- Exception: `+` works on strings for concatenation
- Mixed int/float promotes to float
- Division by zero: integer raises error, float returns Infinity

**Examples:**
```
print(10 + 3)     // 13
print(10 - 3)     // 7
print(10 * 3)     // 30
print(10 / 3);     // 3 (integer division)
print(10.0 / 3);   // 3.333... (float division)
print(10 % 3)     // 1
print(2 ** 8)     // 256

// String concatenation
print("Hello" + " " + "World")  // "Hello World"

// Type promotion
print(10 + 3.14);  // 13.14 (float)
```

#### 5.5.2. Comparison Operators

Compare values and return boolean results.

**Operators:**
```
==  // Equal
!=  // Not equal
<   // Less than
>   // Greater than
<=  // Less than or equal
>=  // Greater than or equal
```

**Comparison Rules:**
- Equality works for all types
- Ordering (<, >, <=, >=) requires compatible types
- Strings compare lexicographically
- No comparison chaining (use `and` for multiple comparisons)

**Examples:**
```
print(10 == 10)   // true
print(10 != 20)   // true
print(10 < 20)    // true
print("abc" < "def")  // true

// Type checking
print(10 == "10"); // false (different types)

// No chaining
// if 0 < x < 10 { }  // Error
if 0 < x and x < 10 { }  // Correct
```

#### 5.5.3. Logical Operators

Operate on values with truthiness.

**Operators:**
```
and // Logical AND (short-circuit)
or  // Logical OR (short-circuit)
xor // Logical XOR (exclusive-or)
```

**Truthiness Rules:**
- Only `null` and `bool` have truthiness
- `null` and `false` are falsy
- `true` is truthy
- Other types raise errors in boolean context

**Examples:**
```
print(true and false)   // false
print(true or false)    // true
print(true xor false)   // true
print(true xor true)    // false
print(null or true)     // true

// Short-circuit behavior
var x = true;
var y = (x or print("not evaluated"));

// XOR does not short-circuit - both operands are always evaluated
print(true xor print("always evaluated"))  // "always evaluated" is printed

// Type errors
// if 0 and true { }    // Error: int has no truthiness
// if [] or true { }    // Error: list has no truthiness
// if 1 xor true { }    // Error: int has no truthiness
```

#### 5.5.4. Bitwise Operators

Operate on integer bit representations.

**Operators:**
```
&   // Bitwise AND
|   // Bitwise OR
^   // Bitwise XOR
<<  // Left shift
>>  // Right shift
```

**Rules:**
- Only work on integers
- Shifts use the right operand as shift count
- Negative shift counts raise errors

**Examples:**
```
print(0b1010 & 0b1100);  // 0b1000 (8)
print(0b1010 | 0b1100);  // 0b1110 (14)
print(0b1010 ^ 0b1100);  // 0b0110 (6)
print(1 << 3)           // 8
print(16 >> 2)          // 4
```

#### 5.5.5. Operator Precedence

Operators are evaluated according to precedence levels from highest to lowest precedence. For the complete precedence table with associativity rules, see **Appendix B: Operator Precedence**.

**Examples:**
```
2 + 3 * 4         // 14 (not 20)
not true or false // false (not is higher precedence)
1 << 2 + 1        // 8 (+ before <<)
true and false xor true  // true (and before xor)
true xor false or false  // true (xor before or)
```

### 5.6. Conditional Expressions (if)

If expressions choose between values based on conditions.

**Syntax:**
```
IfExpression = "if" Expression Block 
               ("else" "if" Expression Block)* 
               ("else" Block)?
```

**Semantics:**
- Condition must have truthiness (bool or null)
- Evaluates to the value of the executed block
- If no else clause and condition is false, evaluates to unit
- All blocks use block expression rules (last expression is value)

**Examples:**
```
var status = if score >= 90 {
    "excellent"
} else if score >= 70 {
    "good"  
} else {
    "needs improvement"
}

// Without else evaluates to unit
var reward = if perfect_score {
    1000
}  // reward is 1000 or unit

// Nested if expressions
var category = if age < 18 {
    if age < 13 { "child" } else { "teen" }
} else {
    "adult"
}
```

### 5.7. Match Expressions

Match expressions provide pattern matching with multiple cases.

**Syntax:**
```
MatchExpression = "match" Expression "{" MatchArm+ "}"
MatchArm = "case" Pattern Guard? Block
Guard = "if" Expression
```

**Pattern Types:**
- Literal patterns: `42`, `"hello"`, `true`
- Variable patterns: `x` (binds the value)
- Wildcard pattern: `_` (matches anything)
- List patterns: `[a, b]`, `[head, *tail]`
- Dict patterns: `{x, y}`, `{name: n}`
- Range patterns: `1..10` (inclusive)

**Semantics:**
- Patterns are tested top-to-bottom
- First matching pattern is selected
- Guards provide additional conditions
- Non-exhaustive matches are allowed (evaluates to unit if no match)
- Variables bound in patterns are scoped to the case block

**Examples:**
```
var description = match value {
    case 0 {
        "zero"
    }
    case 1..10 {
        "single digit"
    }
    case n if n < 0 {
        "negative"
    }
    case [first, *rest] {
        "list with first: ${first}"
    }
    case {type: "user", name: name} {
        "user named ${name}"
    }
    case _ {
        "other"
    }
}

// Non-exhaustive match
var result = match x {
    case 1 { "one" }
    case 2 { "two" }
}  // result is unit if x is not 1 or 2

// Pattern binding
match point {
    case {x: 0, y: 0} {
        print("origin")
    }
    case {x: x, y: 0} {
        print("on x-axis at ${x}")
    }
    case {x: 0, y: y} {
        print("on y-axis at ${y}")
    }
    case {x: x, y: y} {
        print("at (${x}, ${y})")
    }
}
```

### 5.8. Try Expressions

Try expressions handle errors by catching them.

**Syntax:**
```
TryExpression = "try" Block 
                "catch" Pattern Block
```

**Semantics:**
- Evaluates the try block
- If an error is raised, catches it and evaluates catch block
- The expression evaluates to either the try or catch block value
- Uses identical syntax to try-catch statements (Section 6.7) - context determines usage

**Error Patterns:**
The catch clause can pattern match on the error:
```
catch e         // Catch all errors as e
catch {type: "ValueError", message: m}  // Destructure error
```

**Examples:**
```
var result = try {
    risky_operation();
    42  // Success value
} catch e {
    print("Error: ${e.message}");
    0   // Error value
}

// Use with statement for cleanup instead
var data = try {
    with file = open("data.json") {
        parse_json(file.read())
    }
} catch e {
    print("Failed to load data");
    {}  // Empty dict as fallback
}

// Pattern matching errors
var value = try {
    parse_int(input)
} catch {type: "ValueError"} {
    0  // Default for parse errors
} catch e {
    raise(e)  // Re-raise other errors
}
```

### 5.9. Block Expressions

Blocks are expressions that contain statements and evaluate to a value.

**Syntax:**
```
BlockExpression = "{" Statement* Expression? "}"
```

**Semantics:**
- Executes all statements in order
- The last expression (if present) is the block's value
- If no final expression, evaluates to unit
- Creates a new scope for variables

**Examples:**
```
var result = {
    var temp = calculate_something();
    print("Calculated: ${temp}");
    temp * 2  // Block value
}

// Assignment using block
var category = {
    var score = get_score();
    if score > 90 {
        "excellent"
    } else if score > 70 {
        "good"
    } else {
        "needs work"
    }
}

// Empty block evaluates to unit
var nothing = {};  // unit

// Scope isolation
var outer = 10;
var result = {
    var outer = 20;  // Shadows
    outer + 5       // 25
};
print(outer);        // Still 10
```

### 5.10. Closure Expressions

Closures create function values inline using concise Rust-style syntax.

**Syntax:**
```
ClosureExpression = "|" ParameterList? "|" (Expression | Block)
ParameterList = Identifier ("," Identifier)*
```

**Semantics:**
- Creates a function value
- Captures variables from enclosing scope by reference
- Can be assigned, passed, or called immediately
- Single expressions don't require braces, blocks do

**Parameter Syntax:**
```
|x| expression          // Single parameter
|a, b| expression       // Multiple parameters
|| expression           // No parameters
```

**Body Forms:**
```
|x| x * 2              // Single expression (no braces)
|x| {                  // Block for multiple statements
    var doubled = x * 2
    doubled + 1
}
```

**Examples:**
```
var double = |x| x * 2;
print(double(21));  // 42

// Higher-order functions
var numbers = [1, 2, 3, 4, 5];
var evens = numbers.filter(|n| n % 2 == 0);
var squares = numbers.map(|n| n * n);

// Closures
fn make_adder(x) {
    |y| x + y  // Captures x
}
var add5 = make_adder(5);
print(add5(3));  // 8

// Immediately invoked
var result = |a, b| a + b(10, 20);  // 30

// Multi-statement closure
var process = |data| {
    var cleaned = data.trim();
    var upper = cleaned.upper();
    upper.split(" ")
};

// No parameters
var get_random = || 42;
print(get_random());  // 42

// Complex expressions without braces
var formatted = items.map(|item| "Item: ${item.name}");
var valid = data.filter(|x| x > 0 and x < 100);
```

### 5.11. Object Literal Expressions

Object literals create dictionary objects with string keys.

**Syntax:**
```
ObjectLiteral = "{" (Property ("," Property)*)? "}"
Property = (Identifier | StringLiteral) ":" Expression
```

**Semantics:**
- Creates a new dict object
- Keys can be identifiers (converted to strings) or string literals
- Values are any expressions
- Evaluation order is left-to-right

**Examples:**
```
var point = {x: 10, y: 20};
var person = {
    name: "Alice",
    age: 30,
    "full name": "Alice Smith",  // String key for spaces
    hobbies: ["reading", "coding"]
};

// Computed values
var config = {
    host: get_host(),
    port: DEFAULT_PORT,
    debug: env == "development"
};

// Nested objects
var data = {
    user: {
        id: 123,
        profile: {
            name: "Bob"
        }
    }
}
```

### 5.12. List Literal Expressions

List literals create ordered collections.

**Syntax:**
```
ListLiteral = "[" (Expression ("," Expression)*)? "]"
```

**Semantics:**
- Creates a new list object
- Elements can be any expressions
- Evaluation order is left-to-right
- Lists are mutable after creation

**Examples:**
```
var empty = [];
var numbers = [1, 2, 3, 4, 5];
var mixed = [42, "hello", true, [1, 2]];

// Computed elements
var data = [
    get_value(),
    x + y,
    if condition { 100 } else { 200 }
];

// List comprehension (if supported)
var squares = [x * x for x in range(1, 10)];
var evens = [x for x in numbers if x % 2 == 0];
```

### 5.13. Dict Literal Expressions

Dict literals create key-value mappings. This is the same as object literals (Section 5.12) but shown here for completeness.

**Syntax:**
```
DictLiteral = "{" (DictEntry ("," DictEntry)*)? "}"
DictEntry = Expression ":" Expression
```

**Examples:**
```
var empty = {};
var scores = {"Alice": 95, "Bob": 87};
var dynamic = {
    get_key(): get_value(),
    [computed_key]: computed_value
};
```

### 5.14. Loop Expressions

Loop expressions provide iteration with the ability to return values through break statements.

**Loop Types:**
- **While expressions**: Iterate while a condition is true
- **For expressions**: Iterate over a collection
- **Loop expressions**: Infinite loops (must break to exit)

#### 5.14.1. While Expressions

While expressions repeatedly evaluate a block while a condition is true.

**Syntax:**
```
WhileExpression = "while" Expression Block
```

**Semantics:**
- Evaluates condition before each iteration
- Executes block if condition is truthy
- Returns the value from `break` statement, or unit if condition becomes false
- `break` without expression returns unit
- `continue` skips to next iteration

**Examples:**
```
// Find first even number
var first_even = while i < numbers.len() {
    if numbers[i] % 2 == 0 {
        break numbers[i];  // Return the even number
    }
    i += 1;
};
// first_even is either the number or unit

// Process until condition met
var result = while true {
    var input = get_input();
    if input == "quit" {
        break "user_quit";
    }
    if process(input) {
        break "success";
    }
};
print("Result: ${result}");

// Loop that might not break
var attempt_result = while attempts < max_attempts {
    if try_operation() {
        break "succeeded";
    }
    attempts += 1;
};
// attempt_result is "succeeded" or unit (if max attempts reached)
```

#### 5.14.2. For Expressions

For expressions iterate over collections using the iterator protocol.

**Syntax:**
```
ForExpression = "for" Pattern "in" Expression Block
```

**Semantics:**
- Iterates over the collection using iterator protocol
- Returns the value from `break` statement, or unit if iteration completes
- Pattern binds values as in for statements
- `break` without expression returns unit

**Examples:**
```
// Find first matching item
var found = for item in collection {
    if item.matches(criteria) {
        break item;
    }
};

if is_unit(found) {
    print("Not found");
} else {
    print("Found: ${found}");
}

// Process until specific condition
var result = for name, value in config.items() {
    if name == "database_url" {
        break validate_url(value);
    }
};

// Complex search with destructuring
var match_data = for [key, data] in key_value_pairs {
    if data.priority > threshold {
        break {key: key, data: data, found_at: get_timestamp()};
    }
};
```

#### 5.14.3. Loop Expressions

Loop expressions create infinite loops that must be exited with break.

**Syntax:**
```
LoopExpression = "loop" Block
```

**Semantics:**
- Executes block repeatedly forever
- Must use `break` to exit and return value
- `break` without expression returns unit
- Useful for complex control flow

**Examples:**
```
// Read-eval-print loop
var exit_code = loop {
    var input = read_line();
    
    if input == "quit" {
        break 0;  // Normal exit
    }
    
    if input == "error" {
        break 1;  // Error exit
    }
    
    try {
        var result = evaluate(input);
        print("Result: ${result}");
    } catch e {
        print("Error: ${e}");
        if e.fatal {
            break 2;  // Fatal error exit
        }
    }
};

// Retry with exponential backoff
var final_result = loop {
    try {
        break attempt_operation();  // Success
    } catch e {
        if attempts >= max_attempts {
            break null;  // Give up
        }
        sleep(delay);
        delay *= 2;
        attempts += 1;
    }
};
```

#### 5.14.4. Break and Continue in Expressions

Break and continue statements work in loop expressions with specific semantics.

**Break with Values:**
```
break;           // Returns unit from loop
break value;     // Returns value from loop
break compute(); // Returns computed value from loop
```

**Continue (No Values):**
```
continue;        // Skip to next iteration (for/while only)
```

**Nested Loops:**
```
var result = for outer in outer_collection {
    for inner in inner_collection {
        if complex_condition(outer, inner) {
            break [outer, inner];  // Breaks inner loop only
        }
    }
};
```

**Type Consistency:**
All break statements in a loop must return compatible types:
```
// Valid: all breaks return int
var result = while condition {
    if case1 {
        break 1;
    }
    if case2 {
        break 2;
    }
};  // result has type int | unit

// Invalid: incompatible break types
var result = while condition {
    if case1 {
        break "string";  // string
    }
    if case2 {
        break 42;        // int - type error
    }
};
```

### Operator Overloading

Classes can define special methods to overload operators.

**Operator Methods:**
```
// Arithmetic
op_add(other)      // +
op_sub(other)      // -
op_mul(other)      // *
op_div(other)      // /
op_mod(other)      // %
op_pow(other)      // **

// Comparison
op_eq(other)       // ==
op_ne(other)       // !=
op_lt(other)       // <
op_gt(other)       // >
op_le(other)       // <=
op_ge(other)       // >=

// Bitwise
op_and(other)      // &
op_or(other)       // |
op_xor(other)      // ^
op_lshift(other)   // <<
op_rshift(other)   // >>

// Logical
op_logical_xor(other)  // xor

// Unary
op_neg()           // - (unary)
op_not()           // not
op_bitnot()        // ~

// Other
op_index(key)      // [] (get)
op_setindex(key, value)  // [] (set)
op_contains(item)  // in
```

**Example:**
```
class Vector {
    var x;
    var y;
    
    static fn new(x, y) {
        var v = Vector();
        v.x = x;
        v.y = y;
        v;
    }
    
    fn op_add(other) {
        Vector.new(self.x + other.x, self.y + other.y)
    }
    
    fn op_mul(scalar) {
        Vector.new(self.x * scalar, self.y * scalar)
    }
    
    fn op_eq(other) {
        self.x == other.x and self.y == other.y
    }
}

var a = Vector.new(1, 2);
var b = Vector.new(3, 4);
var c = a + b;      // Vector(4, 6)
var d = a * 2;      // Vector(2, 4)
print(a == b);      // false
```

---

