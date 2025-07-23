# Program

```rustleaf
"hello";
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: String,
            text: Some(
                "hello",
            ),
        },
        Token {
            token_type: Semicolon,
            text: None,
        },
        Token {
            token_type: Eof,
            text: None,
        },
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
        [
            Literal(
                String(
                    "hello",
                ),
            ),
        ],
        None,
    ),
)
```

# Output

```

```

# Result

```rust
Ok(
    Unit,
)
```
