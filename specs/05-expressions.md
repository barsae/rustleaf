# 5. Expressions

Expressions are constructs that evaluate to a value. RustLeaf is an expression-oriented language where most constructs produce values. This chapter defines the syntax, semantics, and evaluation rules for all expression forms.

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
var x = 1
var result = (x = 2) + (x = 3) + x  // Error: assignment is not an expression

// Function argument evaluation
fn side_effect(n) {
    print("evaluating ${n}")
    n
}
var sum = add(side_effect(1), side_effect(2))
// Output: "evaluating 1", "evaluating 2"

// Short-circuit evaluation
var obj = null
if obj != null and obj.name == "test" {  // Safe: obj.name not evaluated
    print("found")
}

fn expensive() {
    print("expensive called")
    true
}
var result = true or expensive()  // expensive() not called
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
var key = "name"
obj[key]         // Dynamic property access
```

**Examples:**
```
var point = {x: 10, y: 20}
print(point.x)   // 10

var user = User()
user.name = "Alice"
user.greet()     // Method call

// Error cases
var n = null
// n.anything    // Error: Cannot access property of null

// Dynamic access
var prop = "x"
print(point[prop])  // 10
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
var list = [10, 20, 30]
print(list[0])    // 10
print(list[-1])   // 30 (negative indexing)

var dict = {a: 1, b: 2}
print(dict["a"])  // 1

// Chained indexing
var matrix = [[1, 2], [3, 4]]
print(matrix[0][1])  // 2

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
print("Hello")
add(2, 3)
get_function()()  // Call returned function

// With spread (if supported)
var args = [1, 2, 3]
sum(*args)  // Spread arguments
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
var list = [3, 1, 4, 1, 5]
list.sort()       // Modifies list
var doubled = list.map(fn(x) { x * 2 })

// Chaining
var words = "  hello world  "
    .trim()
    .upper()
    .split(" ")  // ["HELLO", "WORLD"]
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
var x = 42
print(-x)         // -42
print(not true)   // false
print(~0b1010)    // -11 (bitwise NOT)

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
print(10 / 3)     // 3 (integer division)
print(10.0 / 3)   // 3.333... (float division)
print(10 % 3)     // 1
print(2 ** 8)     // 256

// String concatenation
print("Hello" + " " + "World")  // "Hello World"

// Type promotion
print(10 + 3.14)  // 13.14 (float)
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
print(10 == "10") // false (different types)

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
var x = true
var y = (x or print("not evaluated"))

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
print(0b1010 & 0b1100)  // 0b1000 (8)
print(0b1010 | 0b1100)  // 0b1110 (14)
print(0b1010 ^ 0b1100)  // 0b0110 (6)
print(1 << 3)           // 8
print(16 >> 2)          // 4
```

#### 5.5.5. Operator Precedence

Operators are evaluated according to precedence levels (highest to lowest):

```
1.  **                          (right-associative)
2.  - (unary), not, ~
3.  *, /, %
4.  +, -
5.  <<, >>
6.  &
7.  ^
8.  |
9.  ==, !=, <, >, <=, >=
10. and
11. xor
12. or
```

**Examples:**
```
2 + 3 * 4         // 14 (not 20)
not true or false // false (not is higher precedence)
1 << 2 + 1        // 8 (+ before <<)
true and false xor true  // true (and before xor)
true xor false or false  // true (xor before or)
```

### 5.6. Assignment Expressions

Note: Assignment is a statement in RustLeaf, not an expression. See Section 4.4 for assignment syntax.

### 5.7. Conditional Expressions (if)

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
- If no else clause and condition is false, evaluates to null
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

// Without else evaluates to null
var reward = if perfect_score {
    1000
}  // reward is 1000 or null

// Nested if expressions
var category = if age < 18 {
    if age < 13 { "child" } else { "teen" }
} else {
    "adult"
}
```

### 5.8. Match Expressions

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
- Non-exhaustive matches are allowed (evaluates to null if no match)
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
}  // result is null if x is not 1 or 2

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

### 5.9. Try Expressions

Try expressions handle errors by catching exceptions.

**Syntax:**
```
TryExpression = "try" Block 
                "catch" Pattern Block
                ("finally" Block)?
```

