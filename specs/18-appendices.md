
## A. Grammar Summary

This appendix provides a complete BNF-style grammar for RustLeaf. The grammar uses the following notation:
- `::=` means "is defined as"
- `|` separates alternatives
- `*` means zero or more repetitions
- `+` means one or more repetitions
- `?` means optional (zero or one)
- `( )` groups elements
- Terminal symbols are in quotes

```bnf
Program ::= ModuleItem*

ModuleItem ::= ImportStatement
            | FunctionDeclaration
            | ClassDeclaration
            | VariableDeclaration
            | Statement

ImportStatement ::= "use" ImportPath ImportClause? ";"
ImportPath ::= Identifier ("::" Identifier)*
ImportClause ::= "::" "*"
              | "::" "{" ImportList "}"
              | "::" Identifier
ImportList ::= Identifier ("," Identifier)* ","?

FunctionDeclaration ::= Visibility? "fn" Identifier "(" ParameterList? ")" Block
ParameterList ::= Parameter ("," Parameter)* ("," "*" Identifier)? ("," "**" Identifier)?
Parameter ::= Identifier ("=" Expression)?

ClassDeclaration ::= Visibility? "class" Identifier "{" ClassMember* "}"
ClassMember ::= Visibility? "var" Identifier ("=" Expression)? ";"
             | Visibility? "static"? FunctionDeclaration

Visibility ::= "pub"

VariableDeclaration ::= "var" Identifier ("=" Expression)? ";"

Statement ::= ExpressionStatement
           | VariableDeclaration
           | AssignmentStatement
           | Block
           | IfStatement
           | WhileStatement
           | ForStatement
           | MatchStatement
           | TryStatement
           | WithStatement
           | BreakStatement
           | ContinueStatement
           | ReturnStatement
           | EmptyStatement

ExpressionStatement ::= Expression ";"
EmptyStatement ::= ";"

Block ::= "{" Statement* "}"

IfStatement ::= "if" Expression Block ("else" "if" Expression Block)* ("else" Block)?

WhileStatement ::= "while" Expression Block

ForStatement ::= "for" Identifier ("," Identifier)? "in" Expression Block

MatchStatement ::= "match" Expression "{" MatchArm* "}"
MatchArm ::= "case" Pattern ("if" Expression)? Block

TryStatement ::= "try" Block "catch" Identifier Block

WithStatement ::= "with" WithBinding ("," WithBinding)* Block
WithBinding ::= Identifier "=" Expression

BreakStatement ::= "break" Expression? ";"
ContinueStatement ::= "continue" ";"
ReturnStatement ::= "return" Expression? ";"

AssignmentStatement ::= LValue AssignmentOperator Expression ";"

AssignmentOperator ::= "=" | "+=" | "-=" | "*=" | "/=" | "%="

LValue ::= Identifier
        | PostfixExpression "." Identifier
        | PostfixExpression "[" Expression "]"

Expression ::= ConditionalExpression

ConditionalExpression ::= LogicalOrExpression
                       | LogicalOrExpression "?" Expression ":" ConditionalExpression

LogicalOrExpression ::= LogicalXorExpression
                     | LogicalOrExpression "or" LogicalXorExpression

LogicalXorExpression ::= LogicalAndExpression
                      | LogicalXorExpression "xor" LogicalAndExpression

LogicalAndExpression ::= BitwiseOrExpression
                      | LogicalAndExpression "and" BitwiseOrExpression

BitwiseOrExpression ::= BitwiseXorExpression
                     | BitwiseOrExpression "|" BitwiseXorExpression

BitwiseXorExpression ::= BitwiseAndExpression
                      | BitwiseXorExpression "^" BitwiseAndExpression

BitwiseAndExpression ::= EqualityExpression
                      | BitwiseAndExpression "&" EqualityExpression

EqualityExpression ::= RelationalExpression
                    | EqualityExpression ("==" | "!=") RelationalExpression

RelationalExpression ::= ShiftExpression
                      | RelationalExpression ("<" | ">" | "<=" | ">=" | "in" | "is") ShiftExpression

ShiftExpression ::= AdditiveExpression
                 | ShiftExpression ("<<" | ">>") AdditiveExpression

AdditiveExpression ::= MultiplicativeExpression
                    | AdditiveExpression ("+" | "-") MultiplicativeExpression

MultiplicativeExpression ::= ExponentiationExpression
                          | MultiplicativeExpression ("*" | "/" | "%") ExponentiationExpression

ExponentiationExpression ::= UnaryExpression
                          | UnaryExpression "**" ExponentiationExpression

UnaryExpression ::= PostfixExpression
                 | ("+" | "-" | "not" | "~") UnaryExpression

PostfixExpression ::= PrimaryExpression
                   | PostfixExpression "." Identifier
                   | PostfixExpression "[" Expression "]"
                   | PostfixExpression "(" ArgumentList? ")"

ArgumentList ::= Argument ("," Argument)*
Argument ::= Expression
          | "*" Expression
          | "**" Expression

PrimaryExpression ::= Identifier
                   | Literal
                   | "(" Expression ")"
                   | ListLiteral
                   | DictLiteral
                   | BlockExpression
                   | IfExpression
                   | MatchExpression
                   | TryExpression
                   | AnonymousFunction

ListLiteral ::= "[" (Expression ("," Expression)* ","?)? "]"

DictLiteral ::= "{" (DictEntry ("," DictEntry)* ","?)? "}"
DictEntry ::= Expression ":" Expression

BlockExpression ::= Block

IfExpression ::= "if" Expression Block ("else" "if" Expression Block)* ("else" Block)?

MatchExpression ::= "match" Expression "{" MatchArm* "}"

TryExpression ::= "try" Block ("catch" Identifier Block)?

AnonymousFunction ::= "fn" "(" ParameterList? ")" Block

Pattern ::= LiteralPattern
         | VariablePattern
         | WildcardPattern
         | ListPattern
         | DictPattern
         | RangePattern
         | OrPattern

LiteralPattern ::= Literal

VariablePattern ::= Identifier

WildcardPattern ::= "_"

ListPattern ::= "[" (ListPatternElement ("," ListPatternElement)* ","?)? "]"
ListPatternElement ::= Pattern | "*" Identifier

DictPattern ::= "{" (DictPatternEntry ("," DictPatternEntry)* ","?)? "}"
DictPatternEntry ::= StringLiteral ":" Pattern

RangePattern ::= Expression ".." Expression
              | Expression "..=" Expression

OrPattern ::= Pattern ("|" Pattern)+

Literal ::= IntegerLiteral
         | FloatLiteral
         | StringLiteral
         | BooleanLiteral
         | NullLiteral

IntegerLiteral ::= DecimalLiteral | HexLiteral | OctalLiteral | BinaryLiteral
DecimalLiteral ::= [0-9]+
HexLiteral ::= "0x" [0-9a-fA-F]+
OctalLiteral ::= "0o" [0-7]+
BinaryLiteral ::= "0b" [01]+

FloatLiteral ::= [0-9]+ "." [0-9]+ ([eE] [+-]? [0-9]+)?
              | [0-9]+ [eE] [+-]? [0-9]+

StringLiteral ::= "\"" StringChar* "\""
               | "r\"" RawStringChar* "\""
StringChar ::= [^"\\\n] | EscapeSequence
RawStringChar ::= [^"]
EscapeSequence ::= "\\" ("n" | "t" | "r" | "\\" | "\"" | "'" | "0" | UnicodeEscape)
UnicodeEscape ::= "u" "{" [0-9a-fA-F]+ "}"

BooleanLiteral ::= "true" | "false"

NullLiteral ::= "null"

Identifier ::= [a-zA-Z_] [a-zA-Z0-9_]*

Comment ::= "//" [^\n]* "\n"
         | "/*" CommentChar* "*/"
CommentChar ::= [^*] | "*" [^/]

Whitespace ::= [ \t\n\r]+
```

