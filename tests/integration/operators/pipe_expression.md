# Program
Status: 🔴
Assertions: 0

```rustleaf
fn test(x, y) {
    x + y
}

var z = 1 | test(2);
assert(z == 3);
```

# Output
```
parse_program: starting
parse_program: parsing statement at position 0 (Fn)
parse_statement: starting at position 0 (Fn)
```

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(Ident, "x"),
        Token(Comma),
        Token(Ident, "y"),
        Token(RightParen),
        Token(LeftBrace),
        Token(Ident, "x"),
        Token(Plus),
        Token(Ident, "y"),
        Token(RightBrace),
        Token(Var),
        Token(Ident, "z"),
        Token(Equal),
        Token(Int, "1"),
        Token(Pipe),
        Token(Ident, "test"),
        Token(LeftParen),
        Token(Int, "2"),
        Token(RightParen),
        Token(Semicolon),
        Token(Ident, "assert"),
        Token(LeftParen),
        Token(Ident, "z"),
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
    "Expected Hash, found Fn",
)
```

# Eval
```rust
Skipped due to parse error
```