# Program 🔴
```rustleaf
// #[fail_quietly]
break;
```

# Output
None

# Result
```rust
Err(
    "Unexpected break: Unit",
)
```

# Lex
```rust
Ok(
    [
        Token(Break),
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
            Break(
                None,
            ),
        ],
    ),
)
```

# Eval
```rust
Ok(
    Block(
        [
            Break(
                None,
            ),
        ],
        None,
    ),
)
```