## B. Operator Precedence

This appendix lists all operators in RustLeaf in order of precedence, from highest to lowest. Operators with the same precedence level are evaluated according to their associativity.

| Precedence | Operator(s) | Description | Associativity |
|------------|-------------|-------------|---------------|
| 1 (Highest) | `()` `[]` `.` | Function call, indexing, property access | Left-to-right |
| 2 | `+` `-` `not` `~` | Unary plus, minus, logical not, bitwise not | Right-to-left |
| 3 | `**` | Exponentiation | Right-to-left |
| 4 | `*` `/` `%` | Multiplication, division, modulo | Left-to-right |
| 5 | `+` `-` | Addition, subtraction | Left-to-right |
| 6 | `<<` `>>` | Bitwise left shift, right shift | Left-to-right |
| 7 | `&` | Bitwise AND | Left-to-right |
| 8 | `^` | Bitwise XOR | Left-to-right |
| 9 | `\|` | Bitwise OR | Left-to-right |
| 10 | `<` `<=` `>` `>=` `in` `is` | Comparison operators | Left-to-right |
| 11 | `==` `!=` | Equality operators | Left-to-right |
| 12 | `and` | Logical AND | Left-to-right |
| 13 | `xor` | Logical XOR | Left-to-right |
| 14 | `or` | Logical OR | Left-to-right |
| 15 (Lowest) | `?:` | Ternary conditional | Right-to-left |

