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
[4.1. Variable Declarations](#41-variable-declarations)  
[4.2. Variable Initialization](#42-variable-initialization)  
[4.3. Scope and Lifetime](#43-scope-and-lifetime)  
[4.4. Assignment](#44-assignment)  

### 5. Expressions
[5.1. Expression Evaluation](#51-expression-evaluation)  
[5.2. Primary Expressions](#52-primary-expressions)  
[5.3. Postfix Expressions](#53-postfix-expressions)  
   [5.3.1. Property Access](#531-property-access)  
   [5.3.2. Index Access](#532-index-access)  
   [5.3.3. Function Call](#533-function-call)  
   [5.3.4. Method Call](#534-method-call)  
[5.4. Unary Expressions](#54-unary-expressions)  
[5.5. Binary Expressions](#55-binary-expressions)  
   [5.5.1. Arithmetic Operators](#551-arithmetic-operators)  
   [5.5.2. Comparison Operators](#552-comparison-operators)  
   [5.5.3. Logical Operators](#553-logical-operators)  
   [5.5.4. Bitwise Operators](#554-bitwise-operators)  
   [5.5.5. Operator Precedence](#555-operator-precedence)  
[5.6. Assignment Expressions](#56-assignment-expressions)  
[5.7. Conditional Expressions (if)](#57-conditional-expressions-if)  
[5.8. Match Expressions](#58-match-expressions)  
[5.9. Try Expressions](#59-try-expressions)  
[5.10. Block Expressions](#510-block-expressions)  
[5.11. Anonymous Function Expressions](#511-anonymous-function-expressions)  
[5.12. Object Literal Expressions](#512-object-literal-expressions)  
[5.13. List Literal Expressions](#513-list-literal-expressions)  
[5.14. Dict Literal Expressions](#514-dict-literal-expressions)  
[Operator Overloading](#operator-overloading)  

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
[7.1. Function Declarations](#71-function-declarations)  
[7.2. Function Parameters](#72-function-parameters)  
[7.3. Function Body](#73-function-body)  
[7.4. Return Values](#74-return-values)  
[7.5. Closures](#75-closures)  
[7.6. Anonymous Functions](#76-anonymous-functions)  
[7.7. Function Scope](#77-function-scope)  
[7.8. Recursion](#78-recursion)  
[Function Call Semantics](#function-call-semantics)  

### 8. Pattern Matching
[8.1. Pattern Syntax](#81-pattern-syntax)  
[8.2. Literal Patterns](#82-literal-patterns)  
[8.3. Variable Patterns](#83-variable-patterns)  
[8.4. Wildcard Patterns](#84-wildcard-patterns)  
[8.5. List Patterns](#85-list-patterns)  
[8.6. Dict Patterns](#86-dict-patterns)  
[8.7. Range Patterns](#87-range-patterns)  
[8.8. Guard Expressions](#88-guard-expressions)  
[8.9. Pattern Evaluation](#89-pattern-evaluation)  
[Or Patterns](#or-patterns)  

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

### 17. Attributes
17.1. Attribute Syntax  
17.2. Attribute Application Rules  
17.3. Built-in Attributes  
17.4. User-defined Attributes  
17.5. Attribute Processing  
17.6. AST Transformation  
17.7. Attribute Evaluation Order  

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

---

## 4. Variables

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

## 5. Expressions

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
print(null or true)     // true

// Short-circuit behavior
var x = true
var y = (x or print("not evaluated"))

// Type errors
// if 0 and true { }    // Error: int has no truthiness
// if [] or true { }    // Error: list has no truthiness
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
11. or
```

**Examples:**
```
2 + 3 * 4        // 14 (not 20)
not true or false // false (not is higher precedence)
1 << 2 + 1       // 8 (+ before <<)
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

## 6. Statements

Statements are instructions that perform actions but do not produce values. This chapter defines the various statement types in RustLeaf, their syntax, and execution semantics. Unlike expressions, statements are executed for their side effects.

### 6.1. Statement Evaluation

Statements are executed sequentially within their containing block or module.

**Statement Execution Rules:**
- Statements execute in the order they appear
- Statements must be separated by semicolons, except for the last statement in a block
- The last statement in a block may omit its semicolon if it's an expression whose value should be returned
- Module-level statements execute when the module is loaded
- Certain statements (break, continue, return) alter control flow

**Statement Categories:**
1. **Expression statements** - Expressions executed for side effects
2. **Variable declarations** - Introduce new variables
3. **Control flow statements** - Alter execution flow
4. **Import statements** - Import modules and symbols
5. **Empty statements** - No operation

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
    print("first")    // Error: missing semicolon
    print("second")   // This line is unreachable
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
- Commonly used for function calls, assignments, and method calls
- Pure expressions without side effects are allowed but not useful

**Examples:**
```
// Function call statements
print("Hello, World!");
list.append(42);
file.close();

// Assignment statements  
x = 10;
obj.field = "value";
arr[0] = 100;

// Compound assignment
count += 1;
total *= 2;

// Method calls
list.sort();
dict.clear();

// Pure expressions (legal but not useful)
42;              // Evaluates to 42, then discards it
x + y;           // Computes sum, then discards it
true and false;  // Evaluates to false, then discards it
```

### 6.3. Variable Declaration Statements

Variables are declared using the `var` keyword, optionally with initialization.

**Syntax:**
```
VarStatement = "var" Pattern ("=" Expression)? ";"
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

### 6.4. Empty Statements

An empty statement performs no operation.

**Syntax:**
```
EmptyStatement = ";"
```

**Uses:**
- Generated code placeholders
- Syntactic requirements
- Explicit "do nothing"

**Examples:**
```
// Empty statement
;

// Empty block serves similar purpose
{ }

// In generated code
if condition {
    ; // Placeholder for future code
}
```

### 6.5. Block Statements

Block statements group multiple statements and create a new scope.

**Syntax:**
```
BlockStatement = "{" Statement* "}"
```

**Block Statement vs Block Expression:**
- Block statement: Used where a statement is expected, no value returned
- Block expression: Used where an expression is expected, returns last expression value

**Examples:**
```
// Block statement in if
if condition {
    var x = 10;
    print(x);
    // No value returned
}

// Block statement standalone
{
    var temp = calculate();
    process(temp);
    cleanup(temp);
}

// Block expression (note no semicolon after block)
var result = {
    var x = 10;
    var y = 20;
    x + y  // This value is returned
}
```

### 6.6. Control Flow Statements

Control flow statements alter the sequential execution of statements.

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
- While loops are statements, not expressions (cannot return values)

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

**Iterator Protocol:**
- Expression must return an object with `op_iter()` method
- `op_iter()` returns an iterator object
- Iterator must have `op_next()` method
- `op_next()` returns next value or unit when done
- Built-in types (list, dict, string) implement the protocol

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
The unit type is a special value that:
- Cannot be written as a literal
- Is returned by `return` statements without a value
- Is returned by functions without explicit return
- Is returned by `op_next()` when iteration is complete
- Can be checked with `is_unit(value)` built-in function
- Has type name `"unit"` when checked with `type()`

#### 6.6.3. Try-Catch Statements

Try-catch statements handle exceptions that may occur during execution.

**Syntax:**
```
TryStatement = "try" Block "catch" Pattern Block
Pattern = Identifier | DictPattern
```

**Exception Handling:**
- Try block executes normally until an exception occurs
- On exception, control transfers to catch block
- Catch pattern binds the exception value
- No finally clause (use `with` statement for cleanup)
- Try-catch is also an expression form (see Section 5.9)

**Examples:**
```
// Basic exception handling
try {
    var result = risky_operation();
    process(result);
} catch e {
    print("Error occurred: ${e}");
}

// Pattern matching on exception
try {
    validate_input(data);
} catch {type, message} {
    print("${type} error: ${message}");
}

// Re-raising exceptions
try {
    database_operation();
} catch e {
    log_error(e);
    raise(e);  // Re-raise the same exception
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

#### 6.6.4. With Statements

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
- After block (or on exception), resources are cleaned up
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

// With exception handling
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

#### 6.6.5. Break Statements

Break statements exit from the nearest enclosing loop.

**Syntax:**
```
BreakStatement = "break" ";"
```

**Break Rules:**
- Must appear within a while or for loop
- Immediately exits the loop
- Execution continues after the loop
- Cannot break from nested loops to outer loops
- Break in statement loops does not return a value

**Examples:**
```
// Break from while
while true {
    var cmd = read_command();
    if cmd == "exit" {
        break;
    }
    execute(cmd);
}

// Break from for
for item in large_list {
    if found_target(item) {
        break;
    }
}

// Break in nested loops (only breaks inner)
for i in range(0, 10) {
    for j in range(0, 10) {
        if matrix[i][j] == target {
            break;  // Only exits inner loop
        }
    }
}

// Error: break outside loop
if condition {
    break;  // Error: break not in loop
}
```

#### 6.6.6. Continue Statements

Continue statements skip to the next iteration of the nearest enclosing loop.

**Syntax:**
```
ContinueStatement = "continue" ";"
```

**Continue Rules:**
- Must appear within a while or for loop
- Skips remaining statements in loop body
- For while: re-evaluates condition
- For for: advances to next iteration
- Cannot continue outer loops from nested loops

**Examples:**
```
// Skip even numbers
for i in range(0, 10) {
    if i % 2 == 0 {
        continue;
    }
    print(i);  // Only prints odd numbers
}

// Continue in while
var i = 0;
while i < 100 {
    i += 1;
    if not is_valid(i) {
        continue;
    }
    process(i);
}

// Error: continue outside loop
fn example() {
    continue;  // Error: continue not in loop
}
```

#### 6.6.7. Return Statements

Return statements exit from the current function with an optional value.

**Syntax:**
```
ReturnStatement = "return" Expression? ";"
```

**Return Rules:**
- Must appear within a function body
- Immediately exits the function
- Without expression, returns unit
- With expression, returns expression value
- Cannot be used at module level

**Unit Type and Return:**
```
// These all return unit
fn example1() {
    return;  // Explicit return unit
}

fn example2() {
    // Implicit return unit (empty body)
}

fn example3() {
    print("hello");  // No return, so returns unit
}

// Check for unit
var result = example1();
if is_unit(result) {
    print("Function returned unit");
}
```

**Examples:**
```
// Return with value
fn add(a, b) {
    return a + b;
}

// Early return
fn validate(data) {
    if data == null {
        return false;
    }
    if data.length == 0 {
        return false;
    }
    return true;
}

// Return without value
fn print_message(msg) {
    if msg == "" {
        return;  // Returns unit
    }
    print(msg);
    // Implicit return unit
}

// Error: return outside function
var x = 10;
if x > 5 {
    return x;  // Error: return not in function
}
```

### 6.7. Import Statements

Import statements bring symbols from other modules into scope.

**Syntax:**
```
ImportStatement = "use" ModulePath ImportList ";"
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
- Relative to current module's directory
- `library::math` looks for `library/math.rustleaf`
- No `mod.rs` or module republishing

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

## 7. Functions

Functions are reusable units of code that encapsulate behavior and can accept parameters and return values. RustLeaf treats functions as first-class values that can be assigned, passed, and returned. This chapter defines function declaration, parameters, return values, closures, and execution semantics.

### 7.1. Function Declarations

Functions are declared using the `fn` keyword followed by a name, parameter list, and body.

**Syntax:**
```
FunctionDeclaration = "fn" Identifier "(" ParameterList? ")" Block
ParameterList = Parameter ("," Parameter)*
Parameter = Identifier ("=" Expression)?
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
- Parameters are evaluated left-to-right
- Default values are evaluated when function is defined, not called
- Parameter order: required, defaults, *args, **kwargs
- Only one `*args` and one `**kwargs` allowed
- Parameters after `*args` are keyword-only

**Parameter Destructuring:**
Parameters can use destructuring patterns:
```
fn process_point({x, y}) {
    print("Point at (${x}, ${y})")
}

fn head_tail([first, *rest]) {
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

## 8. Pattern Matching

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
- Or patterns - Match any of several patterns

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
    case 1 | 2 | 3 { "small" }
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
    case "yes" | "y" | "true" { true }
    case "no" | "n" | "false" { false }
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

// Or patterns with lists
match coords {
    case [0, 0] | [0, _] | [_, 0] {
        "on axis"
    }
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

// Or patterns with dicts
match config {
    case {mode: "dev"} | {mode: "development"} {
        enable_debug()
    }
}
```

### 8.7. Range Patterns

Range patterns match integers within inclusive ranges.

**Syntax:**
```
RangePattern = IntegerLiteral ".." IntegerLiteral
```

**Properties:**
- Both bounds are inclusive
- Only integer literals allowed
- Start must be less than or equal to end
- No expressions in bounds

**Examples:**
```
match score {
    case 0 { "zero" }
    case 1..10 { "low" }
    case 11..50 { "medium" }
    case 51..100 { "high" }
    case _ { "out of range" }
}

match char_code {
    case 48..57 { "digit" }      // '0'..'9'
    case 65..90 { "uppercase" }  // 'A'..'Z'
    case 97..122 { "lowercase" } // 'a'..'z'
}

// Or patterns with ranges
match age {
    case 0..12 | 65..120 {
        "discount eligible"
    }
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
3. Or patterns are tested left-to-right
4. Nested patterns are matched outside-in

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
- Or patterns (unless one branch is irrefutable)

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

// Or pattern evaluation
match 5 {
    case 1 | 2 | 3 { "small" }
    case 4 | 5 | 6 { "medium" }  // Matches at 5
    case _ { "large" }
}

// Failed match
var result = match value {
    case 1 { "one" }
    case 2 { "two" }
}
// result is null if value is not 1 or 2
```

### Or Patterns

Or patterns allow matching multiple alternatives with the `|` operator.

**Syntax:**
```
OrPattern = Pattern ("|" Pattern)*
```

**Rules:**
- All alternatives must bind the same variables
- Variable types can differ between alternatives
- Matches left-to-right, stops at first match
- Can combine different pattern types

**Examples:**
```
// Simple alternatives
match command {
    case "quit" | "exit" | "q" {
        shutdown()
    }
    case "help" | "h" | "?" {
        show_help()
    }
}

// With different pattern types
match value {
    case 0 | null | [] {
        "empty"
    }
    case 1 | [1] | {value: 1} {
        "one"
    }
}

// Variables must match
match pair {
    case [x, 0] | [0, x] {
        print("Has zero and ${x}")
    }
    // Both branches bind 'x'
}

// Not allowed - inconsistent bindings
match value {
    // case [x] | [x, y] { }  // Error: y not bound in first alternative
}

// Nested or patterns
match response {
    case {status: 200 | 201 | 204} {
        "success"
    }
    case {status: 400..499} {
        "client error"
    }
}
```