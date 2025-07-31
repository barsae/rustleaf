# Program
Status: 🔴
Assertions: 0

```rustleaf
var x = 0;
var count = 0;
while x < 3 {
    count = count + 1;
    x = x + 1;
}
assert(x == 3);
assert(count == 3);
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
        Token(Ident, "x"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(Var),
        Token(Ident, "count"),
        Token(Equal),
        Token(Int, "0"),
        Token(Semicolon),
        Token(While),
        Token(Ident, "x"),
        Token(Less),
        Token(Int, "3"),
        Token(LeftBrace),
        Token(Ident, "count"),
        Token(Equal),
        Token(Ident, "count"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(Ident, "x"),
        Token(Equal),
        Token(Ident, "x"),
        Token(Plus),
        Token(Int, "1"),
        Token(Semicolon),
        Token(RightBrace),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(EqualEqual),
        Token(Int, "3"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "count"),
        Token(EqualEqual),
        Token(Int, "3"),
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