### Precedence Examples

```
// Unary operators bind tighter than binary
-x + y      // Equivalent to: (-x) + y
not a and b // Equivalent to: (not a) and b

// Exponentiation is right-associative
2 ** 3 ** 2 // Equivalent to: 2 ** (3 ** 2) = 2 ** 9 = 512

// Multiplication before addition
2 + 3 * 4   // Equivalent to: 2 + (3 * 4) = 14

// Comparison before logical
x > 0 and y < 10  // Equivalent to: (x > 0) and (y < 10)

// Logical precedence: and, then xor, then or
a and b xor c or d  // Equivalent to: ((a and b) xor c) or d

// Ternary conditional is lowest precedence
x > 0 ? y + z : w   // Equivalent to: x > 0 ? (y + z) : w
```

### Associativity Rules

**Left-to-right associativity:**
- Binary operators of same precedence are evaluated left to right
- Example: `a - b + c` is equivalent to `(a - b) + c`

**Right-to-left associativity:**
- Unary operators, exponentiation, and ternary conditional
- Example: `a ** b ** c` is equivalent to `a ** (b ** c)`
- Example: `a ? b ? c : d : e` is equivalent to `a ? (b ? c : d) : e`

### Parentheses Override

Parentheses can be used to override the default precedence and associativity:

```
(2 + 3) * 4    // = 20 (not 14)
2 ** (3 ** 2)  // = 512 (explicit right-associativity)
(a ? b : c) + d  // Force ternary evaluation first
```

## C. Reserved Words

This appendix lists all reserved words (keywords) in RustLeaf. These identifiers cannot be used as variable names, function names, class names, or other user-defined identifiers.

### Control Flow Keywords
- `if` - Conditional expression/statement
- `else` - Alternative branch in conditional
- `match` - Pattern matching expression/statement
- `case` - Pattern case in match expression
- `while` - While loop statement
- `for` - For loop statement
- `in` - Iterator keyword in for loops and membership testing
- `break` - Break out of loop with optional value
- `continue` - Continue to next loop iteration
- `return` - Return from function with optional value

### Declaration Keywords
- `var` - Variable declaration
- `fn` - Function declaration
- `class` - Class declaration
- `static` - Static method/variable modifier
- `pub` - Public visibility modifier
- `use` - Module import statement

### Literal Keywords
- `true` - Boolean true literal
- `false` - Boolean false literal
- `null` - Null literal

### Exception Handling Keywords
- `try` - Try block for error handling
- `catch` - Catch block for error handling
- `raise` - Raise an error

### Resource Management Keywords
- `with` - Resource management statement

### Logical Operators
- `and` - Logical AND operator
- `or` - Logical OR operator
- `xor` - Logical XOR operator
- `not` - Logical NOT operator
- `is` - Identity comparison operator

### Module System Keywords
- `super` - Reference to parent module
- `root` - Reference to project root module

### Type System Keywords
- `self` - Reference to current object instance

### Macro Keywords
- `macro` - Macro definition (used in attributes)


### Contextual Keywords

Some identifiers have special meaning only in specific contexts and can be used as regular identifiers elsewhere:

- `new` - Common constructor method name (not reserved)
- `close` - Common cleanup method name (not reserved)
- `get` - Common getter method prefix (not reserved)
- `set` - Common setter method prefix (not reserved)

### Usage Notes

1. **Case Sensitivity**: All keywords are lowercase and case-sensitive. `If`, `IF`, or `iF` are not keywords and can be used as identifiers.

