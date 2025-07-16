# 2. Lexical Structure

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

