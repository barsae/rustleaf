# 8. Pattern Matching

Pattern matching allows destructuring and matching values against patterns. Patterns appear in match expressions, variable declarations, assignments, function parameters, and catch clauses. This chapter defines the syntax and semantics of all pattern forms.

### 8.1. Pattern Syntax

Patterns are structural descriptions that match against values and optionally bind variables.

**Pattern Contexts:**
1. **Match expressions** - `match value { case pattern { ... } }`
2. **Variable declarations** - `var pattern = expression`
3. **Assignments** - `pattern = expression`
4. **Function parameters** - `fn f(pattern) { ... }`
5. **For loops** - `for pattern in iterable { ... }`
6. **Catch clauses** - `catch pattern { ... }`

**Pattern Types:**
- Literal patterns - Match exact values
- Variable patterns - Bind values to names
- Wildcard pattern - Match anything without binding
- List patterns - Match list structure
- Dict patterns - Match dict structure
- Range patterns - Match numeric ranges

**General Rules:**
- Patterns match structurally against values
- Pattern matching is strict (no type coercion)
- Variables in patterns create new bindings
- Patterns in destructuring contexts must be irrefutable

**Examples:**
```
// Match expression
match value {
    case 0 { "zero" }
    case 1 { "small" }
    case 2 { "small" }
    case 3 { "small" }
    case [x, y] { "pair" }
    case _ { "other" }
}

// Destructuring declaration
var {name, age} = person
var [first, *rest] = items

// Function parameter
fn process_pair([x, y]) {
    x + y
}

// For loop destructuring
for key, value in dict.items() {
    print("${key}: ${value}")
}
```

### 8.2. Literal Patterns

Literal patterns match exact values using equality comparison.

**Allowed Literals:**
- Integer literals: `42`, `0xFF`, `0b1010`
- String literals: `"hello"`, `"""multiline"""`
- Boolean literals: `true`, `false`
- Null literal: `null`

**Not Allowed:**
- Float literals (due to inexact comparison)
- Expressions (only literal syntax)

**Matching Rules:**
- Uses `==` equality comparison
- No type coercion
- String patterns use exact string equality

**Examples:**
```
match status {
    case 200 { "OK" }
    case 404 { "Not Found" }
    case 500 { "Server Error" }
}

match value {
    case "yes" { true }
    case "y" { true }
    case "true" { true }
    case "no" { false }
    case "n" { false }
    case "false" { false }
    case null { "missing" }
}

// Not allowed
match x {
    // case 3.14 { }  // Error: float patterns not allowed
    // case 2+2 { }   // Error: expressions not allowed
}
```

### 8.3. Variable Patterns

Variable patterns bind matched values to new variables.

**Syntax:**
- Any valid identifier that's not a keyword
- Creates a new variable in the pattern's scope

**Binding Rules:**
- Always creates a new binding (shadows existing)
- Variable is scoped to the pattern's context
- In match cases, scoped to the case block
- Cannot rebind within the same pattern

**Examples:**
```
// Simple binding
match value {
    case x { print("Got ${x}") }
}

// Binding in destructuring
var [a, b, c] = [1, 2, 3]

// Nested binding
match data {
    case {user: u, score: s} {
        print("User ${u} scored ${s}")
    }
}

// Scope example
var x = 10
match 20 {
    case x {  // New binding, shadows outer x
        print(x)  // 20
    }
}
print(x)  // 10 (unchanged)

// Not allowed - duplicate binding
// var [x, x] = [1, 2]  // Error: x bound twice
```

### 8.4. Wildcard Patterns

The wildcard pattern `_` matches any value without binding it.

**Properties:**
- Matches anything
- Does not create a binding
- Can appear multiple times in a pattern
- Useful for ignoring values

**Examples:**
```
// Ignore values
var [first, _, third] = [1, 2, 3]
print(first)   // 1
print(third)   // 3
// _ is not a variable

// Match anything
match value {
    case 0 { "zero" }
    case _ { "non-zero" }
}

// Ignore multiple values
var [x, _, _, y] = [1, 2, 3, 4]

// In function parameters
list.map(fn(_) { 42 })  // Ignore argument

// Partial dict matching
var {name, _} = {name: "Alice", age: 30, id: 123}
// Only extracts name
```

### 8.5. List Patterns

List patterns match the structure of lists and can destructure elements.

**Syntax:**
```
ListPattern = "[" PatternList? "]"
PatternList = Pattern ("," Pattern)* ("," "*" Pattern)?
```

**Features:**
- Match exact length (without rest)
- Rest patterns `*name` collect remaining elements
- Elements can be any pattern (nested)
- Rest pattern can appear anywhere but only once

**Matching Rules:**
- List must have exact length unless rest pattern used
- Rest pattern binds to a list of remaining elements
- Empty rest binds to empty list