2. **No Raw Identifiers**: Unlike some languages, RustLeaf does not provide a mechanism to use keywords as identifiers (e.g., no `r#keyword` syntax).

3. **Future Compatibility**: Using any of the "Reserved for Future Use" keywords will result in a parse error, ensuring forward compatibility when these features are implemented.

4. **Operator Words**: The logical operators `and`, `or`, `xor`, `not`, and `is` are keywords rather than symbols to improve readability.

### Examples

```
// Valid identifiers (not keywords)
var data = 42
var user_name = "Alice"
var MyClass = SomeClass()

// Invalid identifiers (keywords)
// var if = 42        // Error: 'if' is a keyword
// var class = "foo"  // Error: 'class' is a keyword
// fn and() {}        // Error: 'and' is a keyword

// Contextual keywords can be used as identifiers
var new = 123          // Valid: 'new' is not a keyword
fn get_value() {}      // Valid: 'get' is not a keyword
var close = true       // Valid: 'close' is not a keyword
```

## D. Built-in Function Reference

This appendix provides a comprehensive reference for all built-in functions available in RustLeaf. These functions are automatically available in the global scope without requiring imports.

### Type Functions

#### `type(value)`
Returns the type name of a value as a string.

**Parameters:**
- `value`: Any value

**Returns:** String representing the type name

**Examples:**
```
type(42)           // "int"
type(3.14)         // "float"
type("hello")      // "string"
type(true)         // "bool"
type(null)         // "null"
type([1, 2, 3])    // "list"
type({"a": 1})     // "dict"
type(fn() {})      // "function"
```

### String Functions

#### `str(value)`
Converts a value to its string representation.

**Parameters:**
- `value`: Any value

**Returns:** String representation of the value

**Examples:**
```
str(42)            // "42"
str(3.14)          // "3.14"
str(true)          // "true"
str([1, 2, 3])     // "[1, 2, 3]"
str({"a": 1})      // "{\"a\": 1}"
```

### Numeric Functions

#### `int(value)`
Converts a value to an integer.

**Parameters:**
- `value`: Numeric value or string

**Returns:** Integer representation

**Raises:** Runtime error if conversion is not possible

**Examples:**
```
int(3.14)          // 3
int("42")          // 42
int("3.14")        // 3
int(true)          // 1
int(false)         // 0
```

#### `float(value)`
Converts a value to a floating-point number.

**Parameters:**
- `value`: Numeric value or string

**Returns:** Float representation

**Raises:** Runtime error if conversion is not possible

**Examples:**
```
float(42)          // 42.0
float("3.14")      // 3.14
float(true)        // 1.0
float(false)       // 0.0
```

### Collection Functions

#### `range(start, end, step?)`
Creates a sequence of numbers from start to end (exclusive).

**Parameters:**
- `start`: Starting value (inclusive)
- `end`: Ending value (exclusive)  
- `step`: Step size (optional, defaults to 1)

**Returns:** Range object (iterable)

**Examples:**
```
range(0, 5)        // 0, 1, 2, 3, 4
range(1, 10, 2)    // 1, 3, 5, 7, 9
range(10, 0, -1)   // 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
```

#### `enumerate(iterable)`
Returns enumerated pairs of (index, value) for an iterable.

**Parameters:**
- `iterable`: Any iterable value

**Returns:** Iterator of (index, value) pairs

**Examples:**
```
for i, item in enumerate(["a", "b", "c"]) {
    print(i, item)  // 0 a, 1 b, 2 c
}
```

### Error Functions

#### `raise(message_or_object)`
Raises a runtime error with the given message or error object.

**Parameters:**
- `message_or_object`: String message or error object

**Raises:** Always raises an error

**Examples:**
```
raise("Something went wrong")

raise({
    type: "ValidationError",
    message: "Invalid input",
    code: 400
})
```

#### `assert(condition, message?)`
Asserts that a condition is true, raising an error if false.

**Parameters:**
- `condition`: Boolean condition to check
- `message`: Optional error message (defaults to "Assertion failed")

**Raises:** Runtime error if condition is false

**Examples:**
```
assert(x > 0)
assert(list.len() > 0, "List cannot be empty")
assert(type(value) == "string", "Expected string value")
```

### Input/Output Functions

#### `print(...values)`
Prints values to standard output, separated by spaces.

**Parameters:**
- `...values`: Variable number of values to print

