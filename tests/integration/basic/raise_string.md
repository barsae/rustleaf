# Program

```rustleaf
raise("File not found");
```

# Lex

```rust
Ok(
    [
        Token {
            token_type: Ident,
            text: Some(
                "raise",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "File not found",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
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
                FunctionCall(
                    Identifier(
                        "raise",
                    ),
                    [
                        Literal(
                            String(
                                "File not found",
                            ),
                        ),
                    ],
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
            Call(
                Variable(
                    "raise",
                ),
                [
                    Literal(
                        String(
                            "File not found",
                        ),
                    ),
                ],
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
Err(
    "Undefined variable: raise",
)
```
