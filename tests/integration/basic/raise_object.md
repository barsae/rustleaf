# Program

```rustleaf
raise(NetworkError.new(404, "Resource not found"));
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
            token_type: Ident,
            text: Some(
                "NetworkError",
            ),
        },
        Token {
            token_type: Dot,
            text: None,
        },
        Token {
            token_type: Ident,
            text: Some(
                "new",
            ),
        },
        Token {
            token_type: LeftParen,
            text: None,
        },
        Token {
            token_type: Int,
            text: Some(
                "404",
            ),
        },
        Token {
            token_type: Comma,
            text: None,
        },
        Token {
            token_type: String,
            text: Some(
                "Resource not found",
            ),
        },
        Token {
            token_type: RightParen,
            text: None,
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
                        MethodCall(
                            Identifier(
                                "NetworkError",
                            ),
                            "new",
                            [
                                Literal(
                                    Int(
                                        404,
                                    ),
                                ),
                                Literal(
                                    String(
                                        "Resource not found",
                                    ),
                                ),
                            ],
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
                    Call(
                        GetAttr(
                            Variable(
                                "NetworkError",
                            ),
                            "new",
                        ),
                        [
                            Literal(
                                Int(
                                    404,
                                ),
                            ),
                            Literal(
                                String(
                                    "Resource not found",
                                ),
                            ),
                        ],
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