**Returns:** unit

**Examples:**
```
print("Hello")                    // Hello
print("x =", 42)                 // x = 42
print("Values:", 1, 2, 3)        // Values: 1 2 3
```

#### `input(prompt?)`
Reads a line of input from standard input.

**Parameters:**
- `prompt`: Optional prompt string to display

**Returns:** String containing the user input (without trailing newline)

**Examples:**
```
var name = input("Enter your name: ")
var age = int(input("Enter your age: "))
```

### Utility Functions

#### `len(collection)`
Returns the length of a collection. **Note:** This is available as both a global function and as methods on collections.

**Parameters:**
- `collection`: List, dict, or string

**Returns:** Integer length

**Examples:**
```
len([1, 2, 3])     // 3
len({"a": 1})      // 1
len("hello")       // 5

// Equivalent method calls:
[1, 2, 3].len()    // 3
{"a": 1}.len()     // 1
"hello".len()      // 5
```

#### `hash(value)`
Returns a hash value for the given value.

**Parameters:**
- `value`: Any hashable value

**Returns:** Integer hash value

**Examples:**
```
hash("hello")      // Some integer
hash(42)           // Some integer
hash([1, 2])       // Runtime error - lists are not hashable
```

#### `id(value)`
Returns a unique identifier for the value.

**Parameters:**
- `value`: Any value

**Returns:** Integer representing object identity

**Examples:**
```
var a = [1, 2, 3]
var b = [1, 2, 3]
var c = a

id(a) == id(c)     // true (same object)
id(a) == id(b)     // false (different objects)
```

#### `is_unit(value)`
Tests whether a value is the unit type.

**Parameters:**
- `value`: Any value

**Returns:** Boolean indicating if value is unit type

**Examples:**
```
// Check function returns
fn void_function() {
    print("side effect")
}
is_unit(void_function())  // true

// Check iterator completion
var next_value = iterator.op_next()
if is_unit(next_value) {
    print("Iterator exhausted")
}

is_unit(null)     // false
is_unit(42)       // false
```

### Global Variables

#### `args`
List of command-line arguments passed to the script.

**Type:** List of strings

**Example:**
```
// If script is run with: rustleaf script.rl arg1 arg2
print(args)  // ["script.rl", "arg1", "arg2"]
```

### Function Signatures Summary

```
// Type functions
type(value: any) -> string
str(value: any) -> string
int(value: any) -> int
float(value: any) -> float

// Collection functions  
range(start: int, end: int, step?: int) -> Range
enumerate(iterable: any) -> Iterator

// Error functions
raise(message_or_object: any) -> never
assert(condition: bool, message?: string) -> unit

// I/O functions
print(...values: any) -> unit
input(prompt?: string) -> string

// Utility functions
len(collection: list|dict|string) -> int
hash(value: any) -> int
id(value: any) -> int
is_unit(value: any) -> bool

// Global variables
args: list<string>
```

### Notes

1. **Availability**: All built-in functions are available in global scope without imports.

2. **Method Equivalents**: Some functions like `len()` are also available as methods on objects (e.g., `list.len()`).

3. **Error Handling**: Functions that can fail (like `int()`, `float()`) raise runtime errors rather than returning special values.

4. **Variadic Functions**: Functions like `print()` accept a variable number of arguments using the `...` syntax.

5. **Type Coercion**: Conversion functions perform explicit type coercion and may raise errors for invalid conversions.

## E. Error Codes

This appendix defines the standard error codes and error types used throughout RustLeaf. Understanding these codes helps in debugging and error handling.

### Error Categories

RustLeaf errors are categorized into several types, each with specific error codes and characteristics.

### Parse Errors (1000-1999)

Parse errors occur during the parsing phase when source code contains syntax errors.

#### 1001: Unexpected Token
**Description:** An unexpected token was encountered during parsing.
**Example:**
```
var x = 42 } // Error 1001: Unexpected token '}'
```

#### 1002: Unterminated String Literal  
**Description:** A string literal was not properly closed.
**Example:**
```
var msg = "Hello world // Error 1002: Unterminated string literal
```

#### 1003: Invalid Number Format
**Description:** A numeric literal has invalid syntax.
**Example:**
```
var x = 123.45.67 // Error 1003: Invalid number format
```

