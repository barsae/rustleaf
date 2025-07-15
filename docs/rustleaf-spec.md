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
3.1. Type System Overview  
3.2. Primitive Types  
   3.2.1. Null Type  
   3.2.2. Boolean Type  
   3.2.3. Numeric Types  
   3.2.4. String Type  
3.3. Composite Types  
   3.3.1. List Type  
   3.3.2. Dict Type  
   3.3.3. Object Type  
3.4. Function Types  
3.5. RustValue Type  
3.6. Type Conversions  
3.7. Type Checking  

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