**Semantics:**
- Evaluates the try block
- If an exception is raised, catches it and evaluates catch block
- Finally block always executes but doesn't affect the expression value
- The expression evaluates to either the try or catch block value

**Error Patterns:**
The catch clause can pattern match on the error:
```
catch e         // Catch all errors as e
catch {type: "ValueError", message: m}  // Destructure error
```

**Examples:**
```
var result = try {
    risky_operation()
    42  // Success value
} catch e {
    print("Error: ${e.message}")
    0   // Error value
}

// With finally (for cleanup)
var data = try {
    var file = open("data.json")
    parse_json(file.read())
} catch e {
    print("Failed to load data")
    {}  // Empty dict as fallback
} finally {
    if file {
        file.close()  // Always runs
    }
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

### 5.10. Block Expressions

Blocks are expressions that contain statements and evaluate to a value.

**Syntax:**
```
BlockExpression = "{" Statement* Expression? "}"
```

**Semantics:**
- Executes all statements in order
- The last expression (if present) is the block's value
- If no final expression, evaluates to null
- Creates a new scope for variables

**Examples:**
```
var result = {
    var temp = calculate_something()
    print("Calculated: ${temp}")
    temp * 2  // Block value
}

// Assignment using block
var category = {
    var score = get_score()
    if score > 90 {
        "excellent"
    } else if score > 70 {
        "good"
    } else {
        "needs work"
    }
}

// Empty block evaluates to null
var nothing = {}  // null

// Scope isolation
var outer = 10
var result = {
    var outer = 20  // Shadows
    outer + 5       // 25
}
print(outer)        // Still 10
```

### 5.11. Anonymous Function Expressions

Anonymous functions (lambdas) create function values inline.

**Syntax:**
```
LambdaExpression = "fn" "(" ParameterList? ")" Block
```

**Semantics:**
- Creates a function value
- Captures variables from enclosing scope by reference
- Can be assigned, passed, or called immediately
- Always requires braces for the body

**Examples:**
```
var double = fn(x) { x * 2 }
print(double(21))  // 42

// Higher-order functions
var numbers = [1, 2, 3, 4, 5]
var evens = numbers.filter(fn(n) { n % 2 == 0 })
var squares = numbers.map(fn(n) { n * n })

// Closures
fn make_adder(x) {
    fn(y) { x + y }  // Captures x
}
var add5 = make_adder(5)
print(add5(3))  // 8

// Immediately invoked
var result = fn(a, b) { a + b }(10, 20)  // 30

// Multi-statement lambda
var process = fn(data) {
    var cleaned = data.trim()
    var upper = cleaned.upper()
    upper.split(" ")
}
```

### 5.12. Object Literal Expressions

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
var point = {x: 10, y: 20}
var person = {
    name: "Alice",
    age: 30,
    "full name": "Alice Smith",  // String key for spaces
    hobbies: ["reading", "coding"]
}

// Computed values
var config = {
    host: get_host(),
    port: DEFAULT_PORT,
    debug: env == "development"
}

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

### 5.13. List Literal Expressions

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
var empty = []
var numbers = [1, 2, 3, 4, 5]
var mixed = [42, "hello", true, [1, 2]]

// Computed elements
var data = [
    get_value(),
    x + y,
    if condition { 100 } else { 200 }
]

// List comprehension (if supported)
var squares = [x * x for x in range(1, 10)]
var evens = [x for x in numbers if x % 2 == 0]
```

### 5.14. Dict Literal Expressions

Dict literals create key-value mappings. This is the same as object literals (Section 5.12) but shown here for completeness.

**Syntax:**
```
DictLiteral = "{" (DictEntry ("," DictEntry)*)? "}"
DictEntry = Expression ":" Expression
```

**Examples:**
```
var empty = {}
var scores = {"Alice": 95, "Bob": 87}
var dynamic = {
    get_key(): get_value(),
    [computed_key]: computed_value
}
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
        var v = Vector()
        v.x = x
        v.y = y
        v
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

var a = Vector.new(1, 2)
var b = Vector.new(3, 4)
var c = a + b      // Vector(4, 6)
var d = a * 2      // Vector(2, 4)
print(a == b)      // false
```

---