#### 1004: Invalid Character
**Description:** An invalid character was found in the source code.
**Example:**
```
var x = 42@  // Error 1004: Invalid character '@'
```

#### 1005: Mismatched Brackets
**Description:** Opening and closing brackets do not match.
**Example:**
```
var list = [1, 2, 3}  // Error 1005: Expected ']' but found '}'
```

#### 1006: Expected Expression
**Description:** An expression was expected but not found.
**Example:**
```
var x = +  // Error 1006: Expected expression after '+'
```

#### 1007: Invalid Assignment Target
**Description:** The left side of an assignment is not a valid target.
**Example:**
```
42 = x  // Error 1007: Invalid assignment target
```

### Runtime Errors (2000-2999)

Runtime errors occur during script execution.

#### 2001: Type Error
**Description:** An operation was attempted on an incompatible type.
**Example:**
```
"hello" + 42  // Error 2001: Cannot add string and int
```

#### 2002: Undefined Variable
**Description:** A variable was used before being declared.
**Example:**
```
print(undefined_var)  // Error 2002: Variable 'undefined_var' is not defined
```

#### 2003: Index Out of Bounds
**Description:** An index access was outside the valid range.
**Example:**
```
var list = [1, 2, 3]
print(list[5])  // Error 2003: Index 5 out of bounds for list of length 3
```

#### 2004: Key Not Found
**Description:** A dictionary key was not found.
**Example:**
```
var dict = {"a": 1}
print(dict["b"])  // Error 2004: Key 'b' not found in dict
```

#### 2005: Division by Zero
**Description:** An attempt was made to divide by zero.
**Example:**
```
var result = 10 / 0  // Error 2005: Division by zero
```

#### 2006: Invalid Function Call
**Description:** A non-callable value was called as a function.
**Example:**
```
var x = 42
x()  // Error 2006: Value of type 'int' is not callable
```

#### 2007: Wrong Number of Arguments
**Description:** A function was called with the wrong number of arguments.
**Example:**
```
fn add(a, b) { a + b }
add(1)  // Error 2007: Function 'add' expects 2 arguments, got 1
```

#### 2008: Attribute Not Found
**Description:** An attribute was accessed on an object that doesn't have it.
**Example:**
```
var obj = {}
print(obj.missing)  // Error 2008: Object has no attribute 'missing'
```

#### 2009: Immutable Assignment
**Description:** An attempt was made to modify an immutable value.
**Example:**
```
// This would apply if we had immutable values
```

#### 2010: Stack Overflow
**Description:** The call stack exceeded its maximum depth.
**Example:**
```
fn infinite_recursion() {
    infinite_recursion()
}
infinite_recursion()  // Error 2010: Maximum call stack depth exceeded
```

### Module Errors (3000-3999)

Module system related errors.

#### 3001: Module Not Found
**Description:** An imported module could not be found.
**Example:**
```
use non_existent_module::something;  // Error 3001: Module 'non_existent_module' not found
```

#### 3002: Circular Import
**Description:** A circular dependency was detected between modules.
**Example:**
```
// In module A: use B::something
// In module B: use A::something  
// Error 3002: Circular import detected: A -> B -> A
```

#### 3003: Export Not Found
**Description:** An imported name is not exported by the target module.
**Example:**
```
use some_module::private_function;  // Error 3003: 'private_function' is not exported by 'some_module'
```

#### 3004: Invalid Module Path
**Description:** A module path has invalid syntax.
**Example:**
```
use ::invalid::path;  // Error 3004: Invalid module path
```

### Pattern Matching Errors (4000-4999)

Errors related to pattern matching.

#### 4001: Pattern Match Failure
**Description:** No pattern matched the given value.
**Example:**
```
match value {
    case 1 { "one" }
    case 2 { "two" }
    // No default case
}
// Error 4001: No pattern matched value '3'
```

#### 4002: Invalid Pattern
**Description:** A pattern has invalid syntax or semantics.
**Example:**
```
match value {
    case [a, a] { /* ... */ }  // Error 4002: Variable 'a' appears multiple times in pattern
}
```

### Macro Errors (5000-5999)

Errors related to macro processing.

#### 5001: Macro Not Found
**Description:** A referenced macro was not defined.
**Example:**
```
#[undefined_macro]
fn test() {}  // Error 5001: Macro 'undefined_macro' not found
```

