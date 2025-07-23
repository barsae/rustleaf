# Program

```rustleaf
3.14;
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Float,
            text: Some(
                "3.14",
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
                    Float(
                        3.14,
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
                Float(
                    3.14,
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
