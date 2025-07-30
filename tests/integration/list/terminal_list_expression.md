# Program
Status: 🔴
Assertions: 0

```rustleaf
fn f() {
    for i in j {
        i;
    }
    [1, 2, 3]
}
```

# Output
None

# Result
```rust
Skipped due to parse error
```

# Lex
```rust
Ok(
    [
        Token(Fn),
        Token(Ident, "f"),
        Token(LeftParen),
        Token(RightParen),
        Token(LeftBrace),
        Token(For),
        Token(Ident, "i"),
        Token(In),
        Token(Ident, "j"),
        Token(LeftBrace),
        Token(Ident, "i"),
        Token(Semicolon),
        Token(RightBrace),
        Token(LeftBracket),
        Token(Int, "1"),
        Token(Comma),
        Token(Int, "2"),
        Token(Comma),
        Token(Int, "3"),
        Token(RightBracket),
        Token(RightBrace),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Unexpected token: Comma",
)
```

# Eval
```rust
Skipped due to parse error
```