#### 5002: Macro Application Error
**Description:** A macro failed during application.
**Example:**
```
// If a macro raises an error during transformation
// Error 5002: Macro 'some_macro' failed: Invalid AST transformation
```

#### 5003: Invalid Macro Target
**Description:** A macro was applied to an invalid target.
**Example:**
```
#[test]
var x = 42;  // Error 5003: Macro 'test' cannot be applied to variable declarations
```

### User-Defined Errors (6000+)

Error codes 6000 and above are reserved for user-defined errors in scripts.

#### User Error Example
```
fn validate_age(age) {
    if age < 0 {
        raise({
            code: 6001,
            type: "ValidationError", 
            message: "Age cannot be negative",
            value: age
        })
    }
}
```

### Error Object Structure

All runtime errors in RustLeaf follow a consistent structure:

```
{
    code: int,           // Error code (as defined above)
    type: string,        // Error type name
    message: string,     // Human-readable error message
    line?: int,          // Line number where error occurred
    column?: int,        // Column number where error occurred
    file?: string,       // File where error occurred
    stack?: [string],    // Stack trace
    context?: dict       // Additional context information
}
```

### Error Handling Best Practices

1. **Specific Catching**: Catch specific error types when possible:
   ```
   try {
       risky_operation()
   } catch e {
       if e.code == 2003 {
           // Handle index out of bounds specifically
       } else {
           // Handle other errors
       }
   }
   ```

2. **Error Propagation**: Re-raise errors with additional context:
   ```
   try {
       dangerous_function()
   } catch e {
       raise({
           code: e.code,
           type: e.type,
           message: "Failed in process_data: " + e.message,
           original: e
       })
   }
   ```

3. **User-Friendly Messages**: Provide meaningful error messages:
   ```
   if age < 0 {
       raise({
           code: 6001,
           type: "ValidationError",
           message: "Age must be a positive number, got: " + str(age)
       })
   }
   ```

### Implementation Notes

1. **Error Codes**: Error codes are stable across versions and can be relied upon for programmatic error handling.

2. **Error Types**: Error type strings are also stable and provide a more readable way to categorize errors.

3. **Stack Traces**: Stack traces include function names and line numbers when available.

4. **Context Information**: The `context` field may contain additional debugging information specific to the error type.

## F. Implementation Limits

This appendix specifies the minimum requirements and limits that RustLeaf implementations must support. These limits ensure portability and predictable behavior across different implementations.

### Numeric Limits

#### Integer Limits
- **Minimum range**: -2^63 to 2^63-1 (64-bit signed integers)
- **Maximum value**: 9,223,372,036,854,775,807
- **Minimum value**: -9,223,372,036,854,775,808
- **Overflow behavior**: Runtime error on overflow/underflow

#### Floating-Point Limits
- **Precision**: IEEE 754 double precision (64-bit)
- **Maximum finite value**: Approximately 1.7976931348623157e+308
- **Minimum positive value**: Approximately 2.2250738585072014e-308
- **Special values**: Supports NaN and infinity
- **Precision**: 15-17 significant decimal digits

### String Limits

#### String Length
- **Maximum length**: 2^32-1 characters (4,294,967,295)
- **Character encoding**: UTF-8
- **Memory limit**: Limited by available system memory

#### String Literals
- **Maximum literal length**: 2^16-1 characters (65,535) in source code
- **Escape sequences**: Must support all standard escape sequences
- **Unicode support**: Full Unicode support via UTF-8 encoding

### Collection Limits

#### List Limits
- **Maximum elements**: 2^32-1 elements (4,294,967,295)
- **Nesting depth**: Limited by available stack space (typically 1000+ levels)
- **Element types**: Any valid RustLeaf value
- **Memory limit**: Limited by available system memory

#### Dictionary Limits
- **Maximum entries**: 2^32-1 key-value pairs (4,294,967,295)
- **Key types**: Strings, integers, floats, booleans (hashable types only)
- **Value types**: Any valid RustLeaf value
- **Memory limit**: Limited by available system memory

### Identifier Limits

#### Variable and Function Names
- **Maximum length**: 1024 characters
- **Character set**: Unicode letters, digits, and underscores
- **First character**: Must be letter or underscore
- **Case sensitivity**: Case-sensitive

