# Program

```rustleaf
raise(42);
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
            token_type: Int,
            text: Some(
                "42",
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
                            Int(
                                42,
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
                        Int(
                            42,
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
