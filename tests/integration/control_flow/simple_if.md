# Program
Status: 🔴
Assertions: 0

```rustleaf
var result1 = if true { 42 } else { 0 };
var result2 = if false { 100 } else { 200 };
var x = 5;
var result3 = if x > 3 { "big" } else { "small" };
assert(result1 == 42);
assert(result2 == 200);
assert(result3 == "big");
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Var)
parse_statement: starting at position 0 (Var)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Var),
        Token(Ident, "result1"),
        Token(Equal),
        Token(If),
        Token(True),
        Token(LeftBrace),
        Token(Int, "42"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Int, "0"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result2"),
        Token(Equal),
        Token(If),
        Token(False),
        Token(LeftBrace),
        Token(Int, "100"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(Int, "200"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "5"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "result3"),
        Token(Equal),
        Token(If),
        Token(Ident, "x"),
        Token(Greater),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(String, "big"),
        Token(RightBrace),
        Token(Else),
        Token(LeftBrace),
        Token(String, "small"),
        Token(RightBrace),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result1"),
        Token(EqualEqual),
        Token(Int, "42"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result2"),
        Token(EqualEqual),
        Token(Int, "200"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "result3"),
        Token(EqualEqual),
        Token(String, "big"),
        Token(RightParen),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Expected Hash, found Var",
)
```

# Eval
```rust
Skipped due to parse error
```