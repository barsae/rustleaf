# Program

```rustleaf
raise(NetworkError.new(404, "Resource not found"));
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

# Lex

```rust
Ok(
    [
        Token(Ident, "raise"),
        Token(LeftParen),
        Token(Ident, "NetworkError"),
        Token(Dot),
        Token(Ident, "new"),
        Token(LeftParen),
        Token(Int, "404"),
        Token(Comma),
        Token(String, "Resource not found"),
        Token(RightParen),
        Token(RightParen),
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
        [],
        Some(
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
        ),
    ),
)
```
