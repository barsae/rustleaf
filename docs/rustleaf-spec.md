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
[6.1. Statement Evaluation](#61-statement-evaluation)  
[6.2. Expression Statements](#62-expression-statements)  
[6.3. Variable Declaration Statements](#63-variable-declaration-statements)  
[6.4. Empty Statements](#64-empty-statements)  
[6.5. Block Statements](#65-block-statements)  
[6.6. Control Flow Statements](#66-control-flow-statements)  
   [6.6.1. If Statements](#661-if-statements)  
   [6.6.2. While Statements](#662-while-statements)  
   [6.6.3. For Statements](#663-for-statements)  
   [6.6.4. Match Statements](#664-match-statements)  
   [6.6.5. Try-Catch-Finally Statements](#665-try-catch-finally-statements)  
   [6.6.6. With Statements](#666-with-statements)  
   [6.6.7. Break Statements](#667-break-statements)  
   [6.6.8. Continue Statements](#668-continue-statements)  
   [6.6.9. Return Statements](#669-return-statements)  

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
[9.1. Error Types](#91-error-types)  
[9.2. Raising Errors](#92-raising-errors)  
[9.3. Try-Catch Blocks](#93-try-catch-blocks)  
[9.4. Finally Blocks](#94-finally-blocks)  
[9.5. Error Propagation](#95-error-propagation)  
[9.6. Error Objects](#96-error-objects)  
[Assert Function](#assert-function)  
[Pattern Match Failures](#pattern-match-failures)  

### 10. Modules
[10.1. Module System Overview](#101-module-system-overview)  
[10.2. Import Statements](#102-import-statements)  
[10.3. Export Functions](#103-export-functions)  
[10.4. Module Resolution](#104-module-resolution)  
[10.5. Module Loading](#105-module-loading)  
[10.6. Module Scope](#106-module-scope)  

### 11. Built-in Functions
[11.1. Type Functions](#111-type-functions)  
[11.2. Collection Functions](#112-collection-functions)  
[11.3. String Functions](#113-string-functions)  
[11.4. Error Functions](#114-error-functions)  
[11.5. Utility Functions](#115-utility-functions)  
[11.6. Global Availability](#116-global-availability)  

### 12. Standard Library
[12.1. String Methods](#121-string-methods)  
[12.2. List Methods](#122-list-methods)  
[12.3. Dict Methods](#123-dict-methods)  
[12.4. Object Methods](#124-object-methods)  
[12.5. Method Resolution and Inheritance](#125-method-resolution-and-inheritance)  

### 13. Documentation Comments and Docstrings
[13.1. Overview](#131-overview)  
[13.2. Documentation Comment Syntax](#132-documentation-comment-syntax)  
[13.3. Docstring Syntax](#133-docstring-syntax)  
[13.4. Function Docstrings](#134-function-docstrings)  
[13.5. Module Docstrings](#135-module-docstrings)  
[13.6. Variable and Constant Docstrings](#136-variable-and-constant-docstrings)  
[13.7. Object and Type Docstrings](#137-object-and-type-docstrings)  
[13.8. Docstring Format Conventions](#138-docstring-format-conventions)  
[13.9. Runtime Access to Docstrings](#139-runtime-access-to-docstrings)  
[13.10. Tooling Integration](#1310-tooling-integration)  

### 14. Memory Model
[14.1. Value Semantics](#141-value-semantics)  
[14.2. Reference Semantics](#142-reference-semantics)  
[14.3. Garbage Collection](#143-garbage-collection)  
[14.4. Resource Management](#144-resource-management)  

### 15. Execution Model
[15.1. Program Execution](#151-program-execution)  
[15.2. Expression Evaluation Order](#152-expression-evaluation-order)  
[15.3. Function Call Semantics](#153-function-call-semantics)  
[15.4. Exception Handling](#154-exception-handling)  
[15.5. Resource Cleanup](#155-resource-cleanup)  

### 16. RustValue Integration
[16.1. RustValue Trait](#161-rustvalue-trait)  
[16.2. Value Enum](#162-value-enum)  
[16.3. Field Access](#163-field-access)  
[16.4. Method Dispatch](#164-method-dispatch)  
[16.5. Type Coercion](#165-type-coercion)  
[16.6. Lifetime Management](#166-lifetime-management)  

### 17. Macros
[17.1. Macro Syntax](#171-macro-syntax)  
[17.2. Macro Application Rules](#172-macro-application-rules)  
[17.3. Built-in Macros](#173-built-in-macros)  
[17.4. User-defined Macros](#174-user-defined-macros)  
[17.5. Macro Processing](#175-macro-processing)  
[17.6. AST Transformation](#176-ast-transformation)  
[17.7. Macro Evaluation Order](#177-macro-evaluation-order)  

### Appendices
[A. Grammar Summary](#a-grammar-summary)  
[B. Operator Precedence](#b-operator-precedence)  
[C. Reserved Words](#c-reserved-words)  
[D. Built-in Function Reference](#d-built-in-function-reference)  
[E. Error Codes](#e-error-codes)  
[F. Implementation Limits](#f-implementation-limits)  

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

---

## 9. Error Handling

RustLeaf provides error handling through exceptions that can be raised and caught. Any value can be raised as an error, and errors propagate up the call stack until caught. This chapter defines error raising, catching, propagation, and cleanup semantics.

### 9.1. Error Types

Errors in RustLeaf are not a special type—any value can be raised as an error.

**Error Values:**
- Any value can be raised: strings, numbers, objects, etc.
- Most commonly strings or custom objects
- No required error base class or interface
- Error type identified using `type()` function

**Common Error Patterns:**
```
// String errors
raise("File not found")
raise("Invalid argument: expected positive number")

// Object errors
class NetworkError {
    var code;
    var message;
    
    static fn new(code, message) {
        var e = NetworkError()
        e.code = code
        e.message = message
        e
    }
    
    fn op_str() {
        "NetworkError(${self.code}): ${self.message}"
    }
}

raise(NetworkError.new(404, "Resource not found"))

// Even primitives can be errors
raise(42)
raise(null)
raise([1, 2, 3])
```

**Error Information:**
When an error is raised, the runtime captures:
- The error value
- Stack trace at point of raise
- Source location (file, line, column)

### 9.2. Raising Errors

Errors are raised using the `raise` function, which immediately transfers control to the nearest error handler.

**Syntax:**
```
raise(value)
```

**Raise Semantics:**
- Evaluates the argument to get error value
- Captures current stack trace
- Unwinds stack looking for try-catch block
- If no handler found, terminates program
- Never returns to caller

**Examples:**
```
fn divide(a, b) {
    if b == 0 {
        raise("Division by zero")
    }
    a / b
}

fn validate_age(age) {
    if type(age) != "int" {
        raise("Age must be an integer")
    }
    if age < 0 {
        raise("Age cannot be negative")
    }
    if age > 150 {
        raise("Age seems unrealistic")
    }
}

// Conditional raising
fn process_data(data) {
    if not data {
        raise("No data provided")
    }
    
    if type(data) != "list" {
        raise("Data must be a list")
    }
    
    // Process data...
}

// Raising custom errors
class ValidationError {
    var field;
    var value;
    var message;
    
    static fn new(field, value, message) {
        var e = ValidationError()
        e.field = field
        e.value = value
        e.message = message
        e
    }
    
    fn op_str() {
        "ValidationError in ${self.field}: ${self.message}"
    }
}

fn validate_email(email) {
    if not email.contains("@") {
        raise(ValidationError.new("email", email, "Missing @ symbol"))
    }
}
```

### 9.3. Try-Catch Blocks

Try-catch blocks handle errors by providing an alternative execution path when errors occur.

**Syntax:**
```
TryExpression = "try" Block "catch" Pattern Block
```

**Try-Catch Semantics:**
- Try block is executed
- If no error, catch block is skipped
- If error raised, control transfers to catch
- Catch pattern matches against error value
- Try-catch is an expression (returns a value)
- Value is from try block or catch block

**Pattern Matching:**
- Catch uses pattern matching on error
- Can destructure error objects
- Non-matching patterns re-raise error

**Examples:**
```
// Basic try-catch
var result = try {
    risky_operation()
} catch e {
    print("Error occurred: ${e}")
    null  // Default value on error
}

// Pattern matching errors
var data = try {
    parse_json(input)
} catch {type: "SyntaxError", line: l} {
    print("JSON syntax error on line ${l}")
    {}  // Empty dict as fallback
} catch e {
    print("Unexpected error: ${e}")
    raise(e)  // Re-raise
}

// Using error type
fn safe_divide(a, b) {
    try {
        divide(a, b)
    } catch e {
        if type(e) == "string" and e.contains("zero") {
            0  // Return 0 for division by zero
        } else {
            raise(e)  // Re-raise other errors
        }
    }
}

// Nested try-catch
try {
    var conn = connect_database()
    try {
        conn.execute(query)
    } catch e {
        print("Query failed: ${e}")
        rollback(conn)
    }
} catch e {
    print("Connection failed: ${e}")
}
```

### 9.4. Finally Blocks

Note: RustLeaf does not have finally blocks. Use `with` statements for cleanup:

```
// Instead of try-finally, use with:
with resource = acquire_resource() {
    try {
        use_resource(resource)
    } catch e {
        print("Error using resource: ${e}")
        raise(e)
    }
}  // resource.close() called automatically

// Equivalent to try-catch-finally pattern:
with file = open("data.txt") {
    try {
        process(file)
    } catch e {
        log_error(e)
        raise(e)
    }
}  // file.close() always runs
```

### 9.5. Error Propagation

Errors propagate up the call stack until caught or the program terminates.

**Propagation Rules:**
1. Error raised in function
2. Function immediately returns (abnormally)
3. Caller's execution interrupted
4. Stack unwinds to nearest try-catch
5. If none found, program terminates

**Stack Traces:**
- Captured at raise point
- Include function names and line numbers
- Available in error handlers
- Printed on uncaught errors

**Examples:**
```
fn level3() {
    raise("Deep error")
}

fn level2() {
    print("Before level3")
    level3()  // Error propagates here
    print("Never reached")
}

fn level1() {
    try {
        level2()
    } catch e {
        print("Caught at level1: ${e}")
        // Stack trace available here
    }
}

// Selective catching
fn process_file(path) {
    var file = try {
        open(path)
    } catch e {
        // Only catch file errors
        if e.contains("File") or e.contains("Permission") {
            return null  // File not available
        } else {
            raise(e)  // Propagate other errors
        }
    }
    
    // Process file...
}

// Automatic propagation
fn caller() {
    risky_function()  // Error propagates automatically
}

// Re-raising with context
fn wrapper() {
    try {
        dangerous_operation()
    } catch e {
        // Add context and re-raise
        raise("Failed in wrapper: ${e}")
    }
}
```

### 9.6. Error Objects

While any value can be an error, objects provide structured error information.

**Error Object Conventions:**
- No required structure
- Common pattern: include message, code, details
- Use `op_str()` for display formatting
- Type name identifies error category

**Examples:**
```
// Simple error class
class FileError {
    var path;
    var operation;
    var reason;
    
    static fn new(path, operation, reason) {
        var e = FileError()
        e.path = path
        e.operation = operation
        e.reason = reason
        e
    }
    
    fn op_str() {
        "FileError: ${self.operation} failed on '${self.path}': ${self.reason}"
    }
}

// Usage
fn read_config(path) {
    if not exists(path) {
        raise(FileError.new(path, "read", "file not found"))
    }
    
    var content = try {
        read_file(path)
    } catch e {
        raise(FileError.new(path, "read", str(e)))
    }
    
    parse_config(content)
}

// Error with error code
class HttpError {
    var status;
    var message;
    var url;
    
    static fn new(status, message, url) {
        var e = HttpError()
        e.status = status
        e.message = message
        e.url = url
        e
    }
    
    fn op_str() {
        "HttpError ${self.status}: ${self.message} (${self.url})"
    }
}

// Catching specific error types
try {
    fetch_data(url)
} catch e {
    if type(e) == "HttpError" {
        if e.status == 404 {
            return cached_data()  // Use cache for 404
        } else if e.status >= 500 {
            retry_later()  // Server error
        }
    }
    raise(e)  // Re-raise others
}

// Chained errors
class ChainedError {
    var message;
    var cause;
    
    static fn new(message, cause) {
        var e = ChainedError()
        e.message = message
        e.cause = cause
        e
    }
    
    fn op_str() {
        if self.cause {
            "${self.message}\nCaused by: ${self.cause}"
        } else {
            self.message
        }
    }
}

fn high_level_operation() {
    try {
        low_level_operation()
    } catch e {
        raise(ChainedError.new("High-level operation failed", e))
    }
}
```

### Assert Function

The `assert` function provides runtime assertions for debugging and validation.

**Syntax:**
```
assert(condition, message?)
```

**Assert Semantics:**
- Evaluates condition
- If truthy, returns null
- If falsy, raises error with message
- Message is optional (default describes assertion)

**Examples:**
```
// Basic assertions
assert(x > 0)
assert(type(value) == "int")
assert(list.length > 0, "List cannot be empty")

// In functions
fn sqrt(x) {
    assert(x >= 0, "Cannot take square root of negative number")
    x ** 0.5
}

// Validating invariants
class BankAccount {
    var balance = 0;
    
    fn deposit(amount) {
        assert(amount > 0, "Deposit amount must be positive")
        self.balance += amount
        assert(self.balance >= 0, "Balance invariant violated")
    }
    
    fn withdraw(amount) {
        assert(amount > 0, "Withdrawal amount must be positive")
        assert(amount <= self.balance, "Insufficient funds")
        self.balance -= amount
    }
}

// Complex conditions
fn process_data(data) {
    assert(data, "Data cannot be null")
    assert(type(data) == "list", "Data must be a list")
    assert(data.length > 0, "Data cannot be empty")
    assert(data.all(fn(x) { type(x) == "int" }), "All elements must be integers")
    
    // Process validated data...
}

// Development assertions
fn optimize_path(points) {
    var original_count = points.length
    
    // ... optimization logic ...
    
    assert(points.length <= original_count, "Optimization should not add points")
}
```

### Pattern Match Failures

When pattern matching fails in irrefutable contexts, a runtime error is raised.

**Failure Contexts:**
- Variable declarations with patterns
- Destructuring assignments
- Function parameters with patterns
- For loop bindings

**Error Messages:**
Include helpful information:
- Expected pattern structure
- Actual value structure
- Source location

**Examples:**
```
// Declaration failures
var [x, y] = [1]              // Error: List pattern expected 2 elements, got 1
var {name, age} = {name: "Alice"}  // Error: Dict pattern missing required key 'age'

// Assignment failures
var a, b
[a, b] = [1, 2, 3]           // Error: List pattern expected 2 elements, got 3

// Parameter failures
fn process_pair([x, y]) {
    x + y
}
process_pair([1])            // Error: List pattern expected 2 elements, got 1

// For loop failures
for [x, y] in [[1, 2], [3]] {  // Error on second iteration
    print(x + y)
}

// Safe pattern matching
fn safe_destructure(data) {
    try {
        var [x, y, z] = data
        process_triple(x, y, z)
    } catch e {
        print("Invalid data format: ${e}")
        null
    }
}

// Validating before destructuring
fn extract_point(data) {
    if type(data) == "list" and data.length == 2 {
        var [x, y] = data  // Safe - we checked
        Point.new(x, y)
    } else {
        raise("Expected [x, y] coordinate pair")
    }
}
```

## 10. Modules

RustLeaf provides a module system for organizing code into reusable components. Modules enable namespace management, visibility control, and code separation. This chapter defines module declaration, import/export semantics, path resolution, and dependency management.

### 10.1. Module System Overview

**Design Principles:**
- File-based modules (one module per file)
- Explicit visibility with `pub` keyword
- Path-based import system with `use` statements
- Runtime circular dependency detection
- No module caching (modules imported fresh each time)

**Module Structure:**
```
project/
├── main.rustleaf           // Entry point
├── utils.rustleaf          // Module: utils
├── math/
│   ├── geometry.rustleaf   // Module: math::geometry
│   └── algebra.rustleaf    // Module: math::algebra
└── graphics/
    ├── shapes/
    │   └── circle.rustleaf // Module: graphics::shapes::circle
    └── colors.rustleaf     // Module: graphics::colors
```

### 10.2. Import Statements

Import statements use the `use` keyword to bring items from other modules into scope.

**Basic Import Syntax:**
```
use module_path;                    // Import module
use module_path::item;              // Import specific item
use module_path::{item1, item2};    // Import multiple items
use module_path::*;                 // Import all public items (discouraged)
```

**Path Resolution:**
- **Parent paths**: `use super::sibling_module` (parent directory)
- **Root paths**: `use root::top_level_module` (project root, explicit)
- **Absolute paths**: `use math::geometry` (from project root, default)

**Import Examples:**
```
// File: graphics/renderer.rustleaf

// Import from project root
use math::geometry::Point;
use math::algebra::{Vector, Matrix};

// Import from parent directory
use super::colors::{RED, GREEN, BLUE};

// Import from same directory (just use the name)
use effects::Shader;

// Import from root explicitly
use root::utils::Logger;

// Multiple imports
use math::geometry::{Point, Line, Circle};

// Import all (discouraged)
use math::constants::*;
```

### 10.3. Export Functions

Exports use the `pub` keyword to make items visible outside the current module.

**Visibility Levels:**
- **Private** (default): Only accessible within the current module
- **Public** (`pub`): Accessible from other modules that import this module

**Export Syntax:**
```
// Public function
pub fn public_function() { }

// Private function (default)
fn private_function() { }

// Public variable
pub var PUBLIC_CONSTANT = 42;

// Private variable (default)
var private_data = [];

// Public class
pub class PublicClass {
    pub var public_field;     // Public field
    var private_field;        // Private field
    
    pub fn public_method() { } // Public method
    fn private_method() { }    // Private method
}

// Private class (default)
class PrivateClass { }
```

**Module Example:**
```
// File: math/geometry.rustleaf

// Public exports - available to importers
pub class Point {
    pub var x;
    pub var y;
    
    pub static fn new(x, y) {
        var p = Point()
        p.x = x
        p.y = y
        p
    }
    
    pub fn distance_to(other) {
        var dx = self.x - other.x
        var dy = self.y - other.y
        sqrt(dx * dx + dy * dy)
    }
    
    // Private method - only accessible within this module
    fn validate() {
        if type(self.x) != "float" or type(self.y) != "float" {
            raise("Point coordinates must be numbers")
        }
    }
}

pub fn distance(p1, p2) {
    p1.distance_to(p2)
}

pub var ORIGIN = Point.new(0.0, 0.0);

// Private helper - not accessible from other modules
fn sqrt(x) {
    x ** 0.5
}

var cache = {};  // Private module variable
```

**Usage:**
```
// File: main.rustleaf
use math::geometry::{Point, distance, ORIGIN};

var p1 = Point.new(3.0, 4.0)
var p2 = Point.new(0.0, 0.0)

print(distance(p1, ORIGIN))     // OK - public function
print(p1.distance_to(p2))       // OK - public method
// print(p1.validate())         // Error - private method
// var c = cache                // Error - private variable
```

### 10.4. Module Resolution

Module paths are resolved based on the filesystem structure and special keywords.

**Resolution Rules:**
1. **Absolute paths** start from project root (default)
2. **Parent paths** start with `super::` (parent directory)
3. **Root paths** start with `root::` (project root, explicit)

**Path Resolution Algorithm:**
```
use path::to::module
     ↓
1. If path starts with "super::" → resolve relative to parent directory
2. If path starts with "root::" → resolve from project root
3. Otherwise → resolve from project root (absolute)
```

**Examples:**
```
// Project structure:
// src/
// ├── main.rustleaf
// ├── utils.rustleaf
// ├── graphics/
// │   ├── renderer.rustleaf
// │   └── shapes/
// │       └── circle.rustleaf
// └── math/
//     └── geometry.rustleaf

// File: src/graphics/renderer.rustleaf

use utils                          // → src/utils.rustleaf
use math::geometry                 // → src/math/geometry.rustleaf  
use shapes::circle                 // → src/graphics/shapes/circle.rustleaf
use super::utils                   // → src/utils.rustleaf
use root::math::geometry           // → src/math/geometry.rustleaf
```

**Module File Mapping:**
- `use utils` → `utils.rustleaf`
- `use math::geometry` → `math/geometry.rustleaf`
- `use graphics::shapes::circle` → `graphics/shapes/circle.rustleaf`

### 10.5. Module Loading

Modules are loaded and executed when first imported.

**Loading Behavior:**
- Each module file is executed once when first imported
- Module-level code runs during import (initialization)
- No caching - modules are re-loaded on each import
- Circular dependencies cause runtime errors

**Module Initialization:**
```
// File: database.rustleaf

// Module-level initialization code
print("Loading database module...")

var connection_pool = [];
var default_config = {
    host: "localhost",
    port: 5432,
    timeout: 30
};

// Initialize connection pool
for i in range(0, 10) {
    connection_pool.append(create_connection(default_config))
}

print("Database module loaded with ${connection_pool.length} connections")

// Public API
pub fn get_connection() {
    if connection_pool.length > 0 {
        connection_pool.pop()
    } else {
        create_connection(default_config)
    }
}

pub fn release_connection(conn) {
    connection_pool.append(conn)
}

// Private helper
fn create_connection(config) {
    // Implementation details...
    {host: config.host, port: config.port, active: true}
}
```

**Import Execution:**
```
// File: main.rustleaf
print("Starting application...")

use database  // This triggers database.rustleaf execution
// Output: "Loading database module..."
//         "Database module loaded with 10 connections"

var conn = database.get_connection()
print("Got connection: ${conn}")
```

### 10.6. Module Scope

Each module has its own scope separate from other modules.

**Scope Rules:**
- Module-level variables are private by default
- Only `pub` items are accessible from other modules
- Imported items are available in the importing module's scope
- No global namespace pollution

**Scope Isolation:**
```
// File: module_a.rustleaf
var private_var = "A's private data"
pub var public_var = "A's public data"

pub fn get_private() {
    private_var  // OK - same module
}

// File: module_b.rustleaf  
var private_var = "B's private data"  // Different from A's private_var
pub var public_var = "B's public data"

use module_a

pub fn test() {
    print(module_a.public_var)     // OK - public
    print(module_a.get_private())  // OK - returns A's private data
    // print(module_a.private_var) // Error - not public
    
    print(private_var)             // "B's private data" - local scope
}
```

**Import Scope:**
```
// File: graphics.rustleaf
use math::geometry::Point
use math::algebra::{Vector, Matrix}
use utils::*

// Point, Vector, Matrix, and all utils items are now in scope
pub fn render_scene() {
    var origin = Point.new(0, 0)     // Point from math::geometry
    var transform = Matrix.identity() // Matrix from math::algebra
    log("Rendering...")              // log from utils (via *)
}
```

**Standard Library Scope:**
All standard library functions and types are available globally without explicit import:
```
// These are always available:
print("Hello")           // Built-in function
var x = type(42)         // Built-in function  
var list = [1, 2, 3]     // Built-in type
var dict = {a: 1}        // Built-in type

// No need for:
// use std::io::print
// use std::types::type
```

**Circular Dependency Detection:**
The runtime maintains an import stack to detect circular dependencies during module loading.

```
// File: module_a.rustleaf
use module_b  // ← Circular dependency!

pub fn a_function() {
    module_b.b_function()
}

// File: module_b.rustleaf  
use module_a  // ← Circular dependency!

pub fn b_function() {
    module_a.a_function()
}

// Runtime execution:
// 1. Start loading module_a
// 2. Import stack: [module_a]
// 3. module_a imports module_b
// 4. Import stack: [module_a, module_b]  
// 5. module_b imports module_a
// 6. module_a already in stack → Runtime error!

// Runtime error:
// "Circular dependency detected: module_a → module_b → module_a"
```

**Resolution:**
```
// File: shared.rustleaf
pub fn shared_function() {
    "shared logic"
}

// File: module_a.rustleaf
use shared

pub fn a_function() {
    shared.shared_function() + " from A"
}

// File: module_b.rustleaf
use shared
use module_a  // OK - no circularity

pub fn b_function() {
    module_a.a_function() + " via B"
}
```

## 11. Built-in Functions

RustLeaf provides a comprehensive set of built-in functions available globally without import. These functions cover type introspection, collection operations, string manipulation, error handling, and utility operations. This chapter defines all built-in functions, their signatures, behavior, and usage patterns.

### 11.1. Type Functions

Functions for type introspection and conversion.

**type(value) → string**
Returns the type name of a value as a string.

```
type(42)           // "int"
type(3.14)         // "float"  
type("hello")      // "string"
type(true)         // "bool"
type(null)         // "null"
type([1, 2, 3])    // "list"
type({a: 1})       // "dict"
type(fn() {})      // "function"
```

**str(value) → string**
Converts any value to its string representation.

```
str(42)            // "42"
str(3.14)          // "3.14"
str(true)          // "true"
str(null)          // "null"
str([1, 2, 3])     // "[1, 2, 3]"
str({a: 1, b: 2})  // "{a: 1, b: 2}"

// Uses op_str() method if available
class Point {
    var x, y;
    fn op_str() { "(${self.x}, ${self.y})" }
}
str(Point.new(1, 2))  // "(1, 2)"
```

**int(value) → int**
Converts a value to an integer.

```
int("42")          // 42
int(3.14)          // 3 (truncated)
int(true)          // 1
int(false)         // 0
int(null)          // Error: Cannot convert null to int

// String parsing
int("123")         // 123
int("0xFF")        // 255 (hex)
int("0b1010")      // 10 (binary)
int("0o77")        // 63 (octal)
int("invalid")     // Error: Invalid integer format
```

**float(value) → float**
Converts a value to a floating-point number.

```
float("3.14")      // 3.14
float(42)          // 42.0
float(true)        // 1.0
float(false)       // 0.0
float(null)        // Error: Cannot convert null to float

// Scientific notation
float("1.23e4")    // 12300.0
float("5e-3")      // 0.005
```

**bool(value) → bool**
Converts a value to a boolean using truthiness rules.

```
bool(true)         // true
bool(false)        // false
bool(null)         // false
bool(0)            // Error: Only null and bool have truthiness
bool("")           // Error: Only null and bool have truthiness
bool([])           // Error: Only null and bool have truthiness

// Only null and bool values can be converted
// For other types, use explicit comparisons:
// if list.is_empty() { } instead of if bool(list) { }
```

### 11.2. Collection Functions

Functions for working with collections and iterables.

**range(start, end, step=1) → list**
Generates a list of integers from start (inclusive) to end (exclusive).

```
range(0, 5)        // [0, 1, 2, 3, 4]
range(1, 10, 2)    // [1, 3, 5, 7, 9]
range(10, 0, -1)   // [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
range(5, 5)        // [] (empty range)

// Single argument form (start from 0)
range(3)           // [0, 1, 2]
```

**enumerate(iterable, start=0) → list**
Returns a list of [index, value] pairs.

```
enumerate(["a", "b", "c"])           // [[0, "a"], [1, "b"], [2, "c"]]
enumerate(["x", "y"], 1)             // [[1, "x"], [2, "y"]]

// Usage in loops
for i, item in enumerate(["a", "b"]) {
    print("${i}: ${item}")
}
// Output: "0: a", "1: b"
```

**list(iterable) → list**
Converts an iterable to a list.

```
list("hello")      // ["h", "e", "l", "l", "o"]
list(range(3))     // [0, 1, 2]
list({a: 1, b: 2}.keys())  // ["a", "b"]

// Copy a list
var original = [1, 2, 3]
var copy = list(original)  // [1, 2, 3] (new list)
```

**dict(iterable) → dict**
Converts an iterable of key-value pairs to a dictionary.

```
dict([["a", 1], ["b", 2]])           // {a: 1, b: 2}
dict(enumerate(["x", "y"]))          // {0: "x", 1: "y"}

// Copy a dict
var original = {a: 1, b: 2}
var copy = dict(original.items())    // {a: 1, b: 2} (new dict)
```

### 11.3. String Functions

Functions for string operations and formatting.

**print(...values) → null**
Outputs values to console, separated by spaces.

```
print("Hello")                       // "Hello"
print("Result:", 42)                 // "Result: 42"  
print(1, 2, 3)                       // "1 2 3"
print()                              // "" (empty line)

// Automatically converts values to strings
print([1, 2], {a: 3})               // "[1, 2] {a: 3}"
```

### 11.4. Error Functions

Functions for error handling and debugging.

**raise(error) → never**
Raises an error with the given value.

```
raise("Something went wrong")
raise(404)
raise({type: "ValueError", message: "Invalid input"})

// Custom error objects
class CustomError {
    var message;
    static fn new(msg) {
        var e = CustomError()
        e.message = msg
        e
    }
    fn op_str() { "CustomError: ${self.message}" }
}

raise(CustomError.new("Operation failed"))
```

**assert(condition, message="Assertion failed") → null**
Raises an error if condition is false.

```
assert(x > 0)                        // Error if x <= 0
assert(x > 0, "x must be positive")  // Custom message

// Only works with bool values (strict truthiness)
assert(true)                         // OK
assert(false)                        // Error: Assertion failed
assert(null)                         // Error: Assertion failed
// assert(x)                         // Error: assert requires bool
```

### 11.5. Utility Functions

General utility functions for common operations.

**abs(number) → number**
Returns the absolute value of a number.

```
abs(-5)            // 5
abs(3.14)          // 3.14
abs(0)             // 0
abs(-0.5)          // 0.5
```

**min(...values) → value**
Returns the smallest value from the arguments.

```
min(1, 2, 3)       // 1
min(3.14, 2.71)    // 2.71
min(-5, 0, 10)     // -5

// Works with any comparable types
min("apple", "banana", "cherry")     // "apple"
```

**max(...values) → value**
Returns the largest value from the arguments.

```
max(1, 2, 3)       // 3
max(3.14, 2.71)    // 3.14
max(-5, 0, 10)     // 10

// Works with any comparable types  
max("apple", "banana", "cherry")     // "cherry"
```

**callable(value) → bool**
Tests whether a value can be called as a function.

```
callable(print)              // true
callable(fn() {})            // true
callable("hello")            // false
callable(42)                 // false

// Test before calling
var handler = get_handler()
if callable(handler) {
    handler()
} else {
    print("Handler is not callable")
}
```

### 11.6. Global Availability

All built-in functions are available in the global namespace without import.

**Namespace Rules:**
- Built-ins cannot be shadowed by user definitions in global scope
- Built-ins can be shadowed in local scopes
- Built-ins are always accessible via their original names

**Shadowing Examples:**
```
// Global scope - cannot shadow built-ins
var print = "not a function"  // Error: Cannot redefine built-in 'print'

// Local scope - can shadow
fn test() {
    var print = "local variable"  // OK - local scope
    print  // Returns "local variable"
}

fn demo() {
    var abs = fn(x) { x * x }     // OK - local scope  
    abs(-3)                       // 9 (uses local function)
    
    // Access original built-in explicitly if needed
    // (implementation-specific mechanism)
}
```

**Error Handling:**
All built-in functions provide clear error messages:
```
int("invalid")
// Error: Cannot convert "invalid" to integer
//   at int() (built-in)
//   at main.rustleaf:5:10

type()
// Error: type() requires exactly 1 argument, got 0
//   at type() (built-in)  
//   at main.rustleaf:3:5
```

**Performance Notes:**
- Built-in functions are implemented natively for optimal performance
- Type checking is performed at runtime with helpful error messages
- Built-ins integrate seamlessly with RustLeaf's operator overloading

**Complete Built-in Function List:**

**Type Functions:**
- `type(value)` - Get type name
- `str(value)` - Convert to string  
- `int(value)` - Convert to integer
- `float(value)` - Convert to float
- `bool(value)` - Convert to boolean

**Collection Functions:**
- `range(start, end, step=1)` - Generate integer sequence
- `enumerate(iterable, start=0)` - Create indexed pairs
- `list(iterable)` - Convert to list
- `dict(iterable)` - Convert to dictionary

**String Functions:**
- `print(...values)` - Output to console

**Error Functions:**
- `raise(error)` - Raise an error
- `assert(condition, message="Assertion failed")` - Assert condition

**Utility Functions:**
- `abs(number)` - Absolute value
- `min(...values)` - Minimum value
- `max(...values)` - Maximum value  
- `callable(value)` - Test if callable

## 12. Standard Library

RustLeaf provides a rich standard library of methods for built-in types. These methods are available on all instances of their respective types and provide common operations for strings, lists, dictionaries, and objects. This chapter defines all standard library methods, their signatures, behavior, and usage patterns.

### 12.1. String Methods

String methods provide text manipulation and query operations. All string methods return new strings (strings are immutable).

**len() → int**
Returns the length of the string in characters.

```
"hello".len()          // 5
"".len()               // 0
"🚀".len()             // 1 (Unicode character)
```

**split(delimiter=" ") → list**
Splits the string into a list of substrings.

```
"a,b,c".split(",")         // ["a", "b", "c"]
"hello world".split()      // ["hello", "world"] (default: whitespace)
"a::b::c".split("::")      // ["a", "b", "c"]
"abc".split("")            // ["a", "b", "c"] (split each character)
```

**trim() → string**
Removes whitespace from both ends.

```
"  hello  ".trim()         // "hello"
"\n\tworld\n".trim()       // "world"
"no spaces".trim()         // "no spaces"
```

**upper() → string**
Converts to uppercase.

```
"hello".upper()            // "HELLO"
"Hello World".upper()      // "HELLO WORLD"
```

**lower() → string**
Converts to lowercase.

```
"HELLO".lower()            // "hello"
"Hello World".lower()      // "hello world"
```

**replace(old, new) → string**
Replaces all occurrences of old with new.

```
"hello world".replace("world", "universe")  // "hello universe"
"aaa".replace("a", "b")                     // "bbb"
"test".replace("x", "y")                    // "test" (no change)
```

**contains(substring) → bool**
Tests if string contains the substring.

```
"hello world".contains("world")   // true
"hello world".contains("planet")  // false
"test".contains("")               // true (empty string always found)
```

**starts_with(prefix) → bool**
Tests if string starts with the prefix.

```
"hello world".starts_with("hello")  // true
"hello world".starts_with("world")  // false
"test".starts_with("")               // true
```

**ends_with(suffix) → bool**
Tests if string ends with the suffix.

```
"hello world".ends_with("world")    // true
"hello world".ends_with("hello")    // false
"test".ends_with("")                 // true
```

**to_list() → list**
Converts string to list of characters.

```
"hello".to_list()          // ["h", "e", "l", "l", "o"]
"".to_list()               // []
```

### 12.2. List Methods

List methods provide collection manipulation operations. Methods that modify the list return `self` for chaining, while query methods return the requested values.

**len() → int**
Returns the number of elements in the list.

```
[1, 2, 3].len()            // 3
[].len()                   // 0
```

**append(item) → self**
Adds an item to the end of the list.

```
var list = [1, 2]
list.append(3)             // [1, 2, 3]
list.append(4).append(5)   // [1, 2, 3, 4, 5] (chaining)
```

**extend(other) → self**
Adds all items from another iterable to the end.

```
var list = [1, 2]
list.extend([3, 4])        // [1, 2, 3, 4]
list.extend("ab")          // [1, 2, 3, 4, "a", "b"]
```

**insert(index, item) → self**
Inserts an item at the specified index.

```
var list = [1, 3]
list.insert(1, 2)          // [1, 2, 3]
list.insert(0, 0)          // [0, 1, 2, 3]
list.insert(-1, 2.5)       // [0, 1, 2, 2.5, 3] (negative index)
```

**pop(index=-1) → value**
Removes and returns item at index (default: last item).

```
var list = [1, 2, 3]
list.pop()                 // 3, list is now [1, 2]
list.pop(0)                // 1, list is now [2]
list.pop(5)                // Error: Index out of bounds
```

**remove(item) → self**
Removes the first occurrence of item.

```
var list = [1, 2, 3, 2]
list.remove(2)             // [1, 3, 2] (removes first 2)
list.remove(5)             // Error: Item not found
```

**clear() → self**
Removes all items from the list.

```
var list = [1, 2, 3]
list.clear()               // []
```

**map(function) → list**
Returns new list with function applied to each element.

```
[1, 2, 3].map(fn(x) { x * 2 })        // [2, 4, 6]
["a", "b"].map(fn(s) { s.upper() })   // ["A", "B"]
```

**filter(function) → list**
Returns new list with elements where function returns true.

```
[1, 2, 3, 4].filter(fn(x) { x % 2 == 0 })  // [2, 4]
["", "a", ""].filter(fn(s) { s.len() > 0 }) // ["a"]
```

**reduce(function, initial=null) → value**
Reduces list to single value using function.

```
[1, 2, 3, 4].reduce(fn(acc, x) { acc + x }, 0)     // 10
[1, 2, 3].reduce(fn(acc, x) { acc * x }, 1)        // 6
["a", "b", "c"].reduce(fn(acc, x) { acc + x }, "") // "abc"
```

**slice(start, end=null) → list**
Returns new list with elements from start (inclusive) to end (exclusive).

```
[1, 2, 3, 4, 5].slice(1, 4)    // [2, 3, 4]
[1, 2, 3, 4, 5].slice(2)        // [3, 4, 5] (end defaults to length)
[1, 2, 3, 4, 5].slice(-2)       // [4, 5] (negative start)
```

**contains(item) → bool**
Tests if list contains the item.

```
[1, 2, 3].contains(2)      // true
[1, 2, 3].contains(4)      // false
```

**index(item) → int**
Returns index of first occurrence of item.

```
[1, 2, 3, 2].index(2)      // 1
[1, 2, 3].index(4)         // Error: Item not found
```

**reverse() → self**
Reverses the list in place.

```
var list = [1, 2, 3]
list.reverse()             // [3, 2, 1]
```

**sort(key=null) → self**
Sorts the list in place.

```
var list = [3, 1, 2]
list.sort()                           // [1, 2, 3]

var words = ["banana", "apple", "cherry"]
words.sort()                          // ["apple", "banana", "cherry"]

var items = [{a: 3}, {a: 1}, {a: 2}]
items.sort(fn(x) { x.a })            // [{a: 1}, {a: 2}, {a: 3}]
```

**is_empty() → bool**
Tests if list is empty.

```
[].is_empty()              // true
[1].is_empty()             // false
```

**to_dict() → dict**
Converts list of [key, value] pairs to dictionary.

```
[["a", 1], ["b", 2]].to_dict()       // {a: 1, b: 2}
enumerate(["x", "y"]).to_dict()      // {0: "x", 1: "y"}
```

### 12.3. Dict Methods

Dictionary methods provide key-value operations. Methods that modify the dict return `self` for chaining, while query methods return the requested values.

**len() → int**
Returns the number of key-value pairs.

```
{a: 1, b: 2}.len()         // 2
{}.len()                   // 0
```

**keys() → list**
Returns list of all keys in insertion order.

```
{a: 1, b: 2, c: 3}.keys()  // ["a", "b", "c"]
{}.keys()                  // []
```

**values() → list**
Returns list of all values in insertion order.

```
{a: 1, b: 2, c: 3}.values()  // [1, 2, 3]
{}.values()                   // []
```

**items() → list**
Returns list of [key, value] pairs in insertion order.

```
{a: 1, b: 2}.items()       // [["a", 1], ["b", 2]]
{}.items()                 // []
```

**get(key, default=null) → value**
Returns value for key, or default if key not found.

```
{a: 1, b: 2}.get("a")      // 1
{a: 1, b: 2}.get("c")      // null
{a: 1, b: 2}.get("c", 0)   // 0
```

**pop(key, default=null) → value**
Removes and returns value for key.

```
var dict = {a: 1, b: 2}
dict.pop("a")              // 1, dict is now {b: 2}
dict.pop("c")              // null (key not found)
dict.pop("c", "missing")   // "missing"
```

**clear() → self**
Removes all key-value pairs.

```
var dict = {a: 1, b: 2}
dict.clear()               // {}
```

**merge(other) → self**
Merges another dictionary into this one.

```
var dict = {a: 1, b: 2}
dict.merge({b: 3, c: 4})   // {a: 1, b: 3, c: 4}
```

**contains(key) → bool**
Tests if dictionary contains the key.

```
{a: 1, b: 2}.contains("a")  // true
{a: 1, b: 2}.contains("c")  // false
```

**is_empty() → bool**
Tests if dictionary is empty.

```
{}.is_empty()              // true
{a: 1}.is_empty()          // false
```

**to_list() → list**
Converts dictionary to list of [key, value] pairs.

```
{a: 1, b: 2}.to_list()     // [["a", 1], ["b", 2]]
{}.to_list()               // []
```

### 12.4. Object Methods

All objects (including class instances) have these fundamental methods.

**type() → string**
Returns the type name of the object.

```
var obj = MyClass()
obj.type()                 // "MyClass"
42.type()                  // "int"
"hello".type()             // "string"
```

**op_str() → string**
Returns string representation (called by `str()` function).

```
// Default implementation uses type name
var obj = MyClass()
obj.op_str()               // "MyClass()"

// Custom implementation
class Point {
    var x, y;
    fn op_str() { "(${self.x}, ${self.y})" }
}
Point.new(1, 2).op_str()   // "(1, 2)"
```

**op_eq(other) → bool**
Tests equality (called by `==` operator).

```
// Default implementation uses identity
var a = MyClass()
var b = MyClass()
a.op_eq(b)                 // false (different instances)
a.op_eq(a)                 // true (same instance)

// Custom implementation
class Point {
    var x, y;
    fn op_eq(other) {
        type(other) == "Point" and self.x == other.x and self.y == other.y
    }
}
Point.new(1, 2).op_eq(Point.new(1, 2))  // true
```

**has_method(name) → bool**
Tests if object has a method with the given name.

```
"hello".has_method("upper")     // true
"hello".has_method("missing")   // false
[1, 2, 3].has_method("append")  // true
```

**has_field(name) → bool**
Tests if object has a field with the given name.

```
class Person {
    var name;
    var age = 0;
}
var p = Person()
p.has_field("name")        // true
p.has_field("age")         // true
p.has_field("missing")     // false
```

### 12.5. Method Resolution and Inheritance

**Method Call Resolution:**
1. Look for method on object's class
2. Look for method on built-in type (if applicable)
3. Raise error if method not found

**Method Chaining:**
Methods that modify objects return `self` to enable chaining:
```
var list = []
list.append(1).append(2).extend([3, 4]).reverse()  // [4, 3, 2, 1]

var dict = {}
dict.merge({a: 1}).merge({b: 2}).clear()           // {}
```

**Error Handling:**
Standard library methods provide clear error messages:
```
[].pop()
// Error: Cannot pop from empty list
//   at List.pop() (standard library)
//   at main.rustleaf:3:5

{}.pop("missing")
// Error: Key 'missing' not found in dictionary
//   at Dict.pop() (standard library)
//   at main.rustleaf:5:8
```

## 13. Documentation Comments and Docstrings

RustLeaf provides a comprehensive documentation system using Rust-style documentation comments and docstrings. Documentation can be attached to functions, classes, modules, and variables, and is accessible at runtime for dynamic introspection. This chapter defines the documentation syntax, format conventions, and runtime access mechanisms.

### 13.1. Overview

**Documentation Types:**
- **Documentation comments**: `///` for single-line, `/** */` for multi-line
- **Docstrings**: Located above declarations, not inside function bodies
- **Runtime access**: Documentation available via reflection functions
- **Markdown support**: Rich formatting with Markdown syntax
- **Structured format**: Support for `@param`, `@returns`, `@example` tags

**Documentation Targets:**
- Functions and methods
- Classes and their members
- Modules (file-level)
- Variables and constants
- Type definitions

### 13.2. Documentation Comment Syntax

Documentation comments use Rust-style syntax with special comment markers.

**Single-line Documentation Comments:**
```
/// This is a single-line documentation comment
/// It can span multiple lines by repeating the marker
fn documented_function() {
    // Implementation
}
```

**Multi-line Documentation Comments:**
```
/**
 * This is a multi-line documentation comment
 * It can contain rich formatting and examples
 * 
 * @param x The input value
 * @returns The processed result
 */
fn complex_function(x) {
    // Implementation
}
```

**Mixed Documentation:**
```
/// Brief description of the function
/**
 * Extended description with more details
 * and formatting options.
 */
fn mixed_docs_function() {
    // Implementation
}
```

### 13.3. Docstring Syntax

Docstrings use Rust-style placement above declarations rather than inside function bodies.

**Function Docstrings:**
```
/// Calculates the factorial of a number
/// 
/// @param n The number to calculate factorial for
/// @returns The factorial result
/// @example
/// ```
/// factorial(5)  // Returns 120
/// factorial(0)  // Returns 1
/// ```
fn factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

**Class Docstrings:**
```
/// Represents a point in 2D space
/// 
/// This class provides basic geometric operations
/// for working with 2D coordinates.
/// 
/// @example
/// ```
/// var p = Point.new(3, 4)
/// print(p.distance_to_origin())  // 5.0
/// ```
class Point {
    /// The x-coordinate of the point
    var x;
    
    /// The y-coordinate of the point  
    var y;
    
    /// Creates a new point with given coordinates
    /// 
    /// @param x The x-coordinate
    /// @param y The y-coordinate
    /// @returns A new Point instance
    static fn new(x, y) {
        var p = Point()
        p.x = x
        p.y = y
        p
    }
    
    /// Calculates the distance from this point to the origin
    /// 
    /// @returns The Euclidean distance to (0, 0)
    fn distance_to_origin() {
        (self.x * self.x + self.y * self.y) ** 0.5
    }
}
```

### 13.4. Function Docstrings

Function documentation supports structured tags for parameters, return values, and examples.

**Basic Function Documentation:**
```
/// Adds two numbers together
/// 
/// @param a The first number
/// @param b The second number
/// @returns The sum of a and b
fn add(a, b) {
    a + b
}
```

**Complex Function Documentation:**
```
/// Processes a list of items with a transformation function
/// 
/// This function applies the given transformation to each item
/// in the input list and returns a new list with the results.
/// The original list is not modified.
/// 
/// @param items The list of items to process
/// @param transform_fn The function to apply to each item
/// @returns A new list with transformed items
/// 
/// @example
/// ```
/// var numbers = [1, 2, 3]
/// var doubled = process_items(numbers, fn(x) { x * 2 })
/// print(doubled)  // [2, 4, 6]
/// ```
/// 
/// @example
/// ```
/// var words = ["hello", "world"]
/// var upper = process_items(words, fn(s) { s.upper() })
/// print(upper)  // ["HELLO", "WORLD"]
/// ```
fn process_items(items, transform_fn) {
    items.map(transform_fn)
}
```

**Function with Default Parameters:**
```
/// Creates a range of numbers
/// 
/// @param start The starting number (inclusive)
/// @param end The ending number (exclusive)  
/// @param step The step size (default: 1)
/// @returns A list of numbers in the specified range
/// 
/// @example
/// ```
/// create_range(0, 5)     // [0, 1, 2, 3, 4]
/// create_range(1, 10, 2) // [1, 3, 5, 7, 9]
/// ```
fn create_range(start, end, step = 1) {
    range(start, end, step)
}
```

### 13.5. Module Docstrings

Module-level documentation appears at the top of files to describe the module's purpose and contents.

**Module Documentation:**
```
/// Mathematical utility functions
/// 
/// This module provides common mathematical operations
/// and constants for use throughout the application.
/// 
/// @example
/// ```
/// use math_utils::{PI, circle_area}
/// 
/// var area = circle_area(5.0)
/// print("Area: ${area}")
/// ```

/// The mathematical constant π
pub var PI = 3.14159265359;

/// Calculates the area of a circle
/// 
/// @param radius The radius of the circle
/// @returns The area of the circle
pub fn circle_area(radius) {
    PI * radius * radius
}

/// Calculates the circumference of a circle
/// 
/// @param radius The radius of the circle
/// @returns The circumference of the circle  
pub fn circle_circumference(radius) {
    2 * PI * radius
}
```

### 13.6. Variable and Constant Docstrings

Variables and constants can be documented for API clarity.

**Variable Documentation:**
```
/// The default timeout for network operations (in seconds)
var DEFAULT_TIMEOUT = 30;

/// Configuration options for the application
var CONFIG = {
    /// The server host address
    host: "localhost",
    
    /// The server port number
    port: 8080,
    
    /// Whether to enable debug logging
    debug: false
};

/// List of supported file extensions
pub var SUPPORTED_EXTENSIONS = [".txt", ".md", ".rs"];
```

**Class Member Documentation:**
```
class DatabaseConnection {
    /// The connection string used to connect to the database
    var connection_string;
    
    /// Whether the connection is currently active
    var is_connected = false;
    
    /// The maximum number of retry attempts
    var max_retries = 3;
}
```

### 13.7. Object and Type Docstrings

Classes and their members support hierarchical documentation.

**Class with Comprehensive Documentation:**
```
/// A configuration management system
/// 
/// This class provides a structured way to manage application
/// configuration with support for nested values, defaults,
/// and environment variable overrides.
/// 
/// @example
/// ```
/// var config = ConfigManager.new("app.json")
/// config.set("database.host", "localhost")
/// var host = config.get("database.host", "default")
/// ```
class ConfigManager {
    /// The path to the configuration file
    var config_path;
    
    /// The loaded configuration data
    var data = {};
    
    /// Creates a new configuration manager
    /// 
    /// @param path The path to the configuration file
    /// @returns A new ConfigManager instance
    static fn new(path) {
        var manager = ConfigManager()
        manager.config_path = path
        manager.load()
        manager
    }
    
    /// Loads configuration from the file
    /// 
    /// Reads the configuration file and parses it as JSON.
    /// If the file doesn't exist, starts with empty configuration.
    fn load() {
        // Implementation details...
    }
    
    /// Gets a configuration value
    /// 
    /// Retrieves a value from the configuration using dot notation
    /// for nested keys. Returns the default value if the key is not found.
    /// 
    /// @param key The configuration key (supports dot notation)
    /// @param default_value The value to return if key is not found
    /// @returns The configuration value or default
    /// 
    /// @example
    /// ```
    /// config.get("database.host")           // "localhost"
    /// config.get("missing.key", "default")  // "default"
    /// ```
    fn get(key, default_value = null) {
        // Implementation details...
    }
    
    /// Sets a configuration value
    /// 
    /// @param key The configuration key (supports dot notation)
    /// @param value The value to set
    /// @returns Self for method chaining
    fn set(key, value) {
        // Implementation details...
        self
    }
}
```

### 13.8. Docstring Format Conventions

**Markdown Formatting:**
Documentation supports Markdown for rich formatting.

```
/// # Math Utilities
/// 
/// This module provides **essential** mathematical functions:
/// 
/// - Basic arithmetic operations
/// - Trigonometric functions  
/// - Statistical calculations
/// 
/// ## Usage
/// 
/// ```rustleaf
/// use math::{sin, cos, mean}
/// 
/// var angle = PI / 4
/// var x = cos(angle)
/// var y = sin(angle)
/// 
/// var numbers = [1, 2, 3, 4, 5]
/// var average = mean(numbers)
/// ```
/// 
/// > **Note**: All angle parameters are in radians.
```

**Structured Tags:**
```
/// Brief description of the function
/// 
/// Longer description with multiple paragraphs
/// explaining the behavior and use cases.
/// 
/// @param name description
/// @param name description (optional)
/// @returns description
/// @throws ErrorType description of when this error occurs
/// @example
/// ```
/// code example
/// ```
/// @see related_function
/// @since version 1.2.0
/// @deprecated Use new_function instead
```

### 13.9. Runtime Access to Docstrings

Documentation is accessible at runtime for dynamic introspection and tooling.

**Documentation Access Functions:**
```
/// Gets the documentation for a function
/// 
/// @param function_ref The function to get documentation for
/// @returns The documentation string or null if not documented
fn get_doc(function_ref) {
    // Implementation provided by runtime
}

/// Gets documentation for a class
/// 
/// @param class_ref The class to get documentation for  
/// @returns The class documentation or null
fn get_class_doc(class_ref) {
    // Implementation provided by runtime
}

/// Gets documentation for a specific method
/// 
/// @param class_ref The class containing the method
/// @param method_name The name of the method
/// @returns The method documentation or null
fn get_method_doc(class_ref, method_name) {
    // Implementation provided by runtime
}
```

**Runtime Documentation Examples:**
```
/// Calculates the square of a number
/// 
/// @param x The number to square
/// @returns The square of x
fn square(x) {
    x * x
}

// Access documentation at runtime
var doc = get_doc(square)
print(doc)
// Output: "Calculates the square of a number\n\n@param x The number to square\n@returns The square of x"

// Documentation for classes
var class_doc = get_class_doc(Point)
print(class_doc)
// Output: "Represents a point in 2D space\n\nThis class provides basic geometric operations..."

// Documentation for methods
var method_doc = get_method_doc(Point, "distance_to_origin")
print(method_doc)
// Output: "Calculates the distance from this point to the origin..."
```

### 13.10. Tooling Integration

Documentation integrates with development tools for enhanced developer experience.

**IDE Integration:**
- Hover tooltips show formatted documentation
- Auto-completion includes parameter information
- Quick documentation lookup with keyboard shortcuts
- Inline parameter hints during function calls

**Documentation Generation:**
- Export documentation to HTML/Markdown formats
- Generate API reference documentation
- Extract examples for testing
- Validate documentation completeness

**Linting and Validation:**
- Warn about undocumented public functions
- Validate `@param` tags match function parameters
- Check that `@returns` is present for non-void functions
- Verify example code syntax

**Example Tool Usage:**
```
/// Validates an email address
/// 
/// @param email The email address to validate
/// @returns True if the email is valid, false otherwise
/// 
/// @example
/// ```
/// validate_email("user@example.com")  // true
/// validate_email("invalid-email")     // false
/// ```
fn validate_email(email) {
    // Implementation
}

// IDE shows on hover:
// validate_email(email)
// 
// Validates an email address
// 
// Parameters:
//   email - The email address to validate
// 
// Returns:
//   True if the email is valid, false otherwise
```

**Documentation Coverage:**
```bash
# Generate documentation coverage report
rustleaf doc --coverage

# Output:
# Documentation Coverage Report
# ============================
# 
# Functions:        85% (17/20)
# Classes:          90% (9/10)
# Public methods:   78% (14/18)
# 
# Missing documentation:
# - function calculate_tax (line 45)
# - method User.validate (line 123)
# - class DatabaseError (line 200)
```

## 14. Memory Model

RustLeaf provides automatic memory management with predictable semantics for value copying, reference sharing, and garbage collection. This chapter defines how values are stored, copied, and shared in memory, ensuring memory safety while maintaining performance and predictability.

### 14.1. Value Semantics

RustLeaf uses different semantics for different types to balance performance and intuitive behavior.

**Primitive Value Copying:**
Primitive types are copied by value on assignment and parameter passing.

```
// Primitives are copied
var a = 42
var b = a        // b gets a copy of a's value
a = 100          // a changes, b remains 42
print(b)         // 42

// String copying (strings are immutable)
var str1 = "hello"
var str2 = str1  // str2 gets a copy of the string
str1 = "world"   // str1 points to new string, str2 unchanged
print(str2)      // "hello"

// Boolean and null copying
var flag1 = true
var flag2 = flag1  // Copy
flag1 = false      // flag2 remains true
```

**Primitive Types (Copy Semantics):**
- `int` - Integer values
- `float` - Floating-point values  
- `bool` - Boolean values
- `null` - Null value
- `string` - Immutable strings (copy-on-write)

### 14.2. Reference Semantics

Collections and objects use reference semantics for efficiency and expected behavior.

**Collection Reference Sharing:**
Lists and dictionaries are passed by reference.

```
// Lists are shared by reference
var list1 = [1, 2, 3]
var list2 = list1    // list2 references the same list
list1.append(4)      // Modifies the shared list
print(list2)         // [1, 2, 3, 4] - sees the change

// Dictionaries are shared by reference
var dict1 = {a: 1, b: 2}
var dict2 = dict1    // dict2 references the same dict
dict1.merge({c: 3})  // Modifies the shared dict
print(dict2)         // {a: 1, b: 2, c: 3} - sees the change

// Objects are shared by reference
class Point {
    var x, y;
    static fn new(x, y) {
        var p = Point()
        p.x = x
        p.y = y
        p
    }
}

var p1 = Point.new(1, 2)
var p2 = p1          // p2 references the same object
p1.x = 10            // Modifies the shared object
print(p2.x)          // 10 - sees the change
```

**Reference Types:**
- `list` - Mutable sequences
- `dict` - Mutable key-value mappings
- `function` - Function objects
- `object` - Class instances
- `RustValue` - Custom Rust-implemented types

**Function Parameter Passing:**
```
fn modify_primitive(x) {
    x = 999          // Only modifies local copy
}

fn modify_collection(list) {
    list.append(4)   // Modifies the shared list
}

var num = 42
var arr = [1, 2, 3]

modify_primitive(num)    // num remains 42
modify_collection(arr)   // arr becomes [1, 2, 3, 4]

print(num)  // 42
print(arr)  // [1, 2, 3, 4]
```

### 14.3. Garbage Collection

RustLeaf provides automatic memory management with garbage collection to prevent memory leaks and ensure memory safety.

**Automatic Collection:**
Memory is automatically reclaimed when values are no longer reachable.

```
fn create_data() {
    var large_list = range(0, 10000)  // Allocates memory
    large_list.map(fn(x) { x * x })   // More allocation
    // large_list becomes unreachable when function returns
    // Memory is automatically reclaimed by GC
}

create_data()
// All memory from create_data() is eligible for collection
```

**Circular Reference Handling:**
The garbage collector automatically handles circular references.

```
// Create circular references
var node1 = {value: 1, next: null}
var node2 = {value: 2, next: node1}
node1.next = node2  // Circular reference: node1 -> node2 -> node1

// Later, when nodes go out of scope
node1 = null
node2 = null
// GC will collect both nodes despite the cycle
```

**Collection Triggers:**
- Automatic collection during memory pressure
- Collection when heap size thresholds are exceeded
- Collection during idle periods
- No manual collection interface (fully automatic)

**Memory Safety Guarantees:**
- No use-after-free errors
- No memory leaks from unreachable objects
- No null pointer dereferences
- Automatic cleanup of circular references

### 14.4. Resource Management

Resources beyond memory require explicit management using structured cleanup patterns.

**Automatic Resource Cleanup:**
The `with` statement provides deterministic resource cleanup.

```
// File resources are cleaned up automatically
with file = open("data.txt") {
    var content = file.read()
    process_content(content)
}  // file.close() called automatically, even if error occurs

// Multiple resources cleaned up in reverse order
with db = connect_database(), cache = connect_cache() {
    var data = db.query("SELECT * FROM users")
    cache.store(data)
}  // cache.close() then db.close() called automatically
```

**Resource Cleanup Protocol:**
Objects can implement cleanup behavior:

```
class DatabaseConnection {
    var connection_handle;
    
    fn close() {
        if self.connection_handle != null {
            release_connection(self.connection_handle)
            self.connection_handle = null
        }
    }
}

// Used with 'with' statement
with conn = DatabaseConnection.new("localhost") {
    conn.execute("INSERT INTO logs VALUES ('action')")
}  // conn.close() called automatically
```

**Error Safety:**
Resource cleanup occurs even when errors are raised:

```
with file = open("output.txt") {
    try {
        dangerous_operation(file)
    } catch e {
        print("Error occurred: ${e}")
        raise(e)  // Re-raise the error
    }
}  // file.close() still called despite the error
```

**Manual Resource Management:**
For cases where `with` statements aren't suitable:

```
var connection = create_connection()
try {
    use_connection(connection)
} finally {
    connection.close()  // Manual cleanup
}
```

**Resource Lifetime Rules:**
1. Resources should be acquired as late as possible
2. Resources should be released as early as possible  
3. Use `with` statements for automatic cleanup
4. Implement `close()` methods for custom resources
5. Resources are not garbage collected - they require explicit cleanup

**Best Practices:**
```
// Good: Use with statement
with file = open("data.txt") {
    process_file(file)
}

// Avoid: Manual resource management
var file = open("data.txt")
process_file(file)
file.close()  // Easy to forget, won't happen on errors

// Good: Custom resource with cleanup
class TempDirectory {
    var path;
    
    static fn new(prefix) {
        var temp = TempDirectory()
        temp.path = create_temp_dir(prefix)
        temp
    }
    
    fn close() {
        remove_directory(self.path)
    }
}

with temp = TempDirectory.new("work") {
    create_files_in(temp.path)
}  // Directory automatically removed
```

**Memory vs. Resource Distinction:**
- **Memory**: Managed automatically by garbage collection
- **Resources**: Require explicit management with deterministic cleanup
- **Files, network connections, locks**: Always resources, never just memory
- **Large data structures**: Memory, cleaned up by GC
- **Temporary objects**: Memory, cleaned up by GC

## 15. Execution Model

RustLeaf follows a predictable execution model with well-defined evaluation order, exception handling, and context management. This chapter specifies how programs execute, expressions evaluate, and errors propagate through the system.

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
    .filter(fn(x) { x > 0 })     // First
    .map(fn(x) { x * 2 })        // Second  
    .reduce(fn(a, b) { a + b })  // Third

// Equivalent to:
var temp1 = data.filter(fn(x) { x > 0 })
var temp2 = temp1.map(fn(x) { x * 2 })
var result = temp2.reduce(fn(a, b) { a + b })
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
Functions return the last expression value, or null if no explicit return.

```
fn explicit_return() {
    if condition {
        return "early"  // Explicit return
    }
    "normal"           // Implicit return
}

fn no_return() {
    print("side effect")
    // Implicitly returns null
}

var result1 = explicit_return()  // "early" or "normal"
var result2 = no_return()        // null
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

### 15.4. Exception Handling

Exceptions propagate up the call stack until caught or the program terminates.

**Exception Propagation:**
Uncaught exceptions unwind the stack and terminate the program.

```
fn level3() {
    raise("Error in level3")
}

fn level2() {
    level3()  // Exception propagates through here
}

fn level1() {
    level2()  // Exception propagates through here
}

// Exception will terminate program with stack trace:
// Error: Error in level3
//   at level3() (main.rustleaf:2:5)
//   at level2() (main.rustleaf:6:5)  
//   at level1() (main.rustleaf:10:5)
//   at main.rustleaf:13:1
level1()
```

**Exception Catching:**
Try-catch blocks catch exceptions and prevent propagation.

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
    // Exception is handled, execution continues
}

print("Program continues...")
```

**Exception in Expressions:**
Exceptions can occur in any expression and propagate immediately.

```
// Exception during function call
var result = compute() + risky_function() + finalize()
// If risky_function() raises, compute() result is discarded
// and finalize() never executes

// Exception during property access
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

**Cleanup During Exceptions:**
Resources are cleaned up even when exceptions occur.

```
with file = open("data.txt") {
    try {
        dangerous_processing(file)
    } catch e {
        print("Error during processing: ${e}")
        raise(e)  // Re-raise
    }
}  // file.close() still called despite the exception
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

// Cleanup occurs even if exception is raised:
// 1. inner.close() 
// 2. outer.close()
// 3. Exception propagates
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

## 16. RustValue Integration

RustLeaf provides seamless integration with custom types implemented in Rust through the RustValue system. This enables extending the language with high-performance, native functionality while maintaining type safety and consistent behavior. This chapter defines how RustValues behave from RustLeaf code perspective.

### 16.1. RustValue Trait

RustValues are custom types implemented in Rust that integrate seamlessly with RustLeaf's type system.

**Core Capabilities:**
RustValues provide a minimal, generic interface for attribute access and type identification.

```
// RustValue types behave like regular RustLeaf objects
var point = Point.new(3.0, 4.0)  // Point is a RustValue type

// Field access works transparently
print(point.x)           // 3.0
print(point.y)           // 4.0

// Method calls work transparently  
print(point.distance())  // 5.0

// Type identification
print(type(point))       // "Point"
```

**Attribute Resolution:**
RustValues handle all attribute access through a generic resolution system.

```
// Field access
var value = custom_object.some_field    // Calls get_attr("some_field")
custom_object.some_field = new_value    // Calls set_attr("some_field", new_value)

// Method access
var method = custom_object.some_method  // Calls get_attr("some_method") 
var result = custom_object.some_method(args)  // Method call

// Dynamic access
var field_name = "dynamic_field"
var value = custom_object[field_name]   // Dynamic attribute access
```

**Error Handling:**
RustValue operations integrate with RustLeaf's exception system.

```
var obj = CustomType.new()

try {
    var value = obj.nonexistent_field
} catch e {
    print("Error: ${e}")  // "Error: Unknown attribute: nonexistent_field"
}

try {
    obj.read_only_field = "new value"  
} catch e {
    print("Error: ${e}")  // "Error: Cannot modify read-only field: read_only_field"
}
```

### 16.2. Value Enum

RustValues are integrated into RustLeaf's value system as first-class citizens.

**Type Integration:**
RustValues work seamlessly with all RustLeaf operations.

```
// RustValues in collections
var objects = [Point.new(1, 2), Point.new(3, 4), Point.new(5, 6)]
var distances = objects.map(fn(p) { p.distance() })

// RustValues in dictionaries
var registry = {
    "origin": Point.new(0, 0),
    "unit_x": Point.new(1, 0),
    "unit_y": Point.new(0, 1)
}

// RustValues as function parameters
fn process_point(point) {
    print("Processing point at (${point.x}, ${point.y})")
    point.distance()
}

process_point(Point.new(2, 3))
```

**Memory Semantics:**
RustValues follow reference semantics like other RustLeaf objects.

```
var p1 = Point.new(1, 2)
var p2 = p1          // p2 references the same RustValue

// Mutation affects both references (if the RustValue allows mutation)
p1.move_to(5, 6)     // Hypothetical mutating method
print(p2.x)          // 5 (sees the change if RustValue is mutable)

// Immutable RustValues work like expected
var config = Config.load("app.json")
var backup = config  // Both reference the same immutable config
```

**Garbage Collection:**
RustValues are managed by the garbage collector like other values.

```
fn create_temporary() {
    var temp = LargeDataStructure.new()
    temp.process()
    // temp becomes unreachable when function exits
    // GC will clean up the RustValue
}

create_temporary()
// LargeDataStructure is eligible for collection
```

### 16.3. Field Access

RustValues can expose fields with configurable access patterns.

**Read-Only Fields:**
Some fields may be read-only to maintain invariants.

```
var file = File.open("data.txt")
print(file.path)         // "/path/to/data.txt" (read-only)
print(file.size)         // 1024 (read-only, computed)

// file.path = "new.txt"  // Error: Cannot modify read-only field
```

**Read-Write Fields:**
Other fields may allow modification.

```
var config = Config.new()
config.timeout = 30      // OK - read-write field
config.host = "localhost" // OK - read-write field

print(config.timeout)    // 30
print(config.host)       // "localhost"
```

**Computed Fields:**
Fields can be computed dynamically.

```
var rect = Rectangle.new(10, 20)
print(rect.width)        // 10 (stored field)
print(rect.height)       // 20 (stored field)  
print(rect.area)         // 200 (computed: width * height)
print(rect.perimeter)    // 60 (computed: 2 * (width + height))
```

**Field Validation:**
Field assignments can include validation.

```
var person = Person.new()
person.age = 25          // OK
person.age = -5          // Error: Age cannot be negative
person.email = "invalid" // Error: Invalid email format
```

### 16.4. Method Dispatch

RustValues can provide methods that integrate with RustLeaf's method system.

**Native Methods:**
RustValues can expose Rust-implemented methods.

```
var vector = Vector3.new(1.0, 2.0, 3.0)
var length = vector.magnitude()      // Native Rust method
var normalized = vector.normalize()  // Returns new Vector3
var dot_product = vector.dot(other_vector)
```

**Method Overriding:**
RustValue methods can override built-in methods for custom behavior.

```
// Custom string representation
var point = Point.new(3, 4)
print(str(point))        // "(3, 4)" - custom op_str implementation

// Custom equality
var p1 = Point.new(1, 2)
var p2 = Point.new(1, 2)
print(p1 == p2)          // true - custom op_eq implementation

// Custom arithmetic
var v1 = Vector.new(1, 2)
var v2 = Vector.new(3, 4)
var sum = v1 + v2        // Vector(4, 6) - custom op_add implementation
```

**Method Chaining:**
RustValue methods support chaining where appropriate.

```
var text = TextProcessor.new()
var result = text
    .load_file("input.txt")
    .remove_comments()
    .normalize_whitespace()  
    .save_to("output.txt")
```

**Variadic Methods:**
RustValue methods can accept variable arguments.

```
var logger = Logger.new()
logger.log("info", "User logged in")
logger.log("error", "Database error", error_code: 500, retry: true)
```

### 16.5. Type Coercion

RustValues use explicit conversion rather than automatic coercion.

**Explicit Conversion:**
RustValues convert to primitives through explicit methods or operators.

```
var number_wrapper = NumberWrapper.new(42)

// Explicit conversion required
var as_int = int(number_wrapper)     // 42 (if conversion supported)
var as_str = str(number_wrapper)     // "NumberWrapper(42)"
var as_float = float(number_wrapper) // 42.0 (if conversion supported)

// No automatic coercion
// var result = number_wrapper + 5   // Error: Cannot add NumberWrapper and int
var result = int(number_wrapper) + 5 // 47 (explicit conversion)
```

**Collection Conversion:**
RustValues can convert to standard collections.

```
var rust_list = RustList.new([1, 2, 3, 4, 5])
var rustleaf_list = list(rust_list)  // [1, 2, 3, 4, 5]

var rust_map = RustMap.new()
rust_map.insert("a", 1)
rust_map.insert("b", 2)
var rustleaf_dict = dict(rust_map)   // {a: 1, b: 2}
```

**Truthiness:**
RustValues follow strict truthiness rules like other types.

```
var custom_obj = CustomType.new()

// Only null and bool have truthiness
// if custom_obj { }              // Error: Only null and bool have truthiness
if custom_obj != null { }         // OK - explicit null check
if custom_obj.is_valid() { }      // OK - explicit method call
```

### 16.6. Lifetime Management

RustValues are managed consistently with RustLeaf's memory model.

**Automatic Management:**
RustValues are automatically managed by the garbage collector.

```
fn process_data() {
    var processor = DataProcessor.new()
    processor.load_large_dataset()
    var results = processor.analyze()
    results  // Return results, processor becomes unreachable
}

var analysis = process_data()
// DataProcessor is cleaned up automatically
// Results remain accessible
```

**Resource Cleanup:**
RustValues that manage resources should support the cleanup protocol.

```
// RustValues can implement close() for resource cleanup
with connection = DatabaseConnection.connect("localhost") {
    var data = connection.query("SELECT * FROM users")
    process_data(data)
}  // connection.close() called automatically

// Custom resources
with temp_file = TempFile.create() {
    temp_file.write("temporary data")
    var content = temp_file.read()
}  // temp_file cleanup removes the file
```

**Shared Ownership:**
Multiple RustLeaf variables can reference the same RustValue.

```
var original = SharedResource.new()
var reference1 = original
var reference2 = original

// All variables reference the same RustValue
original.modify_state()
print(reference1.get_state())  // Sees the modification
print(reference2.get_state())  // Sees the modification
```

**Integration Examples:**

**File System Integration:**
```
var file = File.open("config.json")
print(file.path)          // "/path/to/config.json"
print(file.size)          // 1024
print(file.exists)        // true

var content = file.read()
file.write("new content")
file.close()
```

**Network Integration:**
```
var client = HttpClient.new()
client.timeout = 30
client.headers.set("User-Agent", "RustLeaf/1.0")

var response = client.get("https://api.example.com/data")
print(response.status)    // 200
print(response.body)      // Response content
```

**Data Processing Integration:**
```
var parser = JsonParser.new()
var data = parser.parse(json_string)

// RustValue seamlessly works with RustLeaf collections
for item in data.items {
    print("${item.name}: ${item.value}")
}

var filtered = data.items.filter(fn(item) { item.active })
```

**Performance Integration:**
```
var matrix = Matrix.new(1000, 1000)
matrix.fill_random()

// Native performance for compute-heavy operations
var determinant = matrix.determinant()    // Fast native computation
var inverse = matrix.invert()             // Fast native computation

// Seamless integration with RustLeaf
var matrices = [matrix, inverse]
var results = matrices.map(fn(m) { m.trace() })
```

## 17. Macros

RustLeaf provides a powerful macro system for AST transformation and code generation. Macros operate on the parse tree during module loading, enabling compile-time code transformation, boilerplate elimination, and domain-specific language creation. This chapter defines macro syntax, processing, and capabilities.

### 17.1. Macro Syntax

Macros use Rust-style syntax with `#[macro_name]` annotations placed above declarations.

**Basic Macro Syntax:**
```
#[macro_name]
fn target_function() {
    // Original function
}
```

**Parameterized Macros:**
```
#[macro_name(param1, param2)]
fn target_function() {
    // Function with macro parameters
}

#[macro_name(key: "value", flag: true)]
class TargetClass {
    // Class with named parameters
}
```

**Multiple Macros:**
```
#[first_macro]
#[second_macro(config: "production")]
#[third_macro]
fn heavily_decorated_function() {
    // Multiple macros applied in declaration order
}
```

**Macro Targets:**
Macros can be applied to various language constructs:

```
// Functions
#[test]
fn test_addition() {
    assert(2 + 2 == 4)
}

// Classes
#[serializable]
class User {
    var name;
    var email;
}

// Methods
class Database {
    #[cached(ttl: 300)]
    fn expensive_query(sql) {
        // Database query implementation
    }
}

// Variables
#[config("database.host")]
var db_host = "localhost";

// Modules (at file level)
#[api_version("v1")]
// Module-level macro at top of file
```

### 17.2. Macro Application Rules

Macros are applied during module loading, before code execution.

**Processing Phase:**
Macros transform the AST before any code runs.

```
// File: math_utils.rustleaf

#[benchmark]
fn calculate_fibonacci(n) {
    if n <= 1 {
        n
    } else {
        calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)
    }
}

// When this module is imported:
// 1. Parse the file into AST
// 2. Process #[benchmark] macro - transforms the function
// 3. Execute module top-level code with transformed AST
```

**Macro Composition:**
Multiple macros on the same item are processed in declaration order.

```
#[validate_params]
#[cache_result]
#[log_calls]
fn complex_operation(data) {
    // Processed in order:
    // 1. validate_params transforms the function
    // 2. cache_result transforms the result from step 1
    // 3. log_calls transforms the result from step 2
}
```

**Scope and Access:**
Macros have unrestricted access to all language constructs and scope.

```
// Macro can access and modify:
var private_config = {secret: "hidden"}

#[custom_macro]
fn public_function() {
    // Macro can read/modify private_config
    // Macro can access any module-level variables
    // Macro can generate code that uses any identifiers
}
```

### 17.3. Built-in Macros

RustLeaf provides standard macros for common development tasks.

**Testing Macros:**
```
#[test]
fn test_user_creation() {
    var user = User.new("Alice", "alice@example.com")
    assert(user.name == "Alice")
    assert(user.email == "alice@example.com")
}

#[test]
fn test_error_handling() {
    try {
        invalid_operation()
        assert(false, "Should have raised an error")
    } catch e {
        assert(str(e).contains("Invalid"))
    }
}
```

**Deprecation Macros:**
```
#[deprecated("Use new_api() instead")]
fn old_api() {
    // Legacy implementation
    new_api()
}

#[deprecated("This class will be removed in v2.0", since: "1.5.0")]
class LegacyProcessor {
    // Deprecated class
}
```

**Performance Macros:**
```
#[benchmark]
fn performance_critical_function() {
    // Function execution is automatically timed
    // Results logged or collected for analysis
}

#[profile]
fn memory_intensive_operation() {
    // Memory usage tracked during execution
}
```

**Documentation Macros:**
```
#[example]
fn api_usage_example() {
    // This function serves as a living example
    // May be extracted for documentation
    var client = ApiClient.new()
    client.authenticate("token")
    var data = client.fetch_data()
    data.process()
}
```

### 17.4. User-defined Macros

Users can define custom macros that transform the AST.

**Macro Declaration:**
Custom macros are functions marked with `#[macro]`.

```
#[macro]
fn log_execution(ast_node) {
    if ast_node.type != "function" {
        raise("@log_execution can only be applied to functions")
    }
    
    var function_name = ast_node.name
    var original_body = ast_node.body
    
    // Create new function body with logging
    var new_body = parse("""
        {
            print("Entering function: ${function_name}")
            var start_time = current_time()
            
            var result = ${original_body}
            
            var end_time = current_time()
            print("Exiting function: ${function_name}, took: ${end_time - start_time}ms")
            result
        }
    """)
    
    // Return modified AST node
    ast_node.body = new_body
    ast_node
}
```

**Macro Usage:**
```
#[log_execution]
fn calculate_result(data) {
    // Original function implementation
    process_data(data)
}

// After macro processing, equivalent to:
fn calculate_result(data) {
    print("Entering function: calculate_result")
    var start_time = current_time()
    
    var result = {
        process_data(data)  // Original body
    }
    
    var end_time = current_time()
    print("Exiting function: calculate_result, took: ${end_time - start_time}ms")
    result
}
```

**Parameterized Custom Macros:**
```
#[macro]
fn retry(ast_node, max_attempts: 3, delay: 1000) {
    if ast_node.type != "function" {
        raise("@retry can only be applied to functions")
    }
    
    var original_body = ast_node.body
    
    var new_body = parse("""
        {
            var attempts = 0
            while attempts < ${max_attempts} {
                try {
                    ${original_body}
                } catch e {
                    attempts = attempts + 1
                    if attempts >= ${max_attempts} {
                        raise(e)
                    }
                    sleep(${delay})
                }
            }
        }
    """)
    
    ast_node.body = new_body
    ast_node
}

// Usage
#[retry(max_attempts: 5, delay: 2000)]
fn unreliable_network_call() {
    fetch_data_from_api()
}
```

### 17.5. Macro Processing

Macros receive AST nodes and return transformed AST nodes.

**AST Node Structure:**
Macros work with structured representations of code.

```
// Example AST node for a function
{
    type: "function",
    name: "example_function",
    parameters: [
        {name: "param1", default: null},
        {name: "param2", default: "default_value"}
    ],
    body: {
        type: "block",
        statements: [...]
    },
    modifiers: ["pub", "static"],  // If applicable
    location: {file: "example.rustleaf", line: 10, column: 1}
}
```

**Code Generation:**
Macros can generate new code using the `parse()` function.

```
#[macro]
fn property_accessor(ast_node) {
    if ast_node.type != "class" {
        raise("@property_accessor only applies to classes")
    }
    
    var new_methods = []
    
    // Generate getter and setter for each field
    for field in ast_node.fields {
        var getter = parse("""
            fn get_${field.name}() {
                self.${field.name}
            }
        """)
        
        var setter = parse("""
            fn set_${field.name}(value) {
                self.${field.name} = value
                self
            }
        """)
        
        new_methods.append(getter)
        new_methods.append(setter)
    }
    
    // Add generated methods to the class
    ast_node.methods.extend(new_methods)
    ast_node
}

// Usage
#[property_accessor]
class User {
    var name;
    var email;
    var age;
}

// Generates methods: get_name(), set_name(), get_email(), set_email(), etc.
```

### 17.6. AST Transformation

Macros can perform complex AST transformations.

**Code Injection:**
```
#[macro]
fn inject_validation(ast_node) {
    if ast_node.type != "function" {
        raise("@inject_validation only applies to functions")
    }
    
    // Inject validation at the beginning of function
    var validation_code = parse("""
        if args.len() != ${ast_node.parameters.len()} {
            raise("Invalid number of arguments")
        }
    """)
    
    // Prepend validation to function body
    ast_node.body.statements.insert(0, validation_code)
    ast_node
}
```

**Code Wrapping:**
```
#[macro]
fn transaction(ast_node) {
    var original_body = ast_node.body
    
    var wrapped_body = parse("""
        {
            var tx = database.begin_transaction()
            try {
                var result = ${original_body}
                tx.commit()
                result
            } catch e {
                tx.rollback()
                raise(e)
            }
        }
    """)
    
    ast_node.body = wrapped_body
    ast_node
}
```

**Multiple Item Generation:**
```
#[macro]
fn crud_operations(ast_node) {
    if ast_node.type != "class" {
        raise("@crud_operations only applies to classes")
    }
    
    var class_name = ast_node.name
    var module_items = []
    
    // Generate CRUD functions
    var create_fn = parse("""
        fn create_${class_name}(data) {
            var instance = ${class_name}()
            for key, value in data.items() {
                instance[key] = value
            }
            database.save(instance)
            instance
        }
    """)
    
    var read_fn = parse("""
        fn read_${class_name}(id) {
            database.find(${class_name}, id)
        }
    """)
    
    module_items.append(create_fn)
    module_items.append(read_fn)
    
    // Return the original class plus generated functions
    [ast_node] + module_items
}
```

### 17.7. Macro Evaluation Order

Macros are processed in a predictable order during module loading.

**Processing Order:**
1. **File parsing**: Source code parsed into initial AST
2. **Macro resolution**: Macro functions are identified and loaded
3. **Macro application**: Macros applied in declaration order
4. **Final AST**: Transformed AST ready for execution

**Declaration Order Processing:**
```
#[first_macro]
#[second_macro]
#[third_macro]
fn example() {
    // Processing order:
    // 1. first_macro receives original AST
    // 2. second_macro receives result from first_macro
    // 3. third_macro receives result from second_macro
    // 4. Final transformed function is used
}
```

**Module Dependencies:**
Macros from imported modules are available for use.

```
// File: macros/logging.rustleaf
#[macro]
pub fn debug_trace(ast_node) {
    // Macro implementation
}

// File: main.rustleaf
use macros::logging::debug_trace;

#[debug_trace]
fn main_function() {
    // Uses imported macro
}
```

**Error Handling:**
Macro errors halt module loading with detailed error messages.

```
#[invalid_macro_usage]
var not_a_function = 42

// Error: Macro 'invalid_macro_usage' cannot be applied to variable declarations
//   at main.rustleaf:2:1
//   Macro defined at macros/validation.rustleaf:15:1
```

**Macro Scope and Hygiene:**
Macros have unlimited access to generate any code.

```
#[macro]
fn full_access_macro(ast_node) {
    // Can access any variable in scope
    var secret_value = get_module_variable("secret_config")
    
    // Can generate code that uses private functions
    var new_code = parse("""
        {
            var private_data = access_private_function()
            var result = ${ast_node.body}
            log_to_private_channel(result)
            result
        }
    """)
    
    ast_node.body = new_code
    ast_node
}
```

**Complex Transformation Example:**
```
#[macro]
fn state_machine(ast_node) {
    // Transform a class into a state machine
    var states = []
    var transitions = []
    
    // Analyze class to extract state information
    for method in ast_node.methods {
        if method.name.starts_with("state_") {
            states.append(method.name.replace("state_", ""))
        }
        if method.name.starts_with("transition_") {
            transitions.append(parse_transition(method))
        }
    }
    
    // Generate state machine logic
    var state_machine_code = generate_state_machine(states, transitions)
    
    // Inject state machine into class
    ast_node.methods.extend(state_machine_code)
    ast_node.fields.append(parse("var current_state = \"${states[0]}\";"))
    
    ast_node
}

#[state_machine]
class OrderProcessor {
    fn state_pending() { /* ... */ }
    fn state_processing() { /* ... */ }
    fn state_completed() { /* ... */ }
    
    fn transition_start_processing() { /* ... */ }
    fn transition_complete_order() { /* ... */ }
}
// Becomes a full state machine with transition validation
```

---

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

TryStatement ::= "try" Block ("catch" Identifier Block)? ("finally" Block)?

WithStatement ::= "with" WithBinding ("," WithBinding)* Block
WithBinding ::= Identifier "=" Expression

BreakStatement ::= "break" Expression? ";"
ContinueStatement ::= "continue" ";"
ReturnStatement ::= "return" Expression? ";"

Expression ::= AssignmentExpression

AssignmentExpression ::= ConditionalExpression
                      | ConditionalExpression AssignmentOperator AssignmentExpression

AssignmentOperator ::= "=" | "+=" | "-=" | "*=" | "/=" | "%="

ConditionalExpression ::= LogicalOrExpression
                       | LogicalOrExpression "?" Expression ":" ConditionalExpression

LogicalOrExpression ::= LogicalAndExpression
                     | LogicalOrExpression "or" LogicalAndExpression

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
| 13 | `or` | Logical OR | Left-to-right |
| 14 | `?:` | Ternary conditional | Right-to-left |
| 15 (Lowest) | `=` `+=` `-=` `*=` `/=` `%=` | Assignment operators | Right-to-left |

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

// Assignment is lowest precedence
x = y + z   // Equivalent to: x = (y + z)
```

### Associativity Rules

**Left-to-right associativity:**
- Binary operators of same precedence are evaluated left to right
- Example: `a - b + c` is equivalent to `(a - b) + c`

**Right-to-left associativity:**
- Unary operators and assignment operators
- Example: `x = y = z` is equivalent to `x = (y = z)`
- Example: `a ** b ** c` is equivalent to `a ** (b ** c)`

### Parentheses Override

Parentheses can be used to override the default precedence and associativity:

```
(2 + 3) * 4    // = 20 (not 14)
2 ** (3 ** 2)  // = 512 (explicit right-associativity)
x = (y = z)    // Explicit assignment order
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
- `try` - Try block for exception handling
- `catch` - Catch block for exception handling
- `finally` - Finally block for cleanup
- `raise` - Raise an exception

### Resource Management Keywords
- `with` - Resource management statement

### Logical Operators
- `and` - Logical AND operator
- `or` - Logical OR operator
- `not` - Logical NOT operator
- `is` - Identity comparison operator

### Module System Keywords
- `super` - Reference to parent module
- `root` - Reference to project root module

### Type System Keywords
- `self` - Reference to current object instance

### Macro Keywords
- `macro` - Macro definition (used in attributes)

### Reserved for Future Use

The following keywords are reserved for potential future language features:

- `async` - Asynchronous functions
- `await` - Awaiting asynchronous operations
- `const` - Constant declarations
- `enum` - Enumeration types
- `impl` - Implementation blocks
- `interface` - Interface definitions
- `let` - Alternative variable declaration
- `mut` - Mutability modifier
- `private` - Private visibility
- `protected` - Protected visibility
- `struct` - Structure types
- `trait` - Trait definitions
- `type` - Type aliases
- `union` - Union types
- `unsafe` - Unsafe operations
- `where` - Where clauses

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

4. **Operator Words**: The logical operators `and`, `or`, `not`, and `is` are keywords rather than symbols to improve readability.

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

**Returns:** null

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
assert(condition: bool, message?: string) -> null

// I/O functions
print(...values: any) -> null
input(prompt?: string) -> string

// Utility functions
len(collection: list|dict|string) -> int
hash(value: any) -> int
id(value: any) -> int

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

#### Runtime Limits
- **Maximum execution time**: Implementation-defined (may be configurable)
- **Script timeout**: May be imposed by hosting environment
- **Resource cleanup**: Must occur within reasonable time bounds

#### Concurrency
- **Thread safety**: Single-threaded execution model
- **Reentrancy**: Not required (implementation-defined)

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