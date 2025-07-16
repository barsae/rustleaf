# RustLeaf Language Specification

Version 1.0

## Table of Contents

### 1. Introduction
[1.1. Scope](#11-scope)  
[1.2. Conformance](#12-conformance)  
[1.3. Normative References](#13-normative-references)  
[1.4. Terms and Definitions](#14-terms-and-definitions)  

### 2. Lexical Structure
[2.1. Source Text](#21-source-text)  
[2.2. Character Set](#22-character-set)  
[2.3. Lexical Analysis](#23-lexical-analysis)  
[2.4. Tokens](#24-tokens)  
[2.5. Comments](#25-comments)  
[2.6. Whitespace](#26-whitespace)  
[2.7. Line Terminators](#27-line-terminators)  
[2.8. Identifiers](#28-identifiers)  
[2.9. Keywords](#29-keywords)  
[2.10. Literals](#210-literals)  
   [2.10.1. Integer Literals](#2101-integer-literals)  
   [2.10.2. Floating-Point Literals](#2102-floating-point-literals)  
   [2.10.3. String Literals](#2103-string-literals)  
   [2.10.4. Boolean Literals](#2104-boolean-literals)  
   [2.10.5. Null Literal](#2105-null-literal)  
   [2.10.6. Raw String Literals](#raw-string-literals)  

### 3. Types
[3.1. Type System Overview](#31-type-system-overview)  
[3.2. Primitive Types](#32-primitive-types)  
   [3.2.1. Null Type](#321-null-type)  
   [3.2.2. Boolean Type](#322-boolean-type)  
   [3.2.3. Numeric Types](#323-numeric-types)  
   [3.2.4. String Type](#324-string-type)  
[3.3. Composite Types](#33-composite-types)  
   [3.3.1. List Type](#331-list-type)  
   [3.3.2. Dict Type](#332-dict-type)  
   [3.3.3. Object Type](#333-object-type)  
[3.4. Function Types](#34-function-types)  
[3.5. RustValue Type](#35-rustvalue-type)  
[3.6. Type Conversions](#36-type-conversions)  
[3.7. Type Checking](#37-type-checking)  

### 4. Variables
4.1. Variable Declarations  
4.2. Variable Initialization  
4.3. Scope and Lifetime  
4.4. Assignment  

### 5. Expressions
5.1. Expression Evaluation  
5.2. Primary Expressions  
   5.2.1. Literals  
   5.2.2. Identifiers  
   5.2.3. Parenthesized Expressions  
5.3. Postfix Expressions  
   5.3.1. Property Access  
   5.3.2. Index Access  
   5.3.3. Function Call  
   5.3.4. Method Call  
5.4. Unary Expressions  
5.5. Binary Expressions  
   5.5.1. Arithmetic Operators  
   5.5.2. Comparison Operators  
   5.5.3. Logical Operators  
   5.5.4. Bitwise Operators  
5.6. Assignment Expressions  
5.7. Conditional Expressions (if)  
5.8. Match Expressions  
5.9. Try Expressions  
5.10. Block Expressions  
5.11. Anonymous Function Expressions  
5.12. Object Literal Expressions  
5.13. List Literal Expressions  
5.14. Dict Literal Expressions  

### 6. Statements
6.1. Statement Evaluation  
6.2. Expression Statements  
6.3. Variable Declaration Statements  
6.4. Empty Statements  
6.5. Block Statements  
6.6. Control Flow Statements  
   6.6.1. If Statements  
   6.6.2. While Statements  
   6.6.3. For Statements  
   6.6.4. Match Statements  
   6.6.5. Try-Catch-Finally Statements  
   6.6.6. With Statements  
   6.6.7. Break Statements  
   6.6.8. Continue Statements  
   6.6.9. Return Statements  

### 7. Functions
7.1. Function Declarations  
7.2. Function Parameters  
7.3. Function Body  
7.4. Return Values  
7.5. Closures  
7.6. Anonymous Functions  
7.7. Function Scope  
7.8. Recursion  

### 8. Pattern Matching
8.1. Pattern Syntax  
8.2. Literal Patterns  
8.3. Variable Patterns  
8.4. Wildcard Patterns  
8.5. List Patterns  
8.6. Dict Patterns  
8.7. Range Patterns  
8.8. Guard Expressions  
8.9. Pattern Evaluation  

### 9. Error Handling
9.1. Error Types  
9.2. Raising Errors  
9.3. Try-Catch Blocks  
9.4. Finally Blocks  
9.5. Error Propagation  
9.6. Error Objects  

### 10. Modules
10.1. Module System Overview  
10.2. Import Statements  
10.3. Export Functions  
10.4. Module Resolution  
10.5. Module Loading  
10.6. Module Scope  

### 11. Built-in Functions
11.1. Type Functions  
11.2. Collection Functions  
11.3. String Functions  
11.4. I/O Functions  
11.5. Error Functions  
11.6. Module Functions  

### 12. Standard Library
12.1. String Methods  
12.2. List Methods  
12.3. Dict Methods  
12.4. Object Methods  

### 13. Documentation Comments and Docstrings
13.1. Overview  
13.2. Documentation Comment Syntax  
13.3. Docstring Syntax  
13.4. Function Docstrings  
13.5. Module Docstrings  
13.6. Variable and Constant Docstrings  
13.7. Object and Type Docstrings  
13.8. Docstring Format Conventions  
13.9. Runtime Access to Docstrings  
13.10. Tooling Integration  

### 14. Memory Model
14.1. Value Semantics  
14.2. Reference Semantics  
14.3. Garbage Collection  
14.4. Resource Management  

### 15. Execution Model
15.1. Program Execution  
15.2. Expression Evaluation Order  
15.3. Function Call Semantics  
15.4. Exception Handling  
15.5. Resource Cleanup  

### 16. RustValue Integration
16.1. RustValue Trait  
16.2. Value Enum  
16.3. Field Access  
16.4. Method Dispatch  
16.5. Type Coercion  
16.6. Lifetime Management  

### Appendices
A. Grammar Summary  
B. Operator Precedence  
C. Reserved Words  
D. Built-in Function Reference  
E. Error Codes  
F. Implementation Limits  

---

## 1. Introduction

RustLeaf is a lightweight, dynamically-typed scripting language designed to be embedded within the editor application. It provides a Rust-inspired syntax with expression-oriented semantics, enabling users to extend and automate the editor through scripts.

The language serves multiple purposes:
- **Automated testing** of editor features, allowing comprehensive test suites to verify editor functionality
- **Task automation** for repetitive editing tasks and workflow optimization  
- **Configuration** through code, enabling dynamic and conditional editor settings
- **Plugin development** to extend editor capabilities with custom functionality

RustLeaf scripts use the `.rustleaf` file extension and can be executed in several contexts:
- Embedded within the editor binary as part of the standard library
- Loaded from a `.rustleaf` configuration file in the user's home directory
- Explicitly executed via command-line arguments for automation and testing
- Evaluated interactively in the editor's integrated REPL

This specification defines the complete syntax, semantics, and runtime behavior of RustLeaf. All behavior is fully specified to ensure consistent implementation and reliable execution across different contexts.

### 1.1. Scope

This specification describes the syntax, semantics, and runtime behavior of the RustLeaf programming language version 1.0. It defines:

- The lexical structure and grammar of valid RustLeaf programs
- The type system and value semantics
- Expression evaluation and control flow
- The module system and standard library
- Integration with Rust through the RustValue trait
- Error handling and resource management

This specification does not define:
- The editor API accessible to RustLeaf scripts
- Performance characteristics or optimization strategies
- Debugging interfaces or development tools
- Binary format or bytecode representation

**Note:** This specification is subject to breaking changes in future versions as the language evolves.

### 1.2. Conformance

A conforming implementation of RustLeaf must:

1. **Implement all language features** exactly as specified in this document
2. **Report all errors** as specified, with error messages that clearly identify the nature and location of the error
3. **Provide all built-in functions** with the exact signatures and behaviors defined
4. **Follow the specified evaluation order** for all expressions and statements
5. **Handle Unicode text** according to the requirements in Section 2.2

Conforming implementations must not:
- Add additional keywords or operators not defined in this specification
- Modify the behavior of any specified feature
- Introduce additional built-in functions or types
- Allow syntax not explicitly permitted by the grammar

Any deviation from this specification renders an implementation non-conformant.

### 1.3. Normative References

The following documents are referenced by this specification:

- **Unicode Standard, Version 15.0**: The Unicode Consortium. The Unicode Standard, Version 15.0.0, (Mountain View, CA: The Unicode Consortium, 2022. ISBN 978-1-936213-32-0)
  - Used for: Character encoding, string normalization, and text processing

- **IEEE 754-2019**: IEEE Standard for Floating-Point Arithmetic
  - Referenced for: Floating-point number representation and operations (as implemented by Rust f64)

- **Rust Language Semantics**: The semantics of Rust f64 operations and type conversions
  - Used for: Ensuring floating-point behavior matches the host language

### 1.4. Terms and Definitions

For the purposes of this specification, the following terms and definitions apply:

**Chapter**: A major division of this specification, identified by a single number (e.g., Chapter 5)

**Section**: A subdivision of a chapter, identified by two numbers separated by a period (e.g., Section 5.3)

**Subsection**: A subdivision of a section, identified by three numbers separated by periods (e.g., Subsection 5.3.1)

**Clause**: A subdivision of a subsection, identified by four numbers separated by periods (e.g., Clause 5.3.1.1)

**must**: Indicates an absolute requirement. Implementations that do not follow requirements marked with "must" are non-conformant.

**must not**: Indicates an absolute prohibition. Implementations that perform actions marked "must not" are non-conformant.

**error**: A condition that prevents the successful execution of a program. All errors must be reported to the user with sufficient information to identify their cause and location.

**value**: An instance of one of RustLeaf's types (null, boolean, integer, float, string, list, dict, function, object, or RustValue).

**expression**: A syntactic construct that evaluates to a value.

**statement**: A syntactic construct that performs an action but does not produce a value (though it may contain expressions).

**scope**: A region of program text where a binding is visible.

**identifier**: A sequence of characters used to name variables, functions, and fields.

**token**: A lexical unit produced by tokenizing source text.

**evaluation**: The process of computing the value of an expression.

**runtime**: The period during which a RustLeaf program is executing.

**host**: The Rust application (editor) that embeds the RustLeaf interpreter.

**module**: A RustLeaf source file that can export values for use by other modules.

**REPL**: Read-Eval-Print Loop - the interactive RustLeaf environment within the editor.

---

## 2. Lexical Structure

This chapter defines how RustLeaf source text is converted into a sequence of tokens. The lexical grammar operates on the individual Unicode code points of the source text and produces tokens that serve as input to the syntactic grammar.

### 2.1. Source Text

RustLeaf source text consists of a sequence of Unicode code points encoded in UTF-8. Source files must use the `.rustleaf` extension.

**Encoding Requirements:**
- Source files must be encoded in UTF-8 without a byte order mark (BOM)
- If a BOM (U+FEFF) is present at the beginning of a file, it must be ignored
- Invalid UTF-8 sequences must result in a lexical error

**File Size:**
- There is no fixed limit on source file size
- Implementations should issue a warning for files larger than 10 MB
- Implementations must handle files up to at least 100 MB

**Source Text Processing:**
1. The source text is decoded from UTF-8 into a sequence of Unicode code points
2. Line terminators are normalized (see Section 2.7)
3. The resulting sequence is tokenized according to the lexical grammar

### 2.2. Character Set

RustLeaf source text may contain any valid Unicode code point (U+0000 through U+10FFFF), with the following restrictions:

**String and Comment Contents:**
- May contain any Unicode code point except:
  - Unescaped line terminators in regular strings
  - The specific terminating sequence in multi-line strings (`"""`)
  - Invalid UTF-8 sequences

**Identifiers:**
- Must use only ASCII letters (a-z, A-Z), ASCII digits (0-9), and underscore (_)
- See Section 2.8 for detailed identifier rules

**Other Contexts:**
- Keywords, operators, and punctuation must use only ASCII characters
- Whitespace may include Unicode space characters (category Zs)

**Normalization:**
- Source text is not automatically normalized
- Code points are compared using their exact scalar values
- Identifiers that differ only in normalization form are considered distinct

**Examples:**
```
// Valid: ASCII identifier
var count = 42

// Valid: Unicode in strings
var greeting = "Hello, 世界! 🌍"

// Valid: Unicode in comments
// This is a comment with émojis 🎉

// Invalid: Unicode in identifier
// var café = "coffee"  // Error: non-ASCII in identifier
```

### 2.3. Lexical Analysis

Lexical analysis converts source text into a sequence of tokens. The process is greedy—at each point, the longest valid token is consumed.

**Lexical Analysis Process:**
1. Skip whitespace and comments (unless preserving for tooling)
2. Attempt to match the longest valid token starting at the current position
3. If no valid token matches, report a lexical error
4. Continue until end of input

**Error Handling:**
- Lexical errors do not stop analysis
- After an error, recovery attempts from the next code point
- All lexical errors are collected and reported
- Each error includes line number, column number, and byte offset

**Token Stream:**
The lexer produces a stream of tokens, where each token contains:
- Token type (keyword, identifier, literal, operator, etc.)
- Lexeme (the actual text)
- Source location (line, column, byte offset)
- For literals: parsed value

**Ambiguity Resolution:**
When multiple token types could match, precedence is:
1. Keywords
2. Literals
3. Identifiers
4. Operators (longest match)

### 2.4. Tokens

Tokens are the atomic lexical elements of RustLeaf programs. The following token categories exist:

**Token Categories:**
1. **Keywords** - Reserved words with special meaning
2. **Identifiers** - Names for variables, functions, etc.
3. **Literals** - Integer, float, string, boolean, and null values
4. **Operators** - Arithmetic, logical, comparison, and other operators
5. **Punctuation** - Delimiters and separators
6. **Whitespace** - Spaces, tabs, and line terminators (usually ignored)
7. **Comments** - Single-line and multi-line comments (usually ignored)

**Token Structure:**
```
Token {
    type: TokenType,
    lexeme: String,
    line: usize,
    column: usize,
    byte_offset: usize,
    value: Option<Value>,  // For literals
}
```

**Token Types:**
```
TokenType =
    // Keywords (see Section 2.9)
    | Var | Fn | If | Else | While | For | Match | Case | Try | Catch 
    | Finally | With | Break | Continue | Return | Class | Static | Self
    | Use | Pub | Raise | And | Or | Not | In | Is | True | False | Null
    
    // Identifiers
    | Identifier
    
    // Literals
    | IntegerLiteral | FloatLiteral | StringLiteral 
    | BooleanLiteral | NullLiteral
    
    // Operators
    | Plus | Minus | Star | Slash | Percent | StarStar
    | Equal | PlusEqual | MinusEqual | StarEqual | SlashEqual | PercentEqual
    | EqualEqual | BangEqual | Less | Greater | LessEqual | GreaterEqual
    | Ampersand | Pipe | Caret | Tilde | LessLess | GreaterGreater
    
    // Punctuation
    | LeftParen | RightParen | LeftBrace | RightBrace 
    | LeftBracket | RightBracket
    | Comma | Dot | Colon | DoubleColon | Semicolon | Arrow
    
    // Special
    | Newline | Eof
```

### 2.5. Comments

Comments are lexical elements that are ignored during parsing but may be preserved for documentation tools.

**Single-line Comments:**
- Begin with `//` and extend to the end of the line
- The `//` and all following characters until a line terminator are ignored
- The line terminator itself is not part of the comment

**Multi-line Comments:**
- Begin with `/*` and end with `*/`
- May span multiple lines
- Support nesting: `/* outer /* inner */ outer */`
- Nesting depth is tracked; each `/*` increments depth, each `*/` decrements
- An error occurs if `*/` appears without matching `/*`

**Documentation Comments:**
- Single-line: `///` followed by documentation text
- Multi-line: `/** */` with documentation text
- Must immediately precede the documented item (no blank lines)
- Preserved in the AST for tooling and runtime access

**Examples:**
```
// This is a single-line comment
var x = 42  // This is an end-of-line comment

/* This is a
   multi-line comment */

/* Nested /* comments */ are supported */

/// Documentation for the function
/// Can span multiple lines
fn calculate(x) {
    /** 
     * Block documentation comment
     * with multiple lines
     */
    x * 2
}
```

### 2.6. Whitespace

Whitespace characters separate tokens but are otherwise insignificant (except within string literals).

**Whitespace Characters:**
- Space (U+0020)
- Horizontal tab (U+0009)
- Line terminators (see Section 2.7)
- Any Unicode code point in category Zs (space separators)

**Whitespace Handling:**
- Consecutive whitespace characters are equivalent to a single space
- Whitespace is required between tokens that would otherwise form a different token
- Whitespace is not significant for indentation or layout
- Preserved within string literals exactly as written

**Examples:**
```
// These are equivalent:
var x=42
var   x   =   42
var x = 42

// Whitespace required to separate tokens:
varx = 42     // Error: 'varx' is one identifier
var x = 42    // Correct: 'var' keyword and 'x' identifier
```

### 2.7. Line Terminators

Line terminators end a line of source text and affect line numbering for error reporting.

**Line Terminator Sequences:**
- Line Feed: U+000A (LF, `\n`)
- Carriage Return: U+000D (CR, `\r`)  
- Carriage Return + Line Feed: U+000D U+000A (CRLF, `\r\n`)

**Line Terminator Normalization:**
- CRLF sequences are treated as a single line terminator
- Each LF, CR, or CRLF increments the line number by one
- Column numbers reset to 1 after a line terminator

**Line Terminator Handling:**
- No line continuation syntax (backslash at end of line has no special meaning)
- Line terminators are significant only in:
  - Single-line comments (terminate the comment)
  - Regular string literals (must be escaped)
  - Error reporting (determine line numbers)

### 2.8. Identifiers

Identifiers name variables, functions, parameters, fields, and other program entities.

**Identifier Syntax:**
```
Identifier = IdentifierStart IdentifierContinue*
IdentifierStart = Letter | "_"
IdentifierContinue = Letter | Digit | "_"
Letter = "a"..."z" | "A"..."Z"
Digit = "0"..."9"
```

**Identifier Rules:**
- Must start with an ASCII letter (a-z, A-Z) or underscore (_)
- May continue with ASCII letters, ASCII digits (0-9), or underscores
- Case-sensitive: `foo`, `Foo`, and `FOO` are different identifiers
- No length limit, but implementations may warn for identifiers over 255 characters

**Reserved Patterns:**
- A single underscore `_` is reserved as a wildcard pattern
- Identifiers cannot be keywords (see Section 2.9)

**Examples:**
```
// Valid identifiers:
var name = "Alice"
var _private = 42
var camelCase = true
var snake_case = false
var CONSTANT = 3.14
var x123 = "mixed"
var __internal__ = "ok"

// Invalid identifiers:
// var 123abc = "bad"     // Cannot start with digit
// var my-var = "bad"     // Hyphen not allowed
// var var = "bad"        // Cannot use keyword
```

### 2.9. Keywords

Keywords are reserved identifiers with special syntactic meaning. They cannot be used as regular identifiers.

**Complete Keyword List:**
```
and         else        if          not         self        use
break       false       in          null        static      var
case        finally     is          or          true        while
catch       fn          match       pub         try         with
class       for         not         raise       return
continue    from        of          require     super
```

**Keyword Properties:**
- All keywords are fully reserved in all contexts
- Keywords are case-sensitive (e.g., `If` is an identifier, not a keyword)
- No context-sensitive keywords exist

**Future Reserved Words:**
The following words are reserved for potential future use:
```
async       await       const       enum        impl        
interface   let         module      trait       type        
where       yield
```

### 2.10. Literals

Literals represent constant values directly in source code.

#### 2.10.1. Integer Literals

Integer literals represent 64-bit signed integer values.

**Syntax:**
```
IntegerLiteral = DecimalLiteral | HexLiteral | OctalLiteral | BinaryLiteral
DecimalLiteral = DecimalDigit (DecimalDigit | "_")*
HexLiteral = "0x" HexDigit (HexDigit | "_")*
OctalLiteral = "0o" OctalDigit (OctalDigit | "_")*
BinaryLiteral = "0b" BinaryDigit (BinaryDigit | "_")*

DecimalDigit = "0"..."9"
HexDigit = "0"..."9" | "a"..."f" | "A"..."F"
OctalDigit = "0"..."7"
BinaryDigit = "0" | "1"
```

**Rules:**
- Underscores can appear between digits for readability
- Leading zeros in decimal literals are not allowed (except for 0 itself)
- Values must fit in a signed 64-bit integer (-2^63 to 2^63-1)
- Overflow is a lexical error

**Examples:**
```
42          // Decimal
1_000_000   // Decimal with separators
0xFF        // Hexadecimal (255)
0xff        // Hexadecimal (255)
0o77        // Octal (63)
0b1010      // Binary (10)
0b1111_0000 // Binary with separator (240)

// Errors:
// 012       // Leading zero not allowed
// 0x_FF     // Underscore cannot follow prefix
// 1__000    // Consecutive underscores not allowed
// 1_        // Trailing underscore not allowed
```

#### 2.10.2. Floating-Point Literals

Floating-point literals represent IEEE 754 double-precision (64-bit) values.

**Syntax:**
```
FloatLiteral = DecimalFloat | ScientificFloat
DecimalFloat = DecimalDigits "." DecimalDigits?
             | DecimalDigits "."
             | "." DecimalDigits
ScientificFloat = (DecimalFloat | DecimalDigits) ("e" | "E") ("+" | "-")? DecimalDigits
DecimalDigits = DecimalDigit (DecimalDigit | "_")*
```

**Rules:**
- Must contain either a decimal point or scientific notation
- Underscores can appear between digits
- Values use IEEE 754 double-precision representation
- Special values: `Infinity`, `-Infinity`, `NaN` result from operations, not literals

**Examples:**
```
3.14159
1.0
0.1
.5              // Leading zero optional
42.             // Trailing digits optional
1_234.567_890   // With separators
1e10            // Scientific notation
2.5e-4          // 0.00025
1E+6            // 1000000.0

// Errors:
// 1._23        // Underscore after decimal point
// 1.2_         // Trailing underscore
// 1.2e_3       // Underscore after 'e'
```

#### 2.10.3. String Literals

String literals represent sequences of Unicode characters.

**Regular String Literals:**
- Enclosed in double quotes: `"..."`
- Cannot contain unescaped line terminators
- Support escape sequences and interpolation

**Triple-Quoted String Literals:**
- Enclosed in triple double quotes: `"""..."""`
- Can span multiple lines
- Line terminators are preserved as part of the string
- No escape sequences except `\"""` for literal triple quotes
- No interpolation

**Escape Sequences (regular strings only):**
```
\n          Line feed (U+000A)
\r          Carriage return (U+000D)
\t          Horizontal tab (U+0009)
\\          Backslash (U+005C)
\"          Double quote (U+0022)
\'          Single quote (U+0027)
\$          Dollar sign (U+0024)
\{          Left brace (U+007B)
\}          Right brace (U+007D)
\u{XXXXXX}  Unicode code point (1-6 hex digits)
```

**String Interpolation (regular strings only):**
- `${expression}` embeds the result of an expression
- Expression is evaluated and converted to string
- To include literal `${`, use `\${`

**Examples:**
```
// Regular strings
"Hello, world!"
"Line 1\nLine 2"
"Unicode: \u{1F604}"  // 😄
"Path: C:\\Users\\Name"
"Interpolation: ${2 + 2} equals 4"
"\${not interpolated}"

// Triple-quoted strings
"""This is a
multi-line string
with preserved formatting"""

"""No \n escape sequences"""
"""Include \""" by escaping"""

// Errors:
// "Unterminated
// "Invalid escape \x"
```

#### 2.10.4. Boolean Literals

```
BooleanLiteral = "true" | "false"
```

Boolean literals represent the two boolean values. They are keywords and cannot be used as identifiers.

#### 2.10.5. Null Literal

```
NullLiteral = "null"
```

The null literal represents the absence of a value. It is a keyword and cannot be used as an identifier.

### Raw String Literals

Raw string literals are enclosed in `r"..."` and treat backslashes literally:

**Syntax:**
```
RawStringLiteral = 'r"' RawStringContent '"'
RawStringContent = any character except '"' or CR or LF
```

**Properties:**
- No escape sequences are processed
- Cannot contain line terminators
- Cannot contain unescaped double quotes
- Useful for regular expressions and paths

**Examples:**
```
r"C:\Users\Name\Documents"  // Backslashes are literal
r"\n is not a newline"      // \n is two characters
r"Regex: \d{3}-\d{4}"       // Useful for regex patterns

// For quotes in raw strings, use regular strings:
"He said \"Hello\""         // Regular string with escaped quotes
```

---

## 3. Types

RustLeaf is a dynamically typed language where types are determined and checked at runtime. This chapter defines the type system, including primitive types, composite types, and the rules governing type conversions and type checking.

### 3.1. Type System Overview

RustLeaf employs dynamic typing with strong type checking at runtime. Variables can hold values of any type, and the type of a value is determined when it is created.

**Core Type System Properties:**
- **Dynamic Typing**: Variables do not have declared types; they acquire the type of their assigned value
- **Strong Typing**: Operations check types at runtime and raise errors for invalid type combinations
- **No Implicit Conversions**: Values are never automatically converted between types (except for string interpolation and display)
- **Runtime Type Information**: Full type information is available at runtime via the `type()` function

**Type Categories:**
1. **Primitive Types**: null, bool, int, float, string
2. **Composite Types**: list, dict, object (class instances)
3. **Callable Types**: function (including closures and methods)
4. **Extension Types**: RustValue (Rust-implemented types)

**Type Identity:**
- Each value has exactly one type
- Types are identified by their type name (a string)
- Type names are compared using string equality

**Example:**
```
var x = 42          // x holds an int
var t = type(x)     // t is "int"
x = "hello"         // x now holds a string
print(type(x))      // "string"

// Type checking
if type(value) == "int" {
    // value is definitely an integer
}
```

### 3.2. Primitive Types

Primitive types are the fundamental building blocks of the type system. They are immutable (except for their container in variables) and have value semantics.

#### 3.2.1. Null Type

The null type has exactly one value: `null`.

**Properties:**
- Type name: `"null"`
- Represents the absence of a value
- Used as a default return value when no explicit value is provided
- Truthiness: `null` is falsy in boolean contexts

**Operations:**
- Equality: `null == null` is `true`
- Type check: `type(null) == "null"`
- No other operations are valid on null

**Example:**
```
var x = null
print(type(x))      // "null"
if not x {          // null is falsy
    print("x is null")
}
```

#### 3.2.2. Boolean Type  

The boolean type represents logical truth values.

**Properties:**
- Type name: `"bool"`
- Exactly two values: `true` and `false`
- Result type of comparison and logical operations
- Truthiness: `false` is falsy, `true` is truthy

**Operations:**
- Logical: `and`, `or`, `not`
- Equality: `==`, `!=`
- Type check: `type(true) == "bool"`

**Example:**
```
var a = true
var b = false
print(a and b)      // false
print(a or b)       // true
print(not a)        // false
```

#### 3.2.3. Numeric Types

RustLeaf has two numeric types: integers and floating-point numbers.

**Integer Type (int):**
- Type name: `"int"`
- 64-bit signed integers (-2^63 to 2^63-1)
- Overflow/underflow raises a runtime error
- No implicit conversion to float

**Floating-Point Type (float):**
- Type name: `"float"`
- IEEE 754 double-precision (64-bit)
- Supports special values: Infinity, -Infinity, NaN
- NaN propagates through operations
- No implicit conversion to int

**Numeric Operations:**
```
// Arithmetic (int and float)
+   // Addition
-   // Subtraction/negation  
*   // Multiplication
/   // Division (int division truncates, float division is exact)
%   // Modulo
**  // Exponentiation

// Bitwise (int only)
&   // Bitwise AND
|   // Bitwise OR
^   // Bitwise XOR
~   // Bitwise NOT
<<  // Left shift
>>  // Right shift

// Comparison (int and float)
<   // Less than
>   // Greater than
<=  // Less than or equal
>=  // Greater than or equal
==  // Equal
!=  // Not equal
```

**Overflow Behavior:**
```
var max_int = 9223372036854775807
var overflow = max_int + 1  // Error: Integer overflow

var min_int = -9223372036854775808  
var underflow = min_int - 1  // Error: Integer underflow
```

**Example:**
```
// Integers
var x = 42
var y = 13
print(x + y)        // 55
print(x / y)        // 3 (integer division)
print(x % y)        // 3 (remainder)

// Floats
var a = 3.14
var b = 2.0
print(a / b)        // 1.57
print(type(a))      // "float"

// NaN and Infinity
var inf = 1.0 / 0.0   // Infinity
var nan = 0.0 / 0.0   // NaN
print(inf > 1000000)  // true
print(nan == nan)     // false (NaN != NaN)
```

#### 3.2.4. String Type

Strings are immutable sequences of Unicode characters.

**Properties:**
- Type name: `"string"`
- UTF-8 encoded internally
- Immutable (operations return new strings)
- Support for string interpolation

**String Operations:**
- Concatenation: `+` operator
- Repetition: `*` operator with integer
- Comparison: lexicographic ordering
- Indexing: `str[index]` returns single-character string
- Slicing: `str[start:end]` returns substring
- Length: `len(str)` returns character count

**String Methods:**
- `split(delimiter)` - Split into list of strings
- `trim()` - Remove leading/trailing whitespace
- `upper()` - Convert to uppercase
- `lower()` - Convert to lowercase
- `replace(old, new)` - Replace all occurrences
- `contains(substring)` - Test for substring presence
- `starts_with(prefix)` - Test prefix
- `ends_with(suffix)` - Test suffix
- `is_empty()` - Test if length is 0

**Example:**
```
var s = "Hello"
var t = "World"
var combined = s + ", " + t + "!"  // "Hello, World!"

// String interpolation
var name = "Alice"
var age = 30
var message = "My name is ${name} and I am ${age} years old"

// String methods
var text = "  Hello, World!  "
print(text.trim())          // "Hello, World!"
print(text.upper())         // "  HELLO, WORLD!  "
print(text.contains("World")) // true
```

### 3.3. Composite Types

Composite types are mutable containers that can hold multiple values.

#### 3.3.1. List Type

Lists are ordered, mutable sequences that can contain values of any type.

**Properties:**
- Type name: `"list"`
- Zero-indexed
- Heterogeneous (can mix types)
- Mutable (can be modified in place)
- Dynamic size

**List Operations:**
- Creation: `[expr1, expr2, ...]`
- Indexing: `list[index]` (negative indices count from end)
- Slicing: `list[start:end]`
- Length: `len(list)`
- Membership: `value in list`
- Concatenation: `list1 + list2` (returns new list)

**List Methods:**
- `append(value)` - Add to end
- `extend(other_list)` - Add all elements from other list
- `insert(index, value)` - Insert at position
- `pop(index?)` - Remove and return element (default: last)
- `remove(value)` - Remove first occurrence
- `clear()` - Remove all elements
- `sort()` - Sort in place
- `reverse()` - Reverse in place
- `map(function)` - Transform elements (returns new list)
- `filter(function)` - Select elements (returns new list)
- `reduce(function, initial?)` - Reduce to single value
- `is_empty()` - Test if length is 0

**Example:**
```
var nums = [1, 2, 3]
nums.append(4)              // [1, 2, 3, 4]
nums[0] = 10               // [10, 2, 3, 4]

var mixed = [1, "hello", true, [2, 3]]
print(len(mixed))          // 4
print(mixed[3][1])         // 3

// Functional methods
var doubled = nums.map(fn(x) { x * 2 })
var evens = nums.filter(fn(x) { x % 2 == 0 })
var sum = nums.reduce(fn(a, b) { a + b }, 0)
```

#### 3.3.2. Dict Type

Dictionaries are mutable mappings from keys to values with preserved insertion order.

**Properties:**
- Type name: `"dict"`
- Keys must be immutable types (string, int, float, bool, null)
- Values can be any type
- Preserves insertion order
- Mutable

**Dict Operations:**
- Creation: `{key1: value1, key2: value2, ...}`
- Access: `dict[key]` (raises error if key not found)
- Assignment: `dict[key] = value`
- Membership: `key in dict`
- Length: `len(dict)`

**Dict Methods:**
- `get(key, default?)` - Get value or default if not found
- `set(key, value)` - Set key-value pair
- `pop(key, default?)` - Remove and return value
- `clear()` - Remove all entries
- `keys()` - Return list of keys
- `values()` - Return list of values  
- `items()` - Return list of [key, value] pairs
- `update(other_dict)` - Merge other dict into this one
- `has(key)` - Test if key exists
- `is_empty()` - Test if length is 0

**Example:**
```
var person = {
    "name": "Alice",
    "age": 30,
    "hobbies": ["reading", "coding"]
}

person["city"] = "New York"
print(person.get("age"))           // 30
print(person.get("phone", "N/A"))  // "N/A"

for key, value in person.items() {
    print("${key}: ${value}")
}
```

#### 3.3.3. Object Type

Objects are instances of user-defined classes.

**Properties:**
- Type name: The class name
- Contains fields (data) and methods (behavior)
- Mutable
- Each class creates a distinct type

**Object Creation:**
- Call class as constructor: `ClassName()`
- Fields initialize to default values or null
- No automatic constructor parameters

**Field Access:**
- Get: `object.field`
- Set: `object.field = value`
- Dynamic: `object[fieldname]` where fieldname is a string

**Method Calls:**
- `object.method(args...)`
- Methods receive implicit `self` parameter
- Static methods called on class: `ClassName.static_method(args...)`

**Example:**
```
class Point {
    var x = 0;
    var y = 0;
    
    fn distance() {
        (self.x ** 2 + self.y ** 2) ** 0.5
    }
}

var p = Point()
p.x = 3
p.y = 4
print(type(p))         // "Point"
print(p.distance())    // 5.0
```

### 3.4. Function Types

Functions are first-class values that encapsulate executable code.

**Properties:**
- Type name: `"function"`
- Includes regular functions, methods, closures, and lambdas
- Can be assigned to variables, passed as arguments, and returned from functions
- Capture variables from enclosing scope (closures)

**Function Categories:**
1. **Regular Functions**: Defined with `fn` at module or class level
2. **Lambda Functions**: Anonymous functions created with `fn(params) { body }`
3. **Methods**: Functions defined within a class (receive implicit `self`)
4. **Bound Methods**: Methods bound to a specific instance
5. **Built-in Functions**: Functions provided by the runtime

**Function Operations:**
- Call: `function(args...)`
- Type check: `type(function) == "function"`
- Equality: Functions are compared by identity

**Example:**
```
fn add(a, b) {
    a + b
}

var f = add
print(type(f))         // "function"
print(f(2, 3))        // 5

// Lambda
var square = fn(x) { x * x }
print(square(4))      // 16

// Closure
fn make_counter() {
    var count = 0
    fn() {
        count = count + 1
        count
    }
}

var counter = make_counter()
print(counter())      // 1
print(counter())      // 2
```

### 3.5. RustValue Type

RustValue is the extension mechanism for implementing custom types in Rust.

**Properties:**
- Type name: Determined by `RustValue::type_name()`
- Enables custom behavior implemented in Rust
- Integrates seamlessly with RustLeaf's type system
- Can have fields and methods like regular objects

**RustValue Interface:**
```rust
trait RustValue {
    fn type_name(&self) -> &str;
    fn get_field(&self, name: &str) -> Option<Value>;
    fn set_field(&mut self, name: &str, value: Value) -> Result<(), String>;
    fn call_method(&self, name: &str, args: Vec<Value>) -> Result<Value, String>;
}
```

**Type Identity:**
- Each RustValue implementation provides its own type name
- Type checking uses the string returned by `type_name()`
- No inheritance or subtyping relationships

**Equality:**
- Default: Reference equality (same Rust object)
- Can be overridden by implementing custom equality in Rust

**Example (from RustLeaf perspective):**
```
// Assuming a Point type implemented in Rust
var p = Point(3.0, 4.0)
print(type(p))           // "Point"
print(p.x)               // 3.0
print(p.distance())      // 5.0

// Type checking
if type(p) == "Point" {
    print("p is a Point")
}
```

### 3.6. Type Conversions

RustLeaf performs no implicit type conversions. All conversions must be explicit.

**No Implicit Conversions:**
- Numeric types are never automatically converted
- No automatic string coercion (except in string interpolation)
- Boolean contexts require actual boolean values

**Explicit Conversion Functions:**
- `int(value)` - Convert to integer
  - From float: truncates toward zero
  - From string: parses decimal integer
  - From bool: true→1, false→0
- `float(value)` - Convert to float
  - From int: exact conversion
  - From string: parses floating-point number
- `str(value)` - Convert to string
  - All types have string representations
  - Used for display and debugging
- `bool(value)` - Convert to boolean
  - Only valid for types with truthiness

**String Interpolation:**
The only implicit conversion occurs in string interpolation, where values are automatically converted to strings:
```
var n = 42
var s = "The answer is ${n}"  // n implicitly converted to "42"
```

**Conversion Errors:**
Invalid conversions raise runtime errors:
```
int("hello")    // Error: Invalid integer format
float("abc")    // Error: Invalid float format
bool([1, 2, 3]) // Error: List has no truthiness
```

### 3.7. Type Checking

Type checking in RustLeaf occurs at runtime when operations are performed.

**Type Checking Mechanisms:**

1. **type() Function**:
   ```
   var t = type(value)  // Returns type name as string
   ```

2. **Direct Comparison**:
   ```
   if type(x) == "int" {
       // x is an integer
   }
   ```

3. **Pattern Matching**:
   ```
   match value {
       case n if type(n) == "int" {
           // Handle integer
       }
       case s if type(s) == "string" {
           // Handle string  
       }
   }
   ```

**Type Errors:**
Operations that receive invalid types raise descriptive errors:
```
"hello" + 42        // Error: Cannot add string and int
[1, 2] * "abc"      // Error: Cannot multiply list and string
null.foo()          // Error: null has no method 'foo'
```

**Error Message Format:**
Type errors include:
- The operation attempted
- The actual type(s) received
- The expected type(s)
- Source location (line, column)

**Duck Typing:**
RustLeaf supports duck typing—if an object has the required methods or fields, operations succeed regardless of its type:
```
class Duck {
    fn quack() { "Quack!" }
}

class Person {
    fn quack() { "I'm quacking!" }
}

fn make_it_quack(thing) {
    print(thing.quack())
}

make_it_quack(Duck())     // "Quack!"
make_it_quack(Person())   // "I'm quacking!"
```

**Truthiness:**
Only `null` and boolean values have truthiness. All other types raise an error in boolean contexts:
```
// Valid
if true { }
if false { }
if null { }          // null is falsy
if not null { }      // true

// Invalid - these raise errors
if 0 { }             // Error: int has no truthiness
if "" { }            // Error: string has no truthiness  
if [] { }            // Error: list has no truthiness

// Must use explicit tests
if x == 0 { }
if s.is_empty() { }
if list.is_empty() { }
```