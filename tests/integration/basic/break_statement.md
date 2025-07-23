# Program 🔴
```rustleaf
// #[fail_quietly]
break 42;
```

# Output
None

# Result
```rust
Err(
    "Unexpected break: Int(42)",
)
```

# Lex
```rust
Ok(
    [
        Token(Break),
        Token(Int, "42"),
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
                Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
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
                Some(
                    Literal(
                        Int(
                            42,
                        ),
                    ),
                ),
            ),
        ],
        None,
    ),
)
```