#### Module Path Limits
- **Maximum path depth**: 32 levels (e.g., `a::b::c::...`)
- **Maximum component length**: 256 characters per path component
- **Total path length**: 2048 characters maximum

### Call Stack Limits

#### Function Calls
- **Maximum call depth**: 1000 function calls
- **Recursion limit**: Same as call depth (1000)
- **Overflow behavior**: Runtime error with stack overflow message

#### Parameter Limits
- **Maximum parameters per function**: 255 parameters
- **Maximum arguments per call**: 255 arguments
- **Variadic arguments**: No limit beyond memory constraints

### Source Code Limits

#### File Size
- **Maximum file size**: 64 MB per source file
- **Maximum lines**: 2,000,000 lines per file
- **Line length**: No specific limit (limited by memory)

#### Parsing Limits
- **Maximum nesting depth**: 256 levels (blocks, expressions, etc.)
- **Maximum tokens per file**: 16,777,216 (2^24)
- **Comment length**: No specific limit

### Memory Limits

#### Heap Memory
- **Available memory**: Limited by system available memory
- **Garbage collection**: Implementation-defined trigger points
- **Memory growth**: Must handle graceful degradation on memory pressure

#### Variable Scope
- **Maximum variables per scope**: 65,536 variables
- **Maximum scopes**: Limited by call stack depth
- **Closure captures**: Limited by memory availability

### Execution Limits

#### Concurrency
- **Thread safety**: Single-threaded execution model
- **Reentrancy**: Not supported

### Error Handling Limits

#### Error Propagation
- **Maximum error chain depth**: 1000 nested errors
- **Error message length**: 4096 characters maximum
- **Stack trace depth**: Limited by call stack depth

#### Exception Context
- **Context data size**: 1 MB maximum per error object
- **Nested error objects**: Maximum depth of 100 levels

### File System Limits

#### Module Loading
- **Maximum modules per program**: 10,000 modules
- **Module cache size**: Implementation-defined
- **File system access**: Subject to OS limits

#### Import Resolution
- **Maximum import depth**: 100 levels of imports
- **Circular import detection**: Must detect within 1000 import attempts

### Platform-Specific Considerations

#### Minimum System Requirements
- **Available RAM**: 64 MB minimum for basic scripts
- **Stack size**: 8 MB minimum stack space
- **File handles**: Ability to open 100+ files simultaneously

#### Performance Requirements
- **Startup time**: Must start within 1 second on reference hardware
- **Memory overhead**: Maximum 10 MB base memory usage
- **Garbage collection**: Maximum 10ms pause times for small heaps

### Conformance Testing

Implementations should provide mechanisms to verify these limits:

```
// Example limit verification functions
fn test_limits() {
    // Test maximum integer
    var max_int = 9_223_372_036_854_775_807
    assert(type(max_int) == "int")
    
    // Test string length
    var long_string = "a".repeat(1000)
    assert(long_string.len() == 1000)
    
    // Test call stack depth
    fn recursive_test(depth) {
        if depth <= 0 {
            return depth
        }
        return recursive_test(depth - 1)
    }
    
    // This should work without stack overflow
    assert(recursive_test(500) == 0)
}
```

### Implementation Flexibility

While these limits define minimum requirements, implementations may:

1. **Exceed Limits**: Provide higher limits where feasible
2. **Configuration**: Allow limits to be configured by users
3. **Dynamic Limits**: Adjust limits based on available resources
4. **Platform Optimization**: Optimize for specific platforms while maintaining minimums

### Limit Violation Behavior

When limits are exceeded, implementations must:

1. **Graceful Degradation**: Provide clear error messages
2. **Resource Cleanup**: Clean up allocated resources
3. **Error Reporting**: Include relevant context in error messages
4. **No Corruption**: Never corrupt data or state when limits are hit

### Examples of Limit Errors

```
// Stack overflow
fn infinite_recursion() {
    infinite_recursion()
}
// Error: Maximum call stack depth (1000) exceeded

// String too long in source
var huge_string = "very long string literal..."  // > 65535 chars
// Error: String literal exceeds maximum length (65535 characters)

// Too many function parameters
fn too_many_params(p1, p2, /* ... */, p256) {}  // > 255 params
// Error: Function has too many parameters (maximum 255)
```

These limits ensure that RustLeaf implementations are both practical and predictable across different platforms and use cases.