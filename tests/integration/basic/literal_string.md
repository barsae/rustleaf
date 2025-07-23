# Program 🟢
```rustleaf
"hello";
```

# Output
```
None
```

# Result
```rust
Ok(
    String(
        "hello",
    ),
)
```

# Lex
```rust
Ok(
    [
        Token(String, "hello"),
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
            Expression(
                Literal(
                    String(
                        "hello",
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
        [],
        Some(
            Literal(
                String(
                    "hello",
                ),
            ),
        ),
    ),
)
```