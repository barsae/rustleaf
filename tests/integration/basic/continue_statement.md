# Program 🔴
```rustleaf
// #[fail_quietly]
continue;
```

# Output
```
None
```

# Result
```rust
Err(
    "Unexpected continue",
)
```

# Lex
```rust
Ok(
    [
        Token(Continue),
        Token(Semicolon),
        Token(Eof),
    ],
)
```

# Parse
```rust
Ok(
    Program(
        [
            Continue,
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            Continue,
        ],
        None,
    ),
)
```