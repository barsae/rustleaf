# Program 🔴
```rustleaf
// #[fail_quietly]
var x = ;
```

# Output
```
None
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
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Err(
    "Unexpected token: Semicolon",
)
```

# Eval
```rust
Skipped due to parse error
```