**Examples:**
```
// Exact length match
match list {
    case [] { "empty" }
    case [x] { "single: ${x}" }
    case [x, y] { "pair: ${x}, ${y}" }
    case [x, y, z] { "triple" }
}

// Rest patterns
var [head, *tail] = [1, 2, 3, 4]
// head = 1, tail = [2, 3, 4]

var [first, *middle, last] = [1, 2, 3, 4, 5]
// first = 1, middle = [2, 3, 4], last = 5

var [*all] = [1, 2, 3]
// all = [1, 2, 3]

// Nested patterns
match matrix {
    case [[a, b], [c, d]] {
        // 2x2 matrix
        a * d - b * c  // determinant
    }
}

// Multiple patterns
match coords {
    case [0, 0] { "origin" }
    case [0, _] { "on y axis" }
    case [_, 0] { "on x axis" }
}
```

### 8.6. Dict Patterns

Dict patterns match dictionary structure and extract values by key.

**Syntax:**
```
DictPattern = "{" FieldPatternList? "}"
FieldPattern = Identifier (":" Pattern)?
             | StringLiteral ":" Pattern
```

**Features:**
- Partial matching (only specified keys required)
- Shorthand `{x}` for `{x: x}`
- Rename with `{key: newname}`
- Keys must be literal strings or identifiers
- Values can be any pattern (nested)

**Matching Rules:**
- Only specified keys must exist
- Extra keys in dict are ignored
- All specified keys must match

**Examples:**
```
// Basic extraction
var {name, age} = {name: "Alice", age: 30, id: 123}
// name = "Alice", age = 30

// Renaming
var {name: userName, age: userAge} = user
// userName gets user.name value

// Nested patterns
match request {
    case {method: "GET", path: "/users"} {
        list_users()
    }
    case {method: "POST", data: {name: n}} {
        create_user(n)
    }
}

// String literal keys
var {"content-type": contentType} = headers

// Mixed patterns
match response {
    case {status: 200, body: [first, *rest]} {
        process_success(first, rest)
    }
    case {status: 404} {
        handle_not_found()
    }
}

// Multiple dict patterns
match config {
    case {mode: "dev"} { enable_debug() }
    case {mode: "development"} { enable_debug() }
}
```

### 8.7. Range Patterns

Range patterns match integers within ranges.

**Syntax:**
```
RangePattern = Expression ".." Expression
             | Expression "..=" Expression
```

**Properties:**
- `..` creates an exclusive range (end not included)
- `..=` creates an inclusive range (end included) 
- Expressions are evaluated at match time
- Start must be less than or equal to end

**Examples:**
```
match score {
    case 0 { "zero" }
    case 1..=10 { "low" }        // inclusive: 1 through 10
    case 11..50 { "medium" }     // exclusive: 11 through 49
    case 50..=100 { "high" }     // inclusive: 50 through 100
    case _ { "out of range" }
}

match char_code {
    case 48..=57 { "digit" }      // '0' through '9' inclusive
    case 65..=90 { "uppercase" }  // 'A' through 'Z' inclusive
    case 97..=122 { "lowercase" } // 'a' through 'z' inclusive
}

// Multiple range patterns
match age {
    case 0..12 { "child discount" }
    case 65..120 { "senior discount" }
}

// Not allowed
// case x..y { }        // Error: variables not allowed
// case 1.5..2.5 { }    // Error: floats not allowed
// case 10..5 { }       // Error: invalid range
```

### 8.8. Guard Expressions

Note: Guard expressions (conditional patterns) are not supported in RustLeaf. Use nested if statements within case blocks instead.

```
// Instead of guards, use:
match value {
    case x {
        if x > 0 {
            "positive"
        } else {
            "non-positive"
        }
    }
}
```

### 8.9. Pattern Evaluation

Pattern matching follows specific evaluation rules to determine matches and bind variables.

**Evaluation Order:**
1. Patterns are tested top-to-bottom
2. First matching pattern is selected
3. Nested patterns are matched outside-in

**Matching Process:**
1. Compare pattern structure with value
2. For literals, test equality
3. For ranges, test inclusion
4. For lists/dicts, recursively match elements
5. Bind variables if pattern matches
6. Stop at first match (no fallthrough)

**Irrefutable Patterns:**
Patterns that always match:
- Variable patterns
- Wildcard pattern `_`
- Destructuring with only variables and wildcards

Required in:
- Variable declarations
- Assignments
- Function parameters
- For loops

**Refutable Patterns:**
Patterns that might not match:
- Literal patterns
- Range patterns
- List patterns with specific length
- Dict patterns with required keys

Only allowed in:
- Match expressions
- Catch clauses

**Examples:**
```
// Irrefutable - always succeeds
var x = value
var [a, *rest] = list
var {name, _} = obj
fn f(x, [a, b]) { }

// Refutable - might fail
// var [x, y] = list      // Error if list doesn't have exactly 2 elements
// var {id: 123} = data   // Error: literal pattern in destructuring

// Match evaluation order
match [1, 2] {
    case [x, y] { "matched first" }  // This matches
    case [1, x] { "never reached" }
    case _ { "never reached" }
}

// Pattern evaluation
match 5 {
    case 1 { "small" }
    case 2 { "small" }
    case 3 { "small" }
    case 4 { "medium" }
    case 5 { "medium" }  // Matches here
    case 6 { "medium" }
    case _ { "large" }
}

// Failed match
var result = match value {
    case 1 { "one" }
    case 2 { "two" }
}
// result is unit if value is not 1 or 